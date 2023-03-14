
import * as wfd from 'wasm-feature-detect';

let promises = [];
let names = [];

let bigInt= await wfd.bigInt();
promises.push(bigInt);
names.push('bigInt');
let bulkMemory= await wfd.bulkMemory();
promises.push(bulkMemory);
names.push('bulkMemory');
let exceptions= await wfd.exceptions();
promises.push(exceptions);
names.push('exceptions');
let extendedConst= await wfd.extendedConst();
promises.push(extendedConst);
names.push('extendedConst');
let gc= await wfd.gc();
promises.push(gc);
names.push('gc');
let memory64= await wfd.memory64();
promises.push(memory64);
names.push('memory64');
let multiValue= await wfd.multiValue();
promises.push(multiValue);
names.push('multiValue');
let mutableGlobals= await wfd.mutableGlobals();
promises.push(mutableGlobals);
names.push('mutableGlobals');
let referenceTypes= await wfd.referenceTypes();
promises.push(referenceTypes);
names.push('referenceTypes');
let relaxedSimd= await wfd.relaxedSimd();
promises.push(relaxedSimd);
names.push('relaxedSimd');
let saturatedFloatToInt= await wfd.saturatedFloatToInt();
promises.push(saturatedFloatToInt);
names.push('saturatedFloatToInt');
let signExtensions= await wfd.signExtensions();
promises.push(signExtensions);
names.push('signExtensions');
let simd= await wfd.simd();
promises.push(simd);
names.push('simd');
let streamingCompilation= await wfd.streamingCompilation();
promises.push(streamingCompilation);
names.push('streamingCompilation');
let tailCall= await wfd.tailCall();
promises.push(tailCall);
names.push('tailCall');
let threads= await wfd.threads();
promises.push(threads);
names.push('threads');

await Promise.all(promises);
for (let i = 0; i < names.length; i++) {
  console.log(names[i], promises[i])
}
