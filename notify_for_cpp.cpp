#include "notify_for_cpp.h"

extern "C" {
extern void nfc_init();
extern void nfc_start(const char*, void(*callback)(int32_t));
extern void nfc_release();
}

namespace notify {

static std::function<void(const FileEvent&)> s_callback;

void init() {
    nfc_init();
}

void native_callback(int32_t code) {
    s_callback(FileEvent {});
}

void start(const std::string& path, std::function<void(const FileEvent&)> callback) {
    s_callback = callback;
    nfc_start(path.c_str(), native_callback);
}

void release() {
    nfc_release();
}

}
