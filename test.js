// Smoke test (run on Windows after `npm run build`):
//
//   node test.js "C:\\path\\to\\folder"          -> soft refresh (SHChangeNotify)
//   node test.js "C:\\path\\to\\folder" --clear   -> also rebuild the icon cache
//
// Proper test scenario:
//   1. apply icon A to the folder (seticon)
//   2. change to icon B  -> Explorer still shows A
//   3. run this -> it should flip to B (try --clear if the soft refresh is not enough)

const { refreshFolder, clearIconCache } = require('./index.js');

const target = process.argv[2] || process.cwd();
const alsoClear = process.argv.includes('--clear');

const ok = refreshFolder(target);
console.log(`refreshFolder(${JSON.stringify(target)}) -> ${ok}`);

if (alsoClear) {
    const cleared = clearIconCache();
    console.log(`clearIconCache() -> ${cleared}`);
}

console.log(ok
    ? 'Notification sent (Windows). Check your Explorer window.'
    : 'No-op (non-Windows) or failed.');
