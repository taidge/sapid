"""Materialize sibling engine sources for development or registry release validation."""
import argparse
import json
import re
import subprocess
import tempfile
from pathlib import Path


# Release acceptance also audits registry consumers against these published sources.
PUBLISHED_TAGS = {
    "apace": "apace-v0.4.0",
    "takia": "takia-v0.16.0",
    "wingy": "wingy-v0.31.2",
}


def revisions(product, source):
    if source == "published":
        return PUBLISHED_TAGS
    manifest = json.loads((product / "deploy/dependencies.json").read_text(encoding="utf-8"))
    pins = manifest["engines"]
    if set(pins) != set(PUBLISHED_TAGS) or not all(
        isinstance(pin, str) and re.fullmatch("[0-9a-f]{40}", pin) for pin in pins.values()
    ):
        raise ValueError("Expected a full commit SHA for each coordinated engine")
    return pins


def git(directory, *args):
    return subprocess.check_output(["git", "-C", str(directory), *args], text=True).strip()


def checkout(product, name, revision):
    directory = product.parent / name
    if not directory.exists():
        with tempfile.TemporaryDirectory(dir=product.parent) as temp:
            staging = Path(temp) / name
            staging.mkdir()
            git(staging, "init", "--quiet")
            git(staging, "remote", "add", "origin", f"https://github.com/glory-rs/{name}.git")
            refs = ["tag", revision] if revision == PUBLISHED_TAGS[name] else [revision]
            git(staging, "fetch", "--quiet", "--depth=1", "origin", *refs)
            git(staging, "checkout", "--quiet", "--detach", "FETCH_HEAD")
            staging.rename(directory)
    if not (directory / ".git").exists():
        raise ValueError(f"Expected a Git checkout at {directory}")
    head = git(directory, "rev-parse", "HEAD")
    expected = git(directory, "rev-parse", "--verify", f"{revision}^{{commit}}")
    if head != expected:
        raise ValueError(f"{name}: expected {expected}, found {head} in {directory}")
    if git(directory, "status", "--porcelain"):
        raise ValueError(f"{name}: working tree has changes in {directory}")
    print(f"{name}: {head}", flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", choices=("development", "published"), default="development")
    args = parser.parse_args()
    root = Path.cwd()
    for engine, revision in revisions(root, args.source).items():
        checkout(root, engine, revision)
