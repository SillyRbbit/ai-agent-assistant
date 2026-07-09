# Increment 1 verification results

Verification date: June 18, 2026.

## Automated checks

The following command completed successfully:

```bash
npm run check
```

It verified:

- Prettier formatting
- `cargo fmt`
- ESLint with zero warnings
- Clippy with warnings denied
- Strict TypeScript project builds
- Two React unit tests
- One Rust library unit test
- One Rust integration smoke test
- Vite production build

The dependency audit also completed successfully:

```bash
npm audit --audit-level=low
```

Result: zero known npm vulnerabilities at the installed lockfile versions.

## Native build

The following command completed successfully on the Linux verification host:

```bash
npm run tauri -- build --no-bundle
```

It produced an optimized native executable. A ten-second headless runtime smoke test kept that executable alive until the intentional timeout.

The verification host cannot produce or launch a macOS binary. On macOS, run the same source with:

```bash
npm ci
npm run tauri -- dev
```

A macOS bundle build is intentionally not claimed until the repository is executed on a macOS runner or workstation.

## Node.js 26/npm 11 compatibility update

The repository metadata was updated for Node.js 26.3.0 and npm 11.16.0 while retaining strict engine enforcement.

Verification performed after the update:

- A clean `npm ci` completed successfully and left `package.json` unchanged.
- All lockfile download URLs resolve through the public npm registry; no build-environment-specific internal registry URL remains.
- All 213 locked packages that declare a Node.js engine range were checked; none reject Node.js 26.3.0.
- ESLint passed with zero warnings.
- Strict TypeScript checking passed.
- Both React unit tests passed.
- The Vite production build passed.
- `npm audit --audit-level=low` reported zero known vulnerabilities.

The verification host has Node.js 22 and does not have the Rust toolchain installed, so the Rust and native Tauri checks were not rerun for this metadata-only update. No Rust, Tauri, React, or application source file changed. The original native build verification remains applicable; the requested Node.js 26.3.0/npm 11.16.0 runtime must still be exercised on the target macOS workstation.
