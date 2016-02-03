#include <stdlib.h>

int main(int argc, char const *argv[]) {
	volatile int *x = (int*)malloc(10 * sizeof(int));
	return 0;
}