let import_object = {
    "js":{
        "print": (arg) => document.writeln(arg),
        "g1": new WebAssembly.Global({value: "i32", mutable: true}, 123),
        "mem": new WebAssembly.Memory({initial: 1})
    }
}
fetch("globals_memory.wasm")
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, import_object))
    .then(result => {
        // result.instance.exports.getG1();
        // result.instance.exports.setG1(1234);
        result.instance.exports.populateMem();

        result.instance.exports.getAt(0);
        result.instance.exports.getAt(10);

        document.writeln(import_object.js.mem.buffer.byteLength)
        import_object.js.mem.grow(1);
        document.writeln(import_object.js.mem.buffer.byteLength)
    })