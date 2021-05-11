let moduleExports = null;

const init = () => {
    WebAssembly.instantiateStreaming(fetch("primes.wasm"))
        .then((result) => {
            moduleExports = result.instance.exports;
            moduleExports._initialize();
        })
        .catch((error) => {
            console.log(error);
        });
}

const primes = () => {
    let then = new Date;
    document.write("LAST PRIME FOUND: ", moduleExports.primes(100000000));
    let now = new Date;
    document.write("<br/> === TIME TAKEN === <br/>" + (now - then));
}