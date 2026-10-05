"""Generate exact approved Cortexa raster assets; requires installed Pillow + iconutil.

No tracing, recoloring, background removal or network access. The fixed square
crop contains the complete brain/glow and stops above the original wordmark.
"""

import argparse
import hashlib
import io
import shutil
import struct
import subprocess
import tempfile
from pathlib import Path

SOURCE_SHA256 = "12aaa6b3e3b02f69d0cc7e6b624eb3eb211b6eaabf2b831a411f8735ac0323c4"
SIDEBAR_SHA256 = "19827d2131b46a3c74eb52efd5e1453d7965743b29edaf7af5098f0d8e96c17c"
BRAIN_CROP = (335, 210, 910, 785)
PNG_SIZES = {
    "32x32.png": 32,
    "128x128.png": 128,
    "128x128@2x.png": 256,
    "icon.png": 512,
    "Square30x30Logo.png": 30,
    "Square44x44Logo.png": 44,
    "Square71x71Logo.png": 71,
    "Square89x89Logo.png": 89,
    "Square107x107Logo.png": 107,
    "Square142x142Logo.png": 142,
    "Square150x150Logo.png": 150,
    "Square284x284Logo.png": 284,
    "Square310x310Logo.png": 310,
    "StoreLogo.png": 50,
}


def icns_chunks(data):
    if len(data) < 8 or data[:4] != b"icns" or struct.unpack(">I", data[4:8])[0] != len(data):
        raise ValueError("Invalid ICNS container")
    chunks = {}
    position = 8
    while position < len(data):
        if position + 8 > len(data):
            raise ValueError("Truncated ICNS header")
        kind = data[position:position + 4]
        length = struct.unpack(">I", data[position + 4:position + 8])[0]
        if length < 8 or position + length > len(data) or kind in chunks:
            raise ValueError("Invalid or duplicate ICNS representation")
        chunks[kind] = data[position:position + length]
        position += length
    if not chunks:
        raise ValueError("Empty ICNS")
    return chunks


def canonical_icns(data):
    chunks = icns_chunks(data)
    body = b"".join(chunks[kind] for kind in sorted(chunks))
    return b"icns" + struct.pack(">I", len(body) + 8) + body


def generate(source, destination):
    from PIL import Image

    if hashlib.sha256(source.read_bytes()).hexdigest() != SOURCE_SHA256:
        raise ValueError("Approved source identity mismatch")
    with Image.open(source) as artwork:
        if artwork.size != (1254, 1254) or artwork.mode != "RGB":
            raise ValueError("Approved artwork geometry/mode mismatch")
        brain = artwork.crop(BRAIN_CROP)
        branding = destination / "assets/branding"
        icons = destination / "src-tauri/icons"
        branding.mkdir(parents=True, exist_ok=True)
        icons.mkdir(parents=True, exist_ok=True)
        for name in ("logo-primary.png", "logo-light.png", "logo-dark.png"):
            shutil.copyfile(source, branding / name)
        brain.resize((512, 512), Image.Resampling.LANCZOS).convert("RGBA").save(branding / "app-icon-source.png")
        brain.resize((64, 64), Image.Resampling.LANCZOS).convert("RGBA").save(branding / "favicon.png")
        for name, size in PNG_SIZES.items():
            brain.resize((size, size), Image.Resampling.LANCZOS).convert("RGBA").save(icons / name)
        brain.resize((256, 256), Image.Resampling.LANCZOS).convert("RGBA").save(
            icons / "icon.ico", sizes=[(size, size) for size in (16, 24, 32, 48, 64, 128, 256)]
        )
        # Standard ICNS: lossless PNG for modern/retina sizes, uncompressed
        # RGB + opaque masks for legacy 16/32 sizes. No OS lossy re-encoder.
        chunks = []
        for kind, size in ((b"ic07", 128), (b"ic08", 256), (b"ic09", 512),
                           (b"ic10", 1024), (b"ic11", 32), (b"ic12", 64),
                           (b"ic13", 256), (b"ic14", 512)):
            output = io.BytesIO()
            brain.resize((size, size), Image.Resampling.LANCZOS).save(output, format="PNG")
            payload = output.getvalue()
            chunks.append(kind + struct.pack(">I", len(payload) + 8) + payload)
        for kind, mask, size in ((b"is32", b"s8mk", 16), (b"il32", b"l8mk", 32)):
            payload = brain.resize((size, size), Image.Resampling.LANCZOS).tobytes()
            chunks.append(kind + struct.pack(">I", len(payload) + 8) + payload)
            chunks.append(mask + struct.pack(">I", size * size + 8) + bytes([255]) * (size * size))
        body = b"".join(chunks)
        original = b"icns" + struct.pack(">I", len(body) + 8) + body
        canonical = canonical_icns(original)
        if icns_chunks(original) != icns_chunks(canonical):
            raise ValueError("ICNS representation changed during ordering")
        (icons / "icon.icns").write_bytes(canonical)


