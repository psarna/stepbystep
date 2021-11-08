#include <iostream>
#include <thread>
#include <chrono>

extern "C" {
bool test_me_poll(size_t idx);
bool test_me_init(size_t idx);
void put_payload(size_t idx, const char* payload);
}

int main() {
    test_me_init(0);
    put_payload(3, "hello");
    put_payload(7, "world");
    while (!test_me_poll(0)) {
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
        std::cout << "<preempt happened>" << std::endl;
    }
}
