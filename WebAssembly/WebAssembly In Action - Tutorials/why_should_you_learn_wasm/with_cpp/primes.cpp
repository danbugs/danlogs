#ifdef __EMSCRIPTEN__
#include <emscripten.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

#ifdef __EMSCRIPTEN__
EMSCRIPTEN_KEEPALIVE
#endif
int IsPrime(int value) {
    if (value == 2) { return 1; } // two is the only even prime number
    if (value <= 1 || value % 2 == 0) { return 0; } // zero and one aren't prime

    // iterate until the sqrt of the number and skip even #s
    for (int i = 3; (i * i) <= value; i += 2) {
        if (value % i == 0) { return 0; }
    }

    return 1; // equivalent to returning true
}

#ifdef __EMSCRIPTEN__
EMSCRIPTEN_KEEPALIVE
#endif
int primes (int end) {
    int start = 3;
    int currentPrime = 2;

    for(int i = start; i <= end; i += 2) { // note we are skipping evens
        if(IsPrime(i)) {
            currentPrime = i;
        }
    }

    return currentPrime;
}

#ifdef __cplusplus
}
#endif