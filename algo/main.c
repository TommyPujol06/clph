#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdarg.h>
#include <stdio.h>

// #define __DEBUG__
// #define __TESTS_ENABLED__
// #define __TEST_ALL_SIMILAR_CHUNKS__

#if defined(__TEST_ALL_SIMILAR_CHUNKS__) && !defined(__TESTS_ENABLED__)
FAIL("You need to define __TESTS_ENABLED__ for __TEST_ALL_SIMILAR_CHUNKS__ to work.\n");
#endif

#define FAIL(fmt, ...) fprintf(stderr, "(%s)[%s:%zu] "fmt, __func__, __FILE__, \
		__LINE__, ##__VA_ARGS__); exit(1);

typedef struct {
	FILE * __file;
	size_t size;
} image_t;


image_t *
img_open(const char * image_path)
{
	FILE * __file = fopen(image_path, "rb+");
	if (__file == NULL) {
		FAIL("Could not open file: %s\n", image_path);
	}

	if (fseek(__file, 0, SEEK_END) < 0) {
		goto error;
	}

	long size = ftell(__file);
	if (size < 0) {
		goto error;
	}

	if (fseek(__file, 0, SEEK_SET) < 0) {
		goto error;
	}

	image_t * tmp = malloc(sizeof(image_t));
	tmp->__file = __file;
	tmp->size = (size_t)size;

	return tmp;

error:
	if (__file != NULL)
		fclose(__file);

	FAIL("Something unexpected happened while opening the image.\n");
}

uint32_t
img_read(image_t * img, uint32_t * buffer, uint32_t offset, uint32_t size)
{
	if (buffer == NULL)
		FAIL("Did not allocate buffer.\n");

	if (img->size < size)
		FAIL("You are trying to read more bytes than there are in the image.");

	if (sizeof(buffer) < size) {
		goto clean;
		FAIL("Buffer is not big enough.\n");
	}

clean:
	free(buffer);

	return 0;
}

uint32_t
img_write(image_t * img, uint32_t byte, uint32_t x, uint32_t y)
{
	return 0;
}

void
img_close(image_t * img)
{
	fclose(img->__file);
	free(img);
}

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
int
test(void)
{
	uint16_t chunks[64];
	uint16_t pixels[256];

	for (uint16_t i=0; i != 256; i++)
		pixels[i] = pixel_value(i, i, i);

	if (pixels[0] != 0 || pixels[255] != 765)
		FAIL("Did not initialize pixels properly.\n");

	uint16_t idx = 0;
	uint16_t chunk = 0;
	for (uint16_t i=0; i != 256; i++) {
		chunk_value(pixels[i], &chunk);

#ifdef __DEBUG__
		printf("i+1=%d |=> %4 = %d\n", i+1, (i+1) % 4);
#endif
		if ((i+1) % 4 == 0) {
#ifdef __DEBUG__
			printf("[%d:%d]\tc=%d\tm=%d\n", idx, i, chunk, mean(chunk, 4));
#endif
			chunks[idx] = mean(chunk, 4);
			chunk = 0;
			idx++;
		}
	}

	if (chunks[0] != 4 || chunks[63] != 760)
		FAIL("Chunks were not properly calculated.\n");


	uint16_t max_diff = get_max_diff(chunks[0], chunks[63], 64);
	if (max_diff != 6)
		FAIL("Max difference was not properly calculated.\n");

	if (is_similar(chunks[0], chunks[1], max_diff) != false)
		FAIL("c0 and c1 should not be similar!\n");

#ifdef __TEST_ALL_SIMILAR_CHUNKS__
	chunks[1] = 10; // Change the value of c1 so that it's similar to c0.

	// FIXME: O(n^2)
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
	image_t * img = img_open("sample.jpeg");
	printf("Image size (bytes): %zu\n", img->size);
	img_close(img);

#ifdef __TESTS_ENABLED__
	int res = test();
	if (res == 0) printf("All tests successfully passed!\n");
	return res;
#else
	return 0;
#endif
}
