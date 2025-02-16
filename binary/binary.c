#include <stdio.h>
#include <stdlib.h>
#include <strings.h>

struct node {
	int val;
	struct node *left, *right;
};

struct binary_tree {
	struct node *root;
};

static void insert(struct binary_tree *bt, int val)
{
	struct node *node = malloc(sizeof(struct node));
	node->val = val;
	node->left = NULL;
	node->right = NULL;

	if (bt->root == NULL) {
		bt->root = node;
		return;
	}

	struct node *cur;
	cur = bt->root;

	while (cur) {
		if (cur->val == val)
			return;

		if (val > cur->val && cur->right == NULL) {
			cur->right = node;
			break;
		} else if (val < cur->val && cur->left == NULL) {
			cur->left = node;
			break;
		}

		// Has both children
		if (val > cur->val)
			cur = cur->right;
		else if (val < cur->val)
			cur = cur->left;
	}
}

void print_node(struct node *node)
{
	if (!node)
		return;

	printf("%d\n", node->val);
	print_node(node->left);
	print_node(node->right);
}

void print_tree(struct binary_tree *bt) {
	struct node *cur;
	printf("----\n");
	print_node(bt->root);
}

int main()
{
	struct binary_tree bt = {};
	insert(&bt, 2);
	insert(&bt, 3);
	insert(&bt, 1);
	insert(&bt, 4);


	print_tree(&bt);
}
