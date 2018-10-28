#include "notify_for_cpp.h"
#include <iostream>

int main(int argc, const char** argv) {
    auto callback = [](const notify::FileEvent& event) {
        std::cout << "event type: " << (int)event.type << std::endl;
        std::cout << "event path: " << event.contents << std::endl;
    };
    if (argc < 2) {
        std::cout << "usage: notify_for_cpp_test <dirname>" << std::endl;
        return 0;
    }
    std::string path(argv[1]);
    notify::stop();
    notify::start(path, callback);
    std::cout << "press any key to stop" << std::endl;
    getchar();
    notify::stop();
    return 0;
}