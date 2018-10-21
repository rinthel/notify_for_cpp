cmake_minimum_required(VERSION 3.6)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED on)

project(notify_for_cpp)

add_library(notify_for_cpp
    notify_for_cpp.cpp
    notify_for_cpp.h)

add_executable(notify_for_cpp_test
    test.cpp)

# target_link_libraries(notify_for_cpp_test ${PROJECT_BINARY_DIR}/libnotify.a)
target_link_libraries(notify_for_cpp_test
    notify_for_cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/notify_rust/target/debug/libnotify_rust.dylib)

add_dependencies(notify_for_cpp_test notify_for_cpp)

add_custom_command(TARGET notify_for_cpp POST_BUILD
    COMMAND cargo build
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/notify_rust)

# add_custom_command(TARGET notify_for_cpp POST_BUILD
#     COMMAND libtool -static
#         ${CMAKE_CURRENT_SOURCE_DIR}/notify_rust/target/debug/libnotify_rust.a
#         ${PROJECT_BINARY_DIR}/libnotify_for_cpp.a
#         -o ${PROJECT_BINARY_DIR}/libnotify.a)