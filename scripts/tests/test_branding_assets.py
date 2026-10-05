"""Focused deterministic container regressions; no image dependency needed."""
import importlib.util
import hashlib
import json
import struct
import unittest
from pathlib import Path

SPEC = importlib.util.spec_from_file_location("branding_assets", Path(__file__).parents[1] / "branding_assets.py")
assert SPEC is not None and SPEC.loader is not None
BRANDING = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(BRANDING)


def container(chunks):
    body = b"".join(kind + struct.pack(">I", len(payload) + 8) + payload for kind, payload in chunks)
    return b"icns" + struct.pack(">I", len(body) + 8) + body


class BrandingAssetsTests(unittest.TestCase):
    def test_order_is_deterministic_without_changing_representations(self):
        chunks = [(b"ic09", b"large representation"), (b"icp4", b"small representation"), (b"ic08", b"medium representation")]
        first = container(chunks)
        second = container(list(reversed(chunks)))
        canonical = BRANDING.canonical_icns(first)
        self.assertEqual(canonical, BRANDING.canonical_icns(second))
        self.assertEqual(BRANDING.icns_chunks(first), BRANDING.icns_chunks(canonical))
        self.assertEqual(canonical, BRANDING.canonical_icns(canonical))

    def test_invalid_containers_are_rejected(self):
        valid = container([(b"ic09", b"pixels")])
        for candidate in (b"", valid[:-1], b"bad!" + valid[4:], container([]), container([(b"ic09", b"one"), (b"ic09", b"two")]), b"icns" + struct.pack(">I", 16) + b"ic09" + struct.pack(">I", 7)):
            with self.subTest(candidate=candidate):
                with self.assertRaises(ValueError):
                    BRANDING.canonical_icns(candidate)

    def test_crop_and_complete_production_family_are_fixed(self):
        self.assertEqual(BRANDING.BRAIN_CROP, (335, 210, 910, 785))
        self.assertEqual(len(BRANDING.PNG_SIZES), 14)
        self.assertEqual(BRANDING.PNG_SIZES["128x128@2x.png"], 256)
        self.assertEqual(BRANDING.PNG_SIZES["icon.png"], 512)

    def test_approved_full_aliases_and_native_rgba_contract(self):
        root = Path(__file__).parents[2]
        source = (root / "assets/branding/approved-september23.png").read_bytes()
        self.assertEqual(hashlib.sha256(source).hexdigest(), BRANDING.SOURCE_SHA256)
        for name in ("logo-primary.png", "logo-light.png", "logo-dark.png"):
            self.assertEqual((root / "assets/branding" / name).read_bytes(), source)
        for name, size in BRANDING.PNG_SIZES.items():
            data = (root / "src-tauri/icons" / name).read_bytes()
            self.assertEqual(data[:8], b"\x89PNG\r\n\x1a\n")
            self.assertEqual(struct.unpack(">II", data[16:24]), (size, size))
            self.assertEqual(data[24:26], bytes([8, 6]))  # 8-bit RGBA for Tauri

    def test_existing_packaging_identifiers_and_icon_references_remain(self):
        root = Path(__file__).parents[2]
        config = json.loads((root / "src-tauri/tauri.conf.json").read_text())
        self.assertEqual(config["identifier"], "com.aiagentassistant.desktop")
        self.assertEqual(config["productName"], "Cortexa")
        self.assertEqual(config["bundle"]["targets"], ["app", "dmg"])
        for name in config["bundle"]["icon"]:
            self.assertTrue((root / "src-tauri" / name).is_file())
        chunks = BRANDING.icns_chunks((root / "src-tauri/icons/icon.icns").read_bytes())
        self.assertEqual(set(chunks), {b"is32", b"s8mk", b"il32", b"l8mk", b"ic07", b"ic08", b"ic09", b"ic10", b"ic11", b"ic12", b"ic13", b"ic14"})

    def test_sidebar_uses_separate_transparent_cutout_without_wordmark(self):
        root = Path(__file__).parents[2]
        data = (root / "assets/branding/sidebar-symbol.png").read_bytes()
        self.assertEqual(hashlib.sha256(data).hexdigest(), BRANDING.SIDEBAR_SHA256)
        self.assertEqual(struct.unpack(">II", data[16:24]), (1254, 1254))
        self.assertEqual(data[24:26], bytes([8, 6]))
        sidebar = (root / "src/components/ApplicationSidebar.tsx").read_text()
        self.assertEqual(sidebar.count("src={sidebarSymbol}"), 2)
        self.assertNotIn("brandLogo", sidebar)
        self.assertNotIn("brandFavicon", sidebar)
