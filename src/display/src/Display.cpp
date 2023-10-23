#include "Display.h"

#include <zephyr/device.h>
#include <zephyr/drivers/gpio.h>

#include <array>
#include <cstdio>

namespace
{
    uint8_t constexpr ROW0_PORT_BIT = 21;
    uint8_t constexpr ROW1_PORT_BIT = 22;
    uint8_t constexpr ROW2_PORT_BIT = 15;
    uint8_t constexpr ROW3_PORT_BIT = 24;
    uint8_t constexpr ROW4_PORT_BIT = 19;

    uint8_t constexpr COL0_PORT_BIT = 28;
    uint8_t constexpr COL1_PORT_BIT = 11;
    uint8_t constexpr COL2_PORT_BIT = 31;
    uint8_t constexpr COL3_PORT_BIT = 5;
    uint8_t constexpr COL4_PORT_BIT = 30;

    uint8_t constexpr ROW0_MASK = 1;
    uint8_t constexpr ROW1_MASK = 1 << 1;
    uint8_t constexpr ROW2_MASK = 1 << 2;
    uint8_t constexpr ROW3_MASK = 1 << 3;
    uint8_t constexpr ROW4_MASK = 1 << 4;

    uint8_t constexpr COL0_MASK = 1;
    uint8_t constexpr COL1_MASK = 1 << 1;
    uint8_t constexpr COL2_MASK = 1 << 2;
    uint8_t constexpr COL3_MASK = 1 << 3;
    uint8_t constexpr COL4_MASK = 1 << 4;
}

Display::Display(device const *gpio0, device const *gpio1)
    : gpio0(gpio0), gpio1(gpio1)
{
}

std::optional<Display> Display::init()
{
    device const *gpio0 = device_get_binding("GPIO_0");

    if (!gpio0)
    {
        printf("Failed to acquire GPIO_0.\n");
        return std::nullopt;
    }

    device const *gpio1 = device_get_binding("GPIO_1");

    if (!gpio1)
    {
        printf("Failed to acquire GPIO_1.\n");
        return std::nullopt;
    }

    std::array<uint8_t, 9> gpio0Pins{
        ROW0_PORT_BIT,
        ROW1_PORT_BIT,
        ROW2_PORT_BIT,
        ROW3_PORT_BIT,
        ROW4_PORT_BIT,
        COL0_PORT_BIT,
        COL1_PORT_BIT,
        COL2_PORT_BIT,
        // col 3 is addressed using port 1
        COL4_PORT_BIT};

    for (auto pin : gpio0Pins)
    {
        if (gpio_pin_configure(gpio0, pin, GPIO_OUTPUT) != 0)
        {
            printf("Failed to configure pin %d on port 0.\n", pin);
            return std::nullopt;
        }
    }
    if (gpio_pin_configure(gpio1, COL3_PORT_BIT, GPIO_OUTPUT) != 0)
    {
        printf("Failed to configure pin %d on port 1.\n", COL3_PORT_BIT);
        return std::nullopt;
    }

    Display disp(gpio0, gpio1);

    return {disp};
}

void Display::turnWhite()
{
    putPattern(0xff, 0xff);
}

void Display::turnBlack()
{
    putPattern(0, 0);
}

void Display::putPattern(uint8_t rows, uint8_t cols)
{
    // Note that there is no error handling included in here at the moment.

    // columns sink current and are active low
    cols = ~cols;

    gpio_pin_set(gpio0, ROW0_PORT_BIT, rows & ROW0_MASK);
    gpio_pin_set(gpio0, ROW1_PORT_BIT, rows & ROW1_MASK);
    gpio_pin_set(gpio0, ROW2_PORT_BIT, rows & ROW2_MASK);
    gpio_pin_set(gpio0, ROW3_PORT_BIT, rows & ROW3_MASK);
    gpio_pin_set(gpio0, ROW4_PORT_BIT, rows & ROW4_MASK);

    gpio_pin_set(gpio0, COL0_PORT_BIT, cols & COL0_MASK);
    gpio_pin_set(gpio0, COL1_PORT_BIT, cols & COL1_MASK);
    gpio_pin_set(gpio0, COL2_PORT_BIT, cols & COL2_MASK);
    gpio_pin_set(gpio1, COL3_PORT_BIT, cols & COL3_MASK);
    gpio_pin_set(gpio0, COL4_PORT_BIT, cols & COL4_MASK);
}