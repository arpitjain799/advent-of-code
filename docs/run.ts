function exitProcess(message) {
	console.log(message);
	Deno.exit(1);
}

const argv = Deno.args;
const day = parseInt(argv[0]);
const part = parseInt(argv[1]);
if (!(day >= 1 && day <= 25)) {
    exitProcess('Invalid day - must be integer between 1 and 25');
} else if (!(part >= 1 && part <= 2)) {
    exitProcess('Invalid part - must be 1 or 2');
}

const input = Deno.readAllSync(Deno.stdin);

function solve(wasmCodeBuffer, day, part, inputBuffer) {
    const wasmModule = new WebAssembly.Module(wasmCodeBuffer);
    const wasmInstance = new WebAssembly.Instance(wasmModule);
    const wasm = wasmInstance.exports;

    const outputPointer = 8;
    const inputLength = inputBuffer.length;
    const inputPointer = wasm.__wbindgen_malloc(inputLength);
    if (isDeno) {
        var memoryBufferArray = new Uint8Array(wasm.memory.buffer);
        for (let i = 0; i < inputLength; i++) {
            memoryBufferArray[i + inputPointer] = inputBuffer[i];
        }
    } else {
        Buffer.from(wasm.memory.buffer).write(inputBuffer, inputPointer, inputLength);
    }

    wasm.solve(outputPointer, day, part, inputPointer, inputLength);

    const memi32 = new Int32Array(wasm.memory.buffer);
    const ptr = memi32[outputPointer / 4 + 0];
    const len = memi32[outputPointer / 4 + 1];
    const outputString = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true })
        .decode(new Uint8Array(wasm.memory.buffer).subarray(ptr, ptr + len));
        //.slice();
    // wasm.__wbindgen_free(memi32[outputPointer / 4 + 0], memi32[outputPointer / 4 + 1] * 1);

    console.log(outputString);
}

const hostname = 'fornwall.github.io';
const path = '/advent-of-code-2019-rs/module.wasm';

const wasmUrl = 'https://' + hostname + path;
fetch(wasmUrl).then(response =>
	response.arrayBuffer()
).then(wasmCode =>
	solve(wasmCode, day, part, input)
);
