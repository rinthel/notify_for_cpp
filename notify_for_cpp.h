#ifndef __NOTIFY_FOR_CPP_H__
#define __NOTIFY_FOR_CPP_H__

#include <functional>

namespace notify {

struct FileEvent {
    
};

void init();
void start(std::function<void(const FileEvent&)> callback);
void release();

}

#endif // __NOTIFY_FOR_CPP_H__