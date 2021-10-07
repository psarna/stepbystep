#include <iostream>
#include <thread>
#include <chrono>

extern "C" {
bool test_me_poll();
}

int main() {
    while (!test_me_poll()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
        std::cout << "poll..." << std::endl;
    }
}
