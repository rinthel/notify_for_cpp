#include "notify_for_cpp.h"

extern "C" {
extern void nfc_start(const char*, void(*callback)(int32_t, const char*));
extern void nfc_stop();
}

namespace notify {

static std::function<void(const FileEvent&)> s_callback;

void native_callback(int32_t code, const char* pathOrError) {
    s_callback(FileEvent {
        (FileEventType)code,
        std::string(pathOrError),
    });
}

void start(const std::string& path, std::function<void(const FileEvent&)> callback) {
    s_callback = callback;
    nfc_start(path.c_str(), native_callback);
}

void stop() {
    nfc_stop();
}

}
