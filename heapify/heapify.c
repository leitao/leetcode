#include <stdio.h>
#include <strings.h>
#include <time.h>
#include <stdlib.h>


struct heap {
	int *array;
	int len;
};

int right(int i)
{
	return 2*i + 1;
}

int left(int i)
{
	return 2*i;
}

void pushdown(struct heap *heap, int i)
{
	int max = heap->len;
	int *array = heap->array;

	if (left(i) >= max && right(i) >= max)
		return;

	if (left(i) < max && array[i] < array[left(i)]) {
		int tmp = array[left(i)];
		array[left(i)] = array[i];
		array[i] = tmp;
		pushdown(heap, left(i));
	}
	if (right(i) < max && array[i] < array[right(i)]) {
		int tmp = array[right(i)];
		array[right(i)] = array[i];
		array[i] = tmp;
		pushdown(heap, right(i));
	}
}

void heapify(struct heap *h)
{
	int max = h->len;

	for (int i = max / 2 ; i >= 1; i--) {
		if (left(i) >= max && right(i) >= max)
			continue;

		pushdown(h, i);
	}
}

int pop(struct heap *heap)
{
	int len = heap->len;

	if (len == 0)
		return -1;

	int ret = heap->array[1];
	heap->array[1] = heap->array[len];
	heap->array[len] = -1;
	heap->len -= 1;

	pushdown(heap, 1);

	return ret;
}

struct heap *init_heap(int len)
{
	struct heap *h = malloc(sizeof(struct heap));
	h->array = malloc(sizeof(int) * (len + 1));
	h->len = len;

	for (int i = 1; i <= len; i++) {
		int r = rand() % 9024;
		h->array[i] = r;
	}
	h->array[0] = -2;

	return h;
}

int main()
{
	int r;
	struct heap *h = init_heap(10);

	srand(arc4random());


	heapify(h);
	printf("Popping\n");
	int i;
	while ((i = pop(h)) >= 0) {
		printf("%d ", i);
	}
	printf("\n");
}