def verify(root, generated, source):
    from PIL import Image

    sidebar = root / "assets/branding/sidebar-symbol.png"
    if hashlib.sha256(sidebar.read_bytes()).hexdigest() != SIDEBAR_SHA256:
        raise ValueError("Retained sidebar cutout identity mismatch")
    with Image.open(sidebar) as symbol:
        if symbol.mode != "RGBA" or symbol.size != (1254, 1254):
            raise ValueError("Sidebar cutout geometry/mode mismatch")
        alpha = symbol.getchannel("A")
        if alpha.getextrema() != (0, 255):
            raise ValueError("Sidebar must contain genuine transparency and opaque artwork")
        for point in ((0, 0), (1253, 0), (0, 1253), (1253, 1253), (627, 627)):
            if alpha.getpixel(point) != 0:
                raise ValueError("Sidebar background/open center is not transparent")
    for expected in sorted(generated.rglob("*")):
        if expected.is_file() and expected.read_bytes() != (root / expected.relative_to(generated)).read_bytes():
            raise ValueError(f"Generated asset mismatch: {expected.relative_to(generated)}")
    with Image.open(source) as artwork:
        brain = artwork.crop(BRAIN_CROP)
        for name, size in PNG_SIZES.items():
            with Image.open(root / "src-tauri/icons" / name) as actual:
                expected = brain.resize((size, size), Image.Resampling.LANCZOS)
                if (actual.mode != "RGBA" or actual.getchannel("A").getextrema() != (255, 255)
                        or actual.size != expected.size or actual.convert("RGB").tobytes() != expected.tobytes()):
                    raise ValueError(f"Decoded PNG fidelity mismatch: {name}")
        with Image.open(root / "src-tauri/icons/icon.ico") as ico:
            if ico.ico.sizes() != {(s, s) for s in (16, 24, 32, 48, 64, 128, 256)}:
                raise ValueError("Incomplete ICO family")
            with Image.open(generated / "src-tauri/icons/icon.ico") as reference:
                for size in ico.ico.sizes():
                    if ico.ico.getimage(size).tobytes() != reference.ico.getimage(size).tobytes():
                        raise ValueError("Decoded ICO mismatch")
        # Decode every stored ICNS representation independently and require
        # exact pixels, not tolerances. Apple iconutil exports the legacy last
        # blue sample as zero on this host; do not reinterpret that tool output
        # as asset fidelity. Still require its structural extraction to succeed.
        with Image.open(root / "src-tauri/icons/icon.icns") as icns:
            sizes = icns.icns.itersizes()
            if len(sizes) != 10:
                raise ValueError("Incomplete decoded ICNS family")
            for size in sizes:
                actual = icns.icns.getimage(size)
                expected = brain.resize(actual.size, Image.Resampling.LANCZOS)
                if actual.convert("RGB").tobytes() != expected.tobytes():
                    raise ValueError(f"Stored ICNS fidelity mismatch: {size}")
                if actual.mode == "RGBA" and actual.getchannel("A").getextrema() != (255, 255):
                    raise ValueError("ICNS unexpectedly introduced transparency")
        with tempfile.TemporaryDirectory(prefix="cortexa-logo-decode-") as temporary:
            decoded = Path(temporary) / "Decoded.iconset"
            subprocess.run(["/usr/bin/iconutil", "-c", "iconset", str(root / "src-tauri/icons/icon.icns"), "-o", str(decoded)], check=True)
            if len(list(decoded.glob("*.png"))) != 10:
                raise ValueError("Incomplete Apple ICNS extraction")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Read-only regeneration and decoded fidelity check")
    parser.add_argument("--output", type=Path, help="Generate into a separate directory")
    args = parser.parse_args()
    if args.check and args.output:
        parser.error("--check and --output are mutually exclusive")
    root = Path(__file__).resolve().parent.parent
    source = root / "assets/branding/approved-september23.png"
    if args.check:
        with tempfile.TemporaryDirectory(prefix="cortexa-logo-check-") as temporary:
            generated = Path(temporary)
            generate(source, generated)
            verify(root, generated, source)
        print("Approved source, repeatable assets, intact ICNS chunks and decoded PNG/ICO/ICNS fidelity passed")
    else:
        generate(source, args.output or root)


if __name__ == "__main__":
    main()
