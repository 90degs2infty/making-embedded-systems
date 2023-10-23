add_library(display)

add_subdirectory(include)
add_subdirectory(src)

target_link_libraries(
    display
    PRIVATE
    zephyr
)
