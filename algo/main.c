#include <math.h>
#include <stdint.h>
#include <stdbool.h>

#define __TESTS_ENABLED__
#define __TEST_ALL_SIMILAR_CHUNKS__

#ifdef __TESTS_ENABLED__
#include <stdlib.h>
#include <stdio.h>
#endif

inline __attribute__((always_inline)) uint16_t
pixel_value(uint8_t r, uint8_t g, uint8_t b)
{

	return r + g + b;
}

inline __attribute__((always_inline)) uint16_t
chunk_value(uint16_t pixel, uint16_t * chunk)
{
	*chunk += pixel;
}

inline __attribute__((always_inline)) uint16_t
mean(uint16_t total, uint16_t size)
{
	return total / size;
}

uint16_t
get_max_diff(uint16_t first, uint16_t last, uint16_t size)
{
	return (uint16_t)(((last - first) / (size - 1)) / 2);
}

inline __attribute__((always_inline)) bool
is_similar(uint16_t c1, uint16_t c2, uint16_t max_diff)
{
	return abs(c1 - c2) <= max_diff;
}

#ifdef __TESTS_ENABLED__
void
fail(const char * err)
{
	fprintf(stderr, err);
	exit(1);
}

int
test(void)
{
	uint16_t chunks[64];
	uint16_t pixels[256];

	for (uint16_t i=0; i != 256; i++)
		pixels[i] = pixel_value(i, i, i);

	if (pixels[0] != 0 || pixels[255] != 765)
		fail("Did not initialize pixels properly.\n");

	uint16_t idx = 0;
	uint16_t chunk = 0;
	for (uint16_t i=1; i != 257; i++) {
		if (i % 4 == 0) {
			chunks[idx] = mean(chunk, 4);
			chunk = 0;
			idx++;
		}

		chunk_value(pixels[i], &chunk);
	}

	if (chunks[0] != 4 || chunks[63] != 760)
		fail("chunks were not properly calculated.\n");


	uint16_t max_diff = get_max_diff(chunks[0], chunks[63], 64);
	if (max_diff != 6)
		fail("max difference was not properly calculated.\n");

	if (is_similar(chunks[0], chunks[1], max_diff) != false)
		fail("c0 and c1 should not be similar!\n");

#ifdef __TEST_ALL_SIMILAR_CHUNKS__
	chunks[1] = 10; // Change the value of c1 so that it's similar to c0.

	for (uint16_t i=0; i != 64; i++)
		for (uint16_t j=0; j != 64; j++) {
			if (i == j) continue;
			if (is_similar(chunks[i], chunks[j], max_diff))
				printf("c%d ≈ c%d\n", i, j);
		}
#endif

	return 0;
}
#endif

int
main(void)
{
#ifdef __TESTS_ENABLED__
	int res = test();
	if (res == 0) printf("All tests successfully passed!\n");
	return res;
#else
	return 0;
#endif
}
