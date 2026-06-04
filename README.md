<div align="center">

# win-folder-refresh

Refresh a Windows folder icon in Explorer via `SHChangeNotify`.

</div>

A tiny **native N-API addon** (Rust) that tells Windows Explorer a folder
changed, so a freshly applied custom icon shows up **without restarting
Explorer**. It calls the Win32 `SHChangeNotify` API directly — the same
mechanism the native "Change icon" dialog uses — which a Node process can't do
on its own and which WASM cannot reach (WASM has no OS-API access).

On **non-Windows** platforms it is a safe no-op that returns `false`, so
cross-platform code can call it unconditionally.

## Why

Setting a folder icon writes `desktop.ini`, but Explorer often keeps showing the
cached icon until its shell is notified. Pure-Node attempts (PowerShell
`SHChangeNotify`, `ie4uinit`) are slow or unreliable. A native addon notifies
the shell instantly and properly.

## Install

```bash
npm i win-folder-refresh
```

## Usage

```js
const { refreshFolder } = require('win-folder-refresh');

// After writing the folder's desktop.ini:
refreshFolder('C:\\Users\\me\\Desktop\\MyFolder'); // true on Windows
```

```ts
import { refreshFolder } from 'win-folder-refresh';
```

| Function | Signature | Returns |
| -------- | --------- | ------- |
| `refreshFolder(path)` | `(string) => boolean` | `true` if the shell was notified (Windows), `false` otherwise |

## How it works

For the given folder it calls, via the `windows` crate:

- `SHChangeNotify(SHCNE_UPDATEITEM, SHCNF_IDLIST, pidl, ...)` — notify the exact
  item by its PIDL (built with `ILCreateFromPath`), like the shell does when an
  icon changes;
- `SHChangeNotify(SHCNE_UPDATEDIR, SHCNF_PATHW, path, ...)` — nudge the directory
  by path as well.

> Note: even native notification is not a hard guarantee — Windows' icon cache
> can lag. This is the most reliable, instant approach available to an external
> process, but a stubborn cache may still need a manual refresh (F5).

## Build from source

Requires the Rust toolchain and Node. Build **on Windows** (it links Windows
APIs):

```bash
npm install
npm run build        # -> win-folder-refresh.<triple>.node + index.js/.d.ts
node test.js "C:\\path\\to\\folder"
```

## Scope

- ✅ Notify Explorer to refresh a folder's icon.
- ❌ It does not set the icon itself (write `desktop.ini` yourself first).

## License

MIT
