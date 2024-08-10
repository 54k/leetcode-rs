#include <iostream>
#include <cstdlib>

using namespace std;
const int INF = 2000000000;

struct node
{
    int data;
    int y;
    int size;
    int min;
    node *left;
    node *right;
    node(int val)
    {
        data = val;
        min = val;
        y = rand();
        size = 1;
        left = nullptr;
        right = nullptr;
    }
    ~node()
    {
        if (left != nullptr)
            delete[] left;
        if (right != nullptr)
            delete[] right;
    }
};

void print(node *root)
{
    if (root == nullptr)
        return;
    print(root->left);
    printf("%d ", root->data);
    print(root->right);
}

void update(node *root)
{
    if (root == nullptr)
        return;
    root->min = root->data;
    root->size = 1;
    if (root->left != nullptr)
    {
        root->min = min(root->min, root->left->min);
        root->size += root->left->size;
    }
    if (root->right != nullptr)
    {
        root->min = min(root->min, root->right->min);
        root->size += root->right->size;
    }
}

int get_min(node *root)
{
    return (root == nullptr) ? INF : root->min;
}

int get_size(node *root)
{
    return (root == nullptr) ? 0 : root->size;
}

pair<node *, node *> split(node *root, int k)
// Разбиваем дерево так, что в левом поддереве будет k вершин
{
    if (root == nullptr)
        return {nullptr, nullptr};
    if (get_size(root) <= k)
        return {root, nullptr};
    if (k == 0)
        return {nullptr, root};
    if (get_size(root->left) >= k)
    {
        auto res = split(root->left, k);
        root->left = res.second;
        update(root);
        return {res.first, root};
    }
    else
    {
        auto res = split(root->right, k - get_size(root->left) - 1);
        root->right = res.first;
        update(root);
        return {root, res.second};
    }
}

node *merge(node *root1, node *root2)
{
    if (root1 == nullptr)
        return root2;
    else if (root2 == nullptr)
        return root1;
    if (root1->y < root2->y)
    {
        root1->right = merge(root1->right, root2);
        update(root1);
        return root1;
    }
    else
    {
        root2->left = merge(root1, root2->left);
        update(root2);
        return root2;
    }
}

node *insert_after(node *root, int k, int val)
{
    node *new_node = new node(val);
    auto split1 = split(root, k);
    return merge(merge(split1.first, new_node), split1.second);
}

int min(node *root, int l, int r)
{
    if (root == nullptr)
        return INF;
    if (l <= 1 && r >= root->size)
        return root->min;

    int ans = INF;
    int s = get_size(root->left);
    if (l <= s + 1 && s + 1 <= r)
        ans = min(ans, root->data);

    if (l <= s)
        ans = min(ans, min(root->left, l, r));

    if (r >= s + 2)
        ans = min(ans, min(root->right, l - s - 1, r - s - 1));

    return ans;
}

int main()
{
    int n;
    cin >> n;
    node *root = nullptr;

    for (int i = 0; i < n; ++i)
    {
        char cmd;
        int l, r;
        cin >> cmd >> l >> r;
        if (cmd == '+')
        {
            root = insert_after(root, l, r);
            // cout << "\ntree is:\n";
            // print(root);
            // cout << "\n";
        }
        else if (cmd == '?')
        {
            int mn = min(root, l, r);
            // cout << "\nmn is:\n";
            cout << mn << endl;
            // cout << endl;
        }
    }
}