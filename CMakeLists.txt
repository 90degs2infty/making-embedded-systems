cmake_minimum_required(VERSION 3.20)

# We're targeting the micro:bit v2 (and only that one)
set(BOARD bbc_microbit_v2)

find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})

project(
    blinky
    VERSION 0.1.0
    DESCRIPTION "A workspace blinky application based on Zephyr targeting the micro:bit."
    LANGUAGES ASM C CXX
)

target_sources(app PRIVATE src/main.cpp)
