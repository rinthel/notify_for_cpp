#ifndef __NOTIFY_FOR_CPP_H__
#define __NOTIFY_FOR_CPP_H__

#include <functional>
#include <string>

namespace notify {

enum FileEventType {
    NOTICE_WRITE = 1,
    NOTICE_REMOVE = 2,
    CREATE = 3,
    WRITE = 4,
    CHMOD = 5,
    REMOVE = 6,
    RENAME = 7,
    RESCAN = 8,
    ERROR = -1,
};

struct FileEvent {
    FileEventType type;
    std::string contents;
};

void start(const std::string& path, std::function<void(const FileEvent&)> callback);
void stop();

}

#endif // __NOTIFY_FOR_CPP_H__