# Increment 1.1 — Node.js 26 and npm 11 compatibility

## Goal

Permit the existing Increment 1 application to install and run with Node.js 26.3.0 and npm 11.16.0 without disabling strict engine enforcement.

## Files changed

- `package.json`
- `package-lock.json`
- `README.md`
- `.nvmrc`
- `.node-version`

## Compatibility policy

The project accepts:

- Node.js 22.12 or later in the Node.js 22 release line
- Node.js 24
- Node.js 26
- npm 10 or npm 11

Node.js 23 and 25 are excluded because those release lines are end-of-life. The preferred local toolchain is Node.js 26.3.0 with npm 11.16.0.

`engine-strict=true` remains enabled. An unsupported runtime still fails early instead of producing an ambiguous build error later. The npm install-script allowlist permits only the locked `esbuild` and macOS `fsevents` lifecycle scripts required by the frontend toolchain. The lockfile uses public npm registry URLs rather than build-environment-specific internal registry URLs.

## Verification commands

```bash
node --version
npm --version
rm -rf node_modules
npm ci
npm run check
npm run tauri -- build --no-bundle
```

Expected version output for the requested environment:

```text
v26.3.0
11.16.0
```

Expected installation result: `npm ci` completes without `EBADENGINE`.
