let import_object = {
    "imports": {
        "console.log": (arg) => console.log(arg)
    }
}

fetch("call_import.wasm")
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, import_object))
    .then(response => {
        let result = response.instance.exports.add(1243,2);
    });
    