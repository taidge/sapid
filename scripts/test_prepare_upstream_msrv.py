"""Check version drift with real Cargo and refuse edits to existing resolutions."""
import json
from pathlib import Path
import subprocess
import tempfile
import tomllib
import unittest

from local_build_paths import local_build_path
from prepare_upstream_msrv import prepare


class UpstreamMsrvTests(unittest.TestCase):
    def setUp(self):
        parent = Path(local_build_path("upstream-msrv-tests"))
        parent.mkdir(parents=True, exist_ok=True)
        self.temp = tempfile.TemporaryDirectory(dir=parent)
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.product = self.root / "sapid"
        self.glory = self.root / "glory"
        self.write("glory/Cargo.toml", '[workspace]\nmembers=["crates/glory"]\nresolver="3"\n'
                   '[workspace.package]\nversion="0.8.0"\n')
        self.write("glory/crates/glory/Cargo.toml", '[package]\nname="glory"\nversion.workspace=true\nedition="2024"\n')
        self.write("glory/crates/glory/src/lib.rs", "pub fn upstream() {}\n")
        self.manifest = self.product / "Cargo.toml"
        self.write("sapid/Cargo.toml", '[package]\nname="sapid"\nversion="0.7.1"\nedition="2024"\n'
                   'rust-version="1.95"\n[workspace]\nresolver="3"\n[features]\nextra=[]\n'
                   '[dependencies]\nglory = { version = "=0.7.0", path = "../glory/crates/glory", default-features = false }\n'
                   '[dev-dependencies]\nglory = { version = "=0.7.0", path = "../glory/crates/glory" }\n')
        self.write("sapid/src/lib.rs", "pub fn consumer() { glory::upstream(); }\n")

    def write(self, name, content):
        path = self.root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")

    def cargo(self, *arguments):
        return subprocess.run(["cargo", "+1.95.0", *arguments], cwd=self.product,
                              text=True, capture_output=True, timeout=120)

    def test_fresh_cargo_fails_on_drift_then_checks_both_feature_sets_against_upstream(self):
        before = self.cargo("check", "--offline", "--all-targets")
        self.assertNotEqual(before.returncode, 0)
        self.assertIn('glory = "=0.7.0"', before.stderr)
        report = prepare(self.product)
        self.assertEqual(report["glory_version"], "0.8.0")
        self.assertEqual(set(report["product_requirements"].values()), {"=0.7.0"})
        for features in ([], ["--all-features"]):
            checked = self.cargo("check", "--offline", "--all-targets", *features)
            self.assertEqual(checked.returncode, 0, checked.stdout + checked.stderr)
        metadata = self.cargo("metadata", "--locked", "--offline", "--format-version", "1")
        self.assertEqual(metadata.returncode, 0, metadata.stderr)
        facade = next(package for package in json.loads(metadata.stdout)["packages"] if package["name"] == "glory")
        self.assertEqual(facade["version"], "0.8.0")
        self.assertEqual(Path(facade["manifest_path"]).resolve(), (self.glory / "crates/glory/Cargo.toml").resolve())

    def test_existing_product_and_example_locks_are_preserved_without_manifest_changes(self):
        original = self.manifest.read_bytes()
        for relative in ("Cargo.lock", "examples/web/Cargo.lock", "examples/native/Cargo.lock"):
            lock = self.product / relative
            lock.parent.mkdir(parents=True, exist_ok=True)
            lock.write_bytes(b"existing resolution\n")
            with self.assertRaisesRegex(ValueError, "fresh checkout"):
                prepare(self.product)
            self.assertEqual(self.manifest.read_bytes(), original)
            self.assertEqual(lock.read_bytes(), b"existing resolution\n")
            lock.unlink()

    def test_foreign_facade_path_is_rejected_before_any_edit(self):
        original = self.manifest.read_text(encoding="utf-8").replace(
            'path = "../glory/crates/glory"', 'path = "../foreign/crates/glory"', 1)
        self.manifest.write_text(original, encoding="utf-8")
        with self.assertRaisesRegex(ValueError, "sibling facade"):
            prepare(self.product)
        self.assertEqual(self.manifest.read_text(encoding="utf-8"), original)

    def test_direct_prerelease_version_and_line_endings_are_retained(self):
        self.write("glory/crates/glory/Cargo.toml", '[package]\nname="glory"\nversion="0.9.0-rc.1"\nedition="2024"\n')
        original = self.manifest.read_bytes().replace(b"\r\n", b"\n").replace(b"\n", b"\r\n")
        self.manifest.write_bytes(original)
        prepare(self.product)
        updated = self.manifest.read_bytes()
        self.assertEqual(updated, original.replace(b'=0.7.0', b'=0.9.0-rc.1'))
        parsed = tomllib.loads(updated.decode("utf-8"))
        self.assertEqual(parsed["package"]["version"], "0.7.1")


if __name__ == "__main__":
    unittest.main()
