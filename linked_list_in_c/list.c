#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <strings.h>

struct root {
	struct head *next;
};

struct head {
	struct head *next;
	int value;
};

void push(struct root *r, int val){
	struct head *next = r->next;
	r->next = malloc(sizeof(struct head));
	r->next->next = next;
	r->next->value = val;
}

void print_head(struct head *elem)
{
	while (elem != NULL) {
		printf("elem = %d\n", elem->value);
		elem = elem->next;
	}
}

void print(struct root *r)
{
	struct head *elem = r->next;
	print_head(elem);

}

void find_last_n(struct root *r, int n)
{
	struct head **buf = malloc(sizeof(struct head *)*n);
	int count = 0;

	struct head *elem = r->next;

	while (elem != NULL) {
		buf[count%n] = elem;
		elem = elem->next;
		count += 1;
	}

	int first = (count) % n;
	for (int i = 0; i < n; i++)
		printf("%d\n", buf[(first + i) % n]->value);
}

int main()
{
	struct root r = {};

	for (int i = 0 ; i < 10; i++) {
		push(&r, i);
	}

	print(&r);
	printf("====\n");

	find_last_n(&r, 3);
	/* print(&r); */
}
