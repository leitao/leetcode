#include <stdio.h>
#include <unistd.h>
#include <stdbool.h>
#include <strings.h>
#include <stdlib.h>

struct node {
	int val;
	struct node *next;
};

struct root {
	struct node *root;
};

static void insert(struct root *root, int val)
{
	struct node *node = malloc(sizeof(struct node));

	node->next = NULL;
	node->val = val;
	node->next = root->root;
	root->root = node;
}

static void print_list(struct root *root)
{
	struct node *element;

	element = root->root;
	int i = 0;

	while (element) {
		printf("%d -> ", element->val);
		element = element->next;
		if (i++ > 20)
			return;
	}
	printf("\n");
}

struct root *merge_two_list(struct root *_l1, struct root *_r1)
{
	struct node *r1, *l1;
	struct root *new_root;
	struct node *last = NULL;
	struct node *next = NULL;

	new_root = malloc(sizeof(struct root));
	new_root->root = NULL;

	r1 = _r1->root;
	l1 = _l1->root;

	while (r1 != NULL || l1 != NULL) {
		if (l1 == NULL && r1 != NULL){
			next = r1;
			r1 = r1->next;
		} else if (r1 == NULL && l1 != NULL) {
			next = l1;
			l1 = l1->next;
		} else if (r1 && l1 && r1->val >= l1->val) {
			next = r1;
			r1 = r1->next;
		} else if (r1 && l1 && r1->val <= l1->val) {
			next = l1;
			l1 = l1->next;
		} else {
			printf("r1 %p\n", r1);
			printf("l1 %p\n", l1);
			printf("BROKEN\n");
			/* print_list(new_root); */
			exit(1);
		}

		if (!new_root->root) {
			new_root->root = next;
		} else {
			last->next = next;
		}
		next->next = NULL;
		last = next;
	}

	return new_root;
}

int main()
{
	struct root l1 = {}, l2 = {};

	for (int i = 0; i < 10; i += 2)
		insert(&l1, i);
	for (int i = 1; i < 10; i += 2)
		insert(&l2, i);


	print_list(&l1);
	print_list(&l2);

	struct root *n = merge_two_list(&l1, &l2);
	print_list(n);
	struct root *m = merge_two_list(&l2, &l1);
	print_list(m);
}
