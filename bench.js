// Benchmark the refresh strategies on YOUR machine.
//
//   node bench.js "C:\\path\\to\\folder" <strategy>
//
// strategy is one of:
//   notify   -> refreshFolder            (SHChangeNotify only)
//   show     -> refreshIe4uinitShow      (ie4uinit -show)
//   clear    -> refreshIe4uinitClear     (ie4uinit -ClearIconCache)
//   files    -> refreshClearCacheFiles   (delete IconCache.db + notify)
//   all      -> refreshAll               (notify + delete cache + ie4uinit)
//
// HOW TO TEST FAIRLY (per strategy):
//   1. apply icon A to the folder (seticon), confirm Explorer shows A
//   2. change to icon B (seticon) — Explorer still shows A
//   3. run:  node bench.js "C:\\folder" <strategy>
//   4. start a stopwatch and note how long until the icon flips to B
//      (0s = instant, or "never" if it doesn't change)
//
// Run each strategy on a FRESH A->B change (the cache state matters).

const m = require('./index.js');

const target = process.argv[2] || process.cwd();
const strategy = (process.argv[3] || 'notify').toLowerCase();

const map = {
    notify: () => m.refreshFolder(target),
    show: () => m.refreshIe4uinitShow(),
    clear: () => m.refreshIe4uinitClear(),
    files: () => m.refreshClearCacheFiles(target),
    all: () => m.refreshAll(target),
};

const fn = map[strategy];
if (!fn) {
    console.error(`Unknown strategy "${strategy}". Use: ${Object.keys(map).join(', ')}`);
    process.exit(2);
}

const t0 = Date.now();
const ok = fn();
const ms = Date.now() - t0;
console.log(`[${strategy}] returned ${ok} in ${ms}ms`);
console.log('Now watch the folder — time how long until the icon changes.');
