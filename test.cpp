#include "notify_for_cpp.h"
#include <iostream>

int main(int argc, const char** argv) {
    std::cout << "hello, world" << std::endl;

    auto callback = [](const notify::FileEvent& event) {

    };
    notify::init();
    notify::start(callback);
    while (true) {
        
    }
    return 0;
}