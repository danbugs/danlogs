// let table = new WebAssembly.Table({initial: 2, element: "funcref"});
// table.set(0, f1)
// table.set(1, f2)
// table.set(1, f3)
// table.grow(1)
// table.length

fetch('tables.wasm')
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes))
    .then(result => {
        let exports = result.instance.exports;
        document.write(exports.call_by_index(0));
    })