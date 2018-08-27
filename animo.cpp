#include <cstdlib>
#include <cstdio>
#include <vector>

#define LENGTH_OF_OBJECT_NUM 4
#define LENGTH_OF_TOTAL_ANIMATION_SIZE 4
#define LENGTH_OF_OBJECT_SIZE 4

struct Object {
	private:
	int32_t object_size;
	char *content;
	public:
	Object(int32_t object_size, char *content): object_size(object_size), content(content) {};
};

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

	int32_t total_animation_size = 0;
	fread((void *) &total_animation_size, LENGTH_OF_TOTAL_ANIMATION_SIZE, 1, input_file);
	fprintf(stderr, "total_animation_size: %d\n", total_animation_size);

	std::vector<Object> objects;
	for (int num_of_object = 0; num_of_object < object_num; ++num_of_object) {
		int32_t object_size = 0;
		fread((void *) &object_size, LENGTH_OF_OBJECT_SIZE, 1, input_file);

		char *object = (char *)malloc(object_size);
		fread((void *) object, object_size, 1, input_file);
		fprintf(stderr, "object [%d]: size(%d), ", num_of_object, object_size);
		fwrite((void *) object, object_size, 1, stderr);
		fprintf(stderr, "\n");
		objects.push_back(Object(object_size, object));
	}
}
