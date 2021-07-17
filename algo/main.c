#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>

inline __attribute__((always_inline)) uint16_t
pixel_value(uint8_t r, uint8_t g, uint8_t b)
{

	return r + g + b;
}

inline __attribute__((always_inline)) uint32_t
chunk_value(uint16_t pixel, uint32_t * chunk)
{
	*chunk += pixel;
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
test(void)
{
	uint16_t chunks[64];
	uint16_t pixels[256];

	for (uint16_t i=0; i != 256; i++)
		pixels[i] = pixel_value(i, i, i);

	if (pixels[0] != 0 || pixels[255] != 765)
		return -1;

	uint16_t idx = 0;
	uint32_t chunk = 0;
	for (uint16_t j=0; j != 256; j++) {
		if (j % 4 == 0) {
			chunks[idx] = mean(chunk, 4);
			chunk = 0;
			idx++;
		}

		chunk_value(pixels[j], &chunk);
	}

	chunks[++idx] = mean(chunk, 4);
	chunk = 0;

	printf("%d\n%d\n", chunks[0], chunks[63]);
	if (chunks[0] != 4 || chunks[63] != 760)
		return -1;

	return 0;
}

int
main(void)
{
	return test();
}
