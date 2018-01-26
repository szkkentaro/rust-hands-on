const ffi = require("ffi");

const lib = ffi.Library('target/release/libembed.dylib', {
    'process': ['void', []]
});

lib.process();
console.log('done!');