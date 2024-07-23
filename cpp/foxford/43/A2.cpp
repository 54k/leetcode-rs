#include <cstdio>
#include <cstdlib>
#include <utility>
using namespace std;
const int INF = 2000000000;
struct node
{
    int x;
    int y;
    node *left;
    node *right;
    node(int val)
    {
        x = val;
        y = rand();
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
pair<node *, node *> split(node *root, int val)
{
    if (!root)
        return make_pair((node *)0, (node *)0);
    if (root->x < val)
    {
        pair<node *, node *> res = split(root->right, val);
        root->right = res.first;
        return make_pair(root, res.second);
    }
    else
    {
        pair<node *, node *> res = split(root->left, val);
        root->left = res.second;
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
        return root1;
    }
    else
    {
        root2->left = merge(root1, root2->left);
        return root2;
    }
}
bool exists(node *root, int val)
{
    if (!root)
        return false;
    else if (root->x < val)
        return exists(root->right, val);
    else if (root->x > val)
        return exists(root->left, val);
    else
        return true;
}
int prev(node *root, int val, int result = -INF)
{
    if (!root)
        return result;
    if (result < root->x && root->x < val)
        result = root->x;
    if (root->x <= val)
        return prev(root->right, val, result);
    else
        return prev(root->left, val, result);
}
int next(node *root, int val, int result = INF)
{
    if (!root)
        return result;
    if (val < root->x && root->x < result)
        result = root->x;
    if (root->x >= val)
        return next(root->left, val, result);
    else
        return next(root->right, val, result);
}
node *insert(node *root, int val)
{
    if (exists(root, val))
        return root;
    pair<node *, node *> subtree = split(root, val);
    node *newnode = new node(val);
    return merge(merge(subtree.first, newnode), subtree.second);
}
node *remove(node *root, int val)
{
    if (!exists(root, val))
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
            printf("%s\n", exists(root, d) ? "true" : "false");
            continue;
        }
    }
}