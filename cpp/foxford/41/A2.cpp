#include <bits/stdc++.h>
using namespace std;

struct node
{
    int left, right, min, max;
    node *child_left, *child_right;
};
const int INF = 2000000000;
node *build(int left, int right, vector<int> &values)
{
    if (left > right)
        return 0;
    node *res = new node;
    res->left = left;
    res->right = right;
    if (right == left)
    {
        res->child_left = res->child_right = 0;
        res->min = values[left];
        res->max = values[left];
    }
    else
    {
        int mid = (left + right) / 2;
        res->child_left = build(left, mid, values);
        res->child_right = build(mid + 1, right, values);
        res->min = min(res->child_left->min, res->child_right->min);
        res->max = max(res->child_left->max, res->child_right->max);
    }
    return res;
}
int query_min(node *root, int left, int right)
{
    if (right < root->left || left > root->right)
        return INF;
    if (left <= root->left && right >= root->right)
        return root->min;
    int ans1 = query_min(root->child_left, left, right);
    int ans2 = query_min(root->child_right, left, right);
    return min(ans1, ans2);
}
int query_max(node *root, int left, int right)
{
    if (right < root->left || left > root->right)
        return -INF;
    if (left <= root->left && right >= root->right)
        return root->max;
    int ans1 = query_max(root->child_left, left, right);
    int ans2 = query_max(root->child_right, left, right);
    return max(ans1, ans2);
}
int query(node *root, int left, int right)
{
    return query_max(root, left, right) - query_min(root, left, right);
}
void update(node *root, int idx, int val)
{
    if (idx < root->left || idx > root->right)
        return;
    if (root->left == root->right)
    {
        root->min = root->max = val;
        return;
    }
    update(root->child_left, idx, val);
    update(root->child_right, idx, val);
    root->min = min(root->child_left->min, root->child_right->min);
    root->max = max(root->child_left->max, root->child_right->max);
}
int main()
{
    int n = 100000, k;
    cin >> k;
    vector<int> a(n + 1);
    for (int i = 1; i <= n; ++i)
        a[i] = ((long long)i * i % 12345 + (long long)i * i * i % 23456);
    node *root = build(1, n, a);
    for (int i = 0; i < k; ++i)
    {
        int x, y;
        cin >> x >> y;
        if (x > 0)
            cout << query(root, x, y) << endl;
        else
            update(root, -x, y);
    }
    return 0;
}