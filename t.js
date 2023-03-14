'use strict'

const wasm = require('./pkg/hello');

let x = wasm.what('bruce, how are you?');

console.log(x)
