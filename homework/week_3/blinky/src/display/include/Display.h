#pragma once

#include <optional>
#include <cstdint>

struct device;

class Display
{
public:
    static std::optional<Display> init();

    void turnWhite();
    void turnBlack();

private:
    Display(device const *gpio0, device const *gpio1);

    void putPattern(uint8_t rows, uint8_t cols);

    device const *gpio0;
    device const *gpio1;
};
