# Approved Cortexa raster asset generation

Use only `assets/branding/approved-september23.png`, copied byte-identically from
the owner-supplied September 23 source. Do not use earlier redrawn SVGs.

With an already installed Python/Pillow and macOS `iconutil`:

```bash
python3 scripts/branding_assets.py
python3 scripts/branding_assets.py --check
python3 -m unittest discover -s scripts/tests -p test_branding_assets.py -v
```

Generation uses a fixed square crop and proportional Lanczos resampling. The
source and full aliases remain byte-identical; no colors or alpha are changed.
The complete 14 PNG / seven ICO representation / ten ICNS representation families
are generated offline. ICNS ordering is canonicalized by sorting intact chunk
identifiers, preserving every chunk payload and the container length. Read-only
check regenerates into temporary storage, compares bytes, then independently decodes PNG, ICO
and every stored ICNS representation against the approved crop. Apple `iconutil`
also must extract all ten sizes. On this host its legacy 16/32 export zeroes the
last blue sample (one pixel); this tool discrepancy is recorded, never treated
as a matching pixel export. Stored assets must still match exactly in the
independent decoder. Legacy sizes use standard uncompressed RGB/opaque masks;
modern/retina representations use lossless PNG. No pixel tolerance is allowed. No Windows execution is
implied. App production identifiers/configuration and existing packaging icon
references remain unchanged.

Native unsigned QA uses a separate task-owned bundle identifier and data location.
No signing, notarization, installation, live networking or provider QA is included.
Preserve previous bundles. The documented process-local Python/Xcode SDK27/Cargo
stripping workaround changes no repository/toolchain configuration.

## Sidebar-only transparent symbol

The owner additionally approved transparent background extraction for the sidebar
only. `sidebar-symbol.png` is retained unchanged from the built-in image tool,
using the approved full raster followed by an edge-cleanup edit. The approved PNG
and all full/native assets are preserved separately. This cutout is not asserted
pixel-identical to the opaque input: its alpha/edge pixels necessarily differ;
contours, blue/cyan detail and proportional appearance require visual comparison.
Both expanded (72px) and collapsed (28px) sidebar use this RGBA asset. It has no
wordmark; expanded mode retains the existing Cortexa/Private workspace copy.

Read-only asset validation checks its frozen SHA-256, RGBA geometry, opaque artwork,
fully transparent background corners and open center. Regeneration of PNG/ICO/ICNS
does not replace this retained cutout. Its extraction prompt and source/output
provenance are preserved externally with the milestone evidence. No approximate
SVG or CSS background-removal workaround is used.
