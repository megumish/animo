#include <stdlib.h>
#include <stdio.h>

#define LENGTH_OF_OBJECT_NUM 4
#define LENGTH_OF_TOTAL_OBJECTS_SIZE 4
#define LENGTH_OF_TOTAL_ANIMATION_SIZE 4

int main(const int argc, const char **argv) {
	if(argc < 2) {
		fprintf(stderr, "Usage: animo <animo_file>");
		exit(1);
	}
	FILE *input_file = fopen(argv[1], "r");
	if (!input_file) {
		fprintf(stderr, "Open Error: Can't open %s", argv[1]);
		exit(1);
	}
	int32_t object_num = 0;
	fread((void *) &object_num, LENGTH_OF_OBJECT_NUM, 1, input_file);
	fprintf(stderr, "object_num: %d\n", object_num);

	int32_t total_objects_size = 0;
	fread((void *) &total_objects_size, LENGTH_OF_TOTAL_OBJECTS_SIZE, 1, input_file);
	fprintf(stderr, "total_objects_size: %d\n", total_objects_size);

	int32_t total_animation_size = 0;
	fread((void *) &total_animation_size, LENGTH_OF_TOTAL_ANIMATION_SIZE, 1, input_file);
	fprintf(stderr, "total_animation_size: %d\n", total_animation_size);

}
