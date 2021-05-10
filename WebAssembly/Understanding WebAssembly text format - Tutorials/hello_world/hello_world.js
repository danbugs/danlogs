let import_object = {
    "js":{
        "mem": new WebAssembly.Memory({initial: 1}),
        "decode_print": (start, end) => {
            let typed_array = new Uint8Array(import_object.js.mem.buffer, start, end);
            document.write(new TextDecoder("utf-8").decode(typed_array));
        }
    }
}
fetch("hello_world.wasm")
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, import_object))
    .then(result =>{
        let exports = result.instance.exports;
        exports.hello_world();
    })