#include <stdint.h>
#include <stdbool.h>

inline __attribute__((always_inline)) uint16_t
pixel_value(uint8_t r, uint8_t g, uint8_t b)
{

	return r + g + b;
}

inline __attribute__((always_inline)) uint32_t
chunk_value(uint16_t pixel, uint32_t * previous)
{
	*previous += pixel;
}

uint16_t
mean(uint32_t total, uint32_t size)
{
	return (uint16_t)(total / size);
}

uint16_t
get_max_diff(uint16_t last, uint16_t first, uint16_t size)
{
	return (uint16_t)(((last - first) / (size - 1)) / 2);
}

inline __attribute__((always_inline)) bool
is_similar(uint16_t c1, uint16_t c2, uint16_t max_diff)
{
	return c1 - c2 <= max_diff;
}

int
main(void)
{
	return 0;
}
