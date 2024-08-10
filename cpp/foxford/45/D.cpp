#include <cstdio>
#include <cstdlib>
#include <utility>
#include <iostream>
using namespace std;

const int INF = (int)1e9 + 7;

struct node
{
    int x, y, size, min;
    node *left, *right;
    bool rev;

    node(int val)
    {
        x = min = val;
        y = rand();
        size = 1;
        left = nullptr;
        right = nullptr;
    }

    ~node()
    {
        if (left)
            delete[] left;
        if (right)
            delete[] right;
    }
};

int get_size(node *root)
{
    if (root == nullptr)
        return 0;
    return root->size;
}

int get_min(node *root)
{
    if (root == nullptr)
    {
        return INF;
    }
    return root->min;
}

void apply_rev(node *root)
{
    if (root == nullptr)
        return;
    swap(root->left, root->right);
    root->rev ^= true;
}

void update(node *root)
{
    root->size = 1 + get_size(root->left) + get_size(root->right);
    root->min = min(min(get_min(root->left), get_min(root->right)), root->x);
}

void push(node *root)
{
    if (root == nullptr)
        return;

    if (root->rev)
    {
        apply_rev(root->left);
        apply_rev(root->right);
        root->rev = false;
    }
}

pair<node *, node *> split(node *root, int k)
{
    if (root == nullptr)
        return {nullptr, nullptr};

    if (get_size(root) <= k)
    {
        return {root, nullptr};
    }
    if (k == 0)
    {
        return {nullptr, root};
    }
    push(root);
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

node *merge(node *left, node *right)
{
    if (left == nullptr)
        return right;
    if (right == nullptr)
        return left;

    if (left->y < right->y)
    {

        push(left);
        left->right = merge(left->right, right);
        update(left);
        return left;
    }
    else
    {

        push(right);
        right->left = merge(left, right->left);
        update(right);
        return right;
    }
}

node *insert(node *root, int x)
{
    node *new_node = new node(x);
    return merge(root, new_node);
}

void print_tree(node *root)
{
    if (root == nullptr)
        return;
    print_tree(root->left);
    cout << root->x << " ";
    print_tree(root->right);
}

int main()
{
    int n, m;
    cin >> n >> m;
    node *root = nullptr;
    for (int i = 1; i <= n; i++)
    {
        int val;
        cin >> val;
        root = insert(root, val);
    }

    // cout << endl;
    // print_tree(root);

    while (m-- > 0)
    {
        int t, l, r;
        cin >> t >> l >> r;

        if (t == 1)
        {
            // rev
            auto split1 = split(root, r);
            auto split2 = split(split1.first, l - 1);
            apply_rev(split2.second);
            root = merge(merge(split2.first, split2.second), split1.second);

            // cout << endl;
            // cout << "tree is after rev: ";
            // print_tree(root);
            // cout << endl;
        }
        else if (t == 2)
        {
            // fin min
            auto split1 = split(root, r);
            auto split2 = split(split1.first, l - 1);
            int mn = get_min(split2.second);
            root = merge(merge(split2.first, split2.second), split1.second);

            // cout << "\n";
            // cout << "min is: " << mn << "\n";
            cout << mn << "\n";

            //     cout << endl;
            //     cout << "tree is after min: ";
            //     print_tree(root);
            //     cout << endl;
        }
    }
    cout << endl;
    return 0;
}