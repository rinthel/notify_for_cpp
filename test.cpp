#include "notify_for_cpp.h"
#include <iostream>

int main(int argc, const char** argv) {
    std::cout << "hello, world" << std::endl;

    auto callback = [](const notify::FileEvent& event) {
        std::cout << "cpp callback called" << std::endl;
    };
    notify::init();
    notify::start("/Users/rinthel/Desktop", callback);
    while (true) {
        
    }
    return 0;
}