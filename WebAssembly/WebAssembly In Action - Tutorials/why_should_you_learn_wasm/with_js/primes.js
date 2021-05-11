const IsPrime = (value) => {
    if (value == 2) { return 1; } // two is the only even prime number
    if (value <= 1 || value % 2 == 0) { return 0; } // zero and one aren't prime

    // iterate until the sqrt of the number and skip even #s
    for (let i = 3; (i * i) <= value; i += 2) {
        if (value % i == 0) { return 0; }
    }

    return 1; // equivalent to returning true
}

const primes = () => {
    let start = 3;
    let end = 100000000;

    let then = new Date;
    let currentPrime = 2;
    for(let i = start; i <= end; i += 2) { // note we are skipping evens
        if(IsPrime(i)) {
            currentPrime = i;
        }
    }
    let now = new Date;
    document.write("LAST PRIME FOUND: ", currentPrime);
    document.write("<br/> === TIME TAKEN === <br/>" + (now - then));
}