cmake_minimum_required(VERSION 3.6)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED on)

project(notify_for_cpp)

add_library(notify_for_cpp
    notify_for_cpp.cpp
    notify_for_cpp.h)

add_executable(notify_for_cpp_test
    test.cpp)

option(NOTIFY_RUST_DEBUG OFF)
# target_link_libraries(notify_for_cpp_test ${PROJECT_BINARY_DIR}/libnotify.a)

if (WIN32)
    set(NOTIFY_LINK_LIB_NAME notify_rust.dll.lib)
    set(NOTIFY_SHARED_LIB_NAME notify_rust.dll)
    set(NOTIFY_RUST_BUILD_OPTION --target=i686-pc-windows-msvc)
    set(NOTIFY_RUST_TARGET_DIR i686-pc-windows-msvc)
else()
    set(NOTIFY_LINK_LIB_NAME libnotify_rust.dylib)
    set(NOTIFY_SHARED_LIB_NAME libnotify_rust.dylib)
    set(NOTIFY_RUST_TARGET_DIR .)
endif()

if (NOTIFY_RUST_DEBUG)
    set(NOTIFY_RUST_BUILD cargo build ${NOTIFY_RUST_BUILD_OPTION})
    set(NOTIFY_RUST_BUILD_DIR ${CMAKE_CURRENT_SOURCE_DIR}/notify_rust/target/${NOTIFY_RUST_TARGET_DIR}/debug)
else()
    set(NOTIFY_RUST_BUILD cargo build --release ${NOTIFY_RUST_BUILD_OPTION})
    set(NOTIFY_RUST_BUILD_DIR ${CMAKE_CURRENT_SOURCE_DIR}/notify_rust/target/${NOTIFY_RUST_TARGET_DIR}/release)
endif()

set(NOTIFY_RUST_LINK_LIB ${NOTIFY_RUST_BUILD_DIR}/${NOTIFY_LINK_LIB_NAME})
set(NOTIFY_RUST_SHARED_LIB ${NOTIFY_RUST_BUILD_DIR}/${NOTIFY_SHARED_LIB_NAME})

target_link_libraries(notify_for_cpp_test
    notify_for_cpp ${NOTIFY_RUST_LINK_LIB})

add_dependencies(notify_for_cpp_test notify_for_cpp)

add_custom_command(TARGET notify_for_cpp POST_BUILD
    COMMAND ${NOTIFY_RUST_BUILD}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/notify_rust)
add_custom_command(TARGET notify_for_cpp_test POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy ${NOTIFY_RUST_SHARED_LIB} ${PROJECT_BINARY_DIR}/)

# add_custom_command(TARGET notify_for_cpp POST_BUILD
#     COMMAND libtool -static
#         ${CMAKE_CURRENT_SOURCE_DIR}/notify_rust/target/debug/libnotify_rust.a
#         ${PROJECT_BINARY_DIR}/libnotify_for_cpp.a
#         -o ${PROJECT_BINARY_DIR}/libnotify.a)

install(FILES ${NOTIFY_RUST_SHARED_LIB} DESTINATION bin)
install(TARGETS notify_for_cpp_test DESTINATION bin)
install(TARGETS notify_for_cpp DESTINATION lib)
if(WIN32)
    install(FILES ${NOTIFY_RUST_LINK_LIB} DESTINATION lib)
endif()
install(FILES notify_for_cpp.h DESTINATION include)