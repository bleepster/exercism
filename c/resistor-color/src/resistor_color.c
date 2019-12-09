#include "resistor_color.h"

resistor_band_t* colors()
{
    static resistor_band_t colors[] = {
        BLACK, BROWN, RED, ORANGE, YELLOW, GREEN, BLUE, VIOLET, GREY, WHITE
    };
    return colors;
}

int color_code(int color)
{
    return colors()[color];
}


