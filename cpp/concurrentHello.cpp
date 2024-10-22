#include <iostream>
#include <thread>

void helloThread() {
    std::cout << "Hello from thread: " << std::this_thread::get_id() << '\n';
}

int main() {
    std::cout << "I am main\n";
    std::thread t(helloThread);
    std::thread u(helloThread);

    t.join();
    u.join();
}
