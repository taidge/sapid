"""Retarget only a fresh, disposable MSRV checkout to its sibling Glory source."""
import argparse
import json
from pathlib import Path
import re
import tomllib


def prepare(project_root):
    root = Path(project_root).resolve()
    manifest = root / "Cargo.toml"
    original = manifest.read_bytes().decode("utf-8")
    product = tomllib.loads(original)
    if product["package"]["name"] != "sapid":
        raise ValueError("Expected the disposable Sapid checkout")
    # Cache preparation resolves all three workspaces. Refuse to silently replace
    # an existing developer or product resolution when switching dependency series.
    for relative in ("Cargo.lock", "examples/web/Cargo.lock", "examples/native/Cargo.lock"):
        if (root / relative).exists():
            raise ValueError("Upstream MSRV preparation requires a fresh checkout without lockfiles")
    glory = root.parent / "glory"
    crate = glory / "crates/glory"
    package = tomllib.loads((crate / "Cargo.toml").read_text(encoding="utf-8"))["package"]
    if package["name"] != "glory":
        raise ValueError("Expected the sibling Glory facade")
    version = package["version"]
    if version == {"workspace": True}:
        version = tomllib.loads((glory / "Cargo.toml").read_text(encoding="utf-8"))["workspace"]["package"]["version"]
    if not isinstance(version, str) or not re.fullmatch(r"\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?", version):
        raise ValueError("Invalid checked-out Glory version")
    requirements = {}
    for section in ("dependencies", "dev-dependencies"):
        dependency = product[section]["glory"]
        if dependency.get("git") or dependency.get("registry"):
            raise ValueError("Expected a crates.io or sibling Glory dependency")
        if "path" in dependency and (root / dependency["path"]).resolve() != crate.resolve():
            raise ValueError("Glory must refer to the checked-out sibling facade")
        requirements[section] = dependency["version"]

    count = 0

    def retarget(match):
        nonlocal count
        source = f'version = "={version}"'
        if "path" not in tomllib.loads(match[0])["glory"]:
            source += ', path = "../glory/crates/glory"'
        line, changed = re.subn(r'\bversion\s*=\s*"[^"]+"', source, match[0])
        if changed != 1:
            raise ValueError("Expected an explicit Glory version in each inline dependency")
        count += 1
        return line

    updated = re.sub(r"(?m)^glory\s*=\s*\{[^\r\n]*\}", retarget, original)
    parsed = tomllib.loads(updated)
    if count != 2 or any(parsed[section]["glory"]["version"] != f"={version}" for section in requirements):
        raise ValueError("Unexpected Glory dependency declarations")
    manifest.write_bytes(updated.encode("utf-8"))
    return {"kind": "upstream-msrv-only", "glory_version": version,
            "product_requirements": requirements, "manifest": str(manifest)}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--project-root", type=Path, default=Path(__file__).resolve().parents[1])
    options = parser.parse_args()
    print(json.dumps(prepare(options.project_root)))
