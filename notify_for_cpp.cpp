#include "notify_for_cpp.h"

extern "C" {
extern void nfc_init();
extern void nfc_start(void(*callback)(int32_t));
extern void nfc_release();
}

namespace notify {

void init() {
    nfc_init();
}

void start(std::function<void(const FileEvent&)> callback) {
    nfc_start(nullptr);
}

void release() {
    nfc_release();
}

}
