// Quick smoke test (run on Windows after `npm run build`):
//   node test.js "C:\\path\\to\\a\\folder"
//
// It calls refreshFolder on the given folder (or the current directory) and
// prints the result. On Windows you should see the folder's icon refresh in any
// open Explorer window after you change its desktop.ini.

const { refreshFolder } = require('./index.js');

const target = process.argv[2] || process.cwd();
const ok = refreshFolder(target);
console.log(`refreshFolder(${JSON.stringify(target)}) -> ${ok}`);
console.log(ok
    ? 'Notification sent (Windows). Check your Explorer window.'
    : 'No-op (non-Windows) or failed.');
