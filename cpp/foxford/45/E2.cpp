#define _CRT_SECURE_NO_WARNINGS
#define _USE_MATH_DEFINES
#include <iostream>
#include <algorithm>
#include <vector>
#include <numeric>
#include <cmath>
#include <set>
#include <unordered_set>
#include <string>
#include <map>
#include <unordered_map>
#include <queue>
using namespace std;
#define all(a) a.begin(), a.end()
typedef long long ll;
typedef unsigned long long ull;
typedef long double ld;
inline int superrand()
{
    return (rand() << 15) + rand();
}
const int INF = 2e9;
struct Node
{
    int val, prior, minn, sz;
    Node *l, *r;
    Node() : l(nullptr), r(nullptr), sz(0), minn(INF) {};
    Node(int val) : val(val), prior(superrand()), minn(val), sz(1), r(nullptr), l(nullptr) {}
};
Node *tmp1, *tmp2, *tmp3;
const int SIZE = 2e5 + 134;
int sz = 0;
Node nodes[SIZE];
inline Node *getNode(int val)
{
    nodes[sz] = Node(val);
    return &nodes[sz++];
}
inline int get_sz(Node *root)
{
    return (root ? root->sz : 0);
}
inline int get_minn(Node *root)
{
    return (root ? root->minn : INF);
}
inline void update(Node *root)
{
    if (!root)
        return;
    root->sz = get_sz(root->l) + get_sz(root->r) + 1;
    root->minn = min({get_minn(root->l), get_minn(root->r), root->val});
}
void split(Node *root, Node *&l, Node *&r, int k)
{
    if (!root)
    {
        l = r = nullptr;
        return;
    }
    if (get_sz(root->l) >= k)
    {
        split(root->l, l, root->l, k);
        r = root;
        update(r);
    }
    else
    {
        split(root->r, root->r, r, k - get_sz(root->l) - 1);
        l = root;
        update(l);
    }
}
void merge(Node *&root, Node *l, Node *r)
{
    if (!l || !r)
    {
        root = (l ? l : r);
        return;
    }
    if (l->prior > r->prior)
    {
        merge(l->r, l->r, r);
        root = l;
    }
    else
    {
        merge(r->l, l, r->l);
        root = r;
    }
    update(root);
}
void insert(Node *&root, int pos, int val)
{
    Node *ins = getNode(val);
    split(root, tmp1, tmp2, pos);
    merge(root, tmp1, ins);
    merge(root, root, tmp2);
}
void solve()
{
    int n;
    cin >> n;
    Node *root = nullptr;
    for (int i = 0; i < n; ++i)
    {
        char t;
        cin >> t;
        if (t == '+')
        {
            int pos, val;
            cin >> pos >> val;
            insert(root, pos, val);
        }
        else
        {
            int l, r;
            cin >> l >> r;
            --l, --r;
            split(root, tmp1, tmp3, r + 1);
            split(tmp1, tmp1, tmp2, l);
            cout << get_minn(tmp2) << '\n';
            merge(root, tmp1, tmp2);
            merge(root, root, tmp3);
        }
    }
}
int main()
{
    ios_base::sync_with_stdio(false);
    cin.tie(0);
    cout.tie(0);
    cout.precision(40);
    int t = 1;
    // cin >> t;
    while (t--)
    {
        solve();
    }
}