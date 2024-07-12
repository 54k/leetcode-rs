#include <iostream>
#include <vector>
using namespace std;
struct node
{
    int left, right, max, max_idx;
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
        res->max = values[left];
        res->max_idx = left;
    }
    else
    {
        int mid = (left + right) / 2;
        res->child_left = build(left, mid, values);
        res->child_right = build(mid + 1, right, values);
        if (res->child_left->max >= res->child_right->max)
        {
            res->max = res->child_left->max;
            res->max_idx = res->child_left->max_idx;
        }
        else
        {
            res->max = res->child_right->max;
            res->max_idx = res->child_right->max_idx;
        }
    }
    return res;
}
pair<int, int> query(node *root, int left, int right)
{
    if (right < root->left || left > root->right)
        return make_pair(-INF, 0);
    if (left <= root->left && root->right <= right)
        return make_pair(root->max, root->max_idx);
    pair<int, int> ans1 = query(root->child_left, left, right);
    pair<int, int> ans2 = query(root->child_right, left, right);
    if (ans1.first >= ans2.first)
        return ans1;
    else
        return ans2;
}
int main()
{
    int n;
    cin >> n;
    vector<int> a(n + 1);
    for (int i = 1; i <= n; ++i)
        cin >> a[i];
    node *root = build(1, n, a);
    int m;
    cin >> m;
    for (int i = 0; i < m; ++i)
    {
        int l, r;
        cin >> l >> r;
        pair<int, int> ans = query(root, l, r);
        cout << ans.first << " " << ans.second << endl;
    }
    return 0;
}