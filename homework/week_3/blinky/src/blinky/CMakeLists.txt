# app is defined by the Zephyr toolchain, so no need to introduce it
# add_executable(app)

add_subdirectory(src)

target_link_libraries(
    app
    PRIVATE
    display
)
