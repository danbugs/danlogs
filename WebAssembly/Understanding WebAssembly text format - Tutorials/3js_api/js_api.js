fetch("js_api.wasm")
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes))
    .then(response => {
        let result = response.instance.exports.add(1243,2);
        console.log(result);
    });
    