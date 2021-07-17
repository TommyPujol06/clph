#ifndef __IMAGE_H__

#include "common.h"

typedef struct {
	FILE * __file;
	size_t size;
	uint32_t * data;
} image_t;

image_t * open(const char * image_path);
uint32_t read(image_t * img);
uint32_t write(image_t * img, uint32_t byte, uint32_t x, uint32_t y);
void close(image_t * img);

#define __IMAGE_H__
#endif
