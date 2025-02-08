#include <stdio.h>
#include <string.h>
#include <stdint.h>

#define MAX 128
struct ring {
	uint64_t buf[MAX];
	int head;
	int count;
	int max;
};

void init_ring(struct ring *r)
{
	memset(r, 0, sizeof(struct ring));
	r->max = MAX;
}

int push(struct ring *r, uint64_t element)
{
	int position;

	if (r->count == r->max)
		return -1;

	position = (r->head + r->count) % r->max;

	r->buf[position] = element;
	r->count += 1;

	return 0;
}

uint64_t pop(struct ring *r)
{
	uint64_t ret;

	if (r->count == 0)
		return -1;

	ret = r->buf[r->head];
	r->count -= 1;

	r->head = (r->head + 1) % r->max;

	return ret;
}

int main() {
	struct ring r;
	init_ring(&r);

	for(int i=0; i < MAX+1; i++) {
		int ret = push(&r, i);
		if (ret != 0)
			printf("failed to push %d\n", i);
	}

	for(int i=0; i < MAX+1; i++) {
		uint64_t ret = pop(&r);
		printf("%d = %ld\n", i, ret);
	}

}
