#include "notify_for_cpp.h"
#include <iostream>

int main(int argc, const char** argv) {
    auto callback = [](const notify::FileEvent& event) {
        std::cout << "event type: " << (int)event.type << std::endl;
        std::cout << "event path: " << event.contents << std::endl;
    };
    std::string path = "/Users/rinthel/Desktop";
    notify::stop();
    notify::start(path, callback);
    std::cout << "press any key to stop" << std::endl;
    getchar();
    notify::stop();
    return 0;
}