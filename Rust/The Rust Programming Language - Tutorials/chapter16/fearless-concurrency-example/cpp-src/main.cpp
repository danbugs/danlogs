#include <iostream>
#include <thread>
#include <chrono>

void print(int& n) {
    for (int i = 0; i < 5; ++i) {
        std::cout << n << " ";
        n += 1;
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
    }
}

int main() {
    int shared_value = 0;
    std::thread t1(print, std::ref(shared_value));
    std::thread t2(print, std::ref(shared_value));
    t1.join();
    t2.join();
    return 0;
}
