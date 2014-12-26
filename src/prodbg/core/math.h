#pragma once

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct Rect
{
    int x;
    int y;
    int width;
    int height;
} Rect;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct FloatRect
{
    float x;
    float y;
    float width;
    float height;
} FloatRect;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#define int_min(a, b) ((a < b) ? b : a)

