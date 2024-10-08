#include <cstdio>
#include <cstdlib>
#include <utility>
using namespace std;
const int INF = 2000000000;
struct node
{
    int x;
    int y;
    int size;
    node *left;
    node *right;
    node(int val)
    {
        x = val;
        y = rand();
        size = 1;
        left = 0;
        right = 0;
    }
    ~node()
    {
        if (left)
            delete[] left;
        if (right)
            delete[] right;
    }
};
void update_size(node *root)
{
    if (!root)
        return;
    root->size = 1;
    if (root->left)
        root->size += root->left->size;
    if (root->right)
        root->size += root->right->size;
}
int get_size(node *root)
{
    return (!root) ? 0 : root->size;
}
pair<node *, node *> split(node *root, int val)
{
    if (!root)
        return make_pair((node *)0, (node *)0);
    if (root->x < val)
    {
        pair<node *, node *> res = split(root->right, val);
        root->right = res.first;
        update_size(root);
        return make_pair(root, res.second);
    }
    else
    {
        pair<node *, node *> res = split(root->left, val);
        root->left = res.second;
        update_size(root);
        return make_pair(res.first, root);
    }
}
node *merge(node *root1, node *root2)
{
    if (!root1)
        return root2;
    else if (!root2)
        return root1;
    if (root1->y < root2->y)
    {
        root1->right = merge(root1->right, root2);
        update_size(root1);
        return root1;
    }
    else
    {
        root2->left = merge(root1, root2->left);
        update_size(root2);
        return root2;
    }
}
int exists(node *root, int val)
{
    if (!root)
        return -INF;
    else if (root->x < val)
        return 1 + get_size(root->left) + exists(root->right, val);
    else if (root->x > val)
        return exists(root->left, val);
    else
        return 1 + get_size(root->left);
}
node *insert(node *root, int val)
{
    if (exists(root, val) > 0)
        return root;
    pair<node *, node *> subtree = split(root, val);
    node *newnode = new node(val);
    return merge(merge(subtree.first, newnode), subtree.second);
}
node *remove(node *root, int val)
{
    if (!exists(root, val) > 0)
        return root;
    pair<node *, node *> subtree1 = split(root, val);
    pair<node *, node *> subtree2 = split(subtree1.second, val + 1);
    return merge(subtree1.first, subtree2.second);
}
int main()
{
    char action[10];
    int d;
    node *root = 0;
    int n;
    scanf("%d", &n);
    for (int i = 0; i < n; ++i)
    {
        scanf("%s%d", action, &d);
        int res;
        switch (action[0])
        {
        case 'i':
            root = insert(root, d);
            continue;
        case 'd':
            root = remove(root, d);
            continue;
        case 'e':
            int res = exists(root, d);
            if (res < 0)
                res = 0;
            printf("%d\n", res);
            continue;
        }
    }
}