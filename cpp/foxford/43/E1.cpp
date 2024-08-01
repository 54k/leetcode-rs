#include <iostream>
#include <random>
using namespace std;
typedef long long ll;
#define pnn pair<Node *, Node *>
const ll mod = 1e9;
// 112913 113219
struct Node
{
    ll key, prior, sz, sum;
    Node *l = nullptr;
    Node *r = nullptr;
    Node() {}
    Node(ll key) : key(key), prior((rand() << 15) | rand()), sz(1), sum(key) {}
};
ll sm(Node *v) { return v ? v->sum : 0; }
void upd_sm(Node *v) { v->sum = sm(v->l) + sm(v->r) + v->key; }
ll size(Node *v) { return v ? v->sz : 0; }
void update(Node *v) { v->sz = size(v->r) + size(v->l) + 1; }
Node *merge(Node *a, Node *b)
{
    if (!a)
        return b;
    if (!b)
        return a;
    if (a->prior > b->prior)
    {
        a->r = merge(a->r, b);
        update(a);
        upd_sm(a);
        return a;
    }
    else
    {
        b->l = merge(a, b->l);
        update(b);
        upd_sm(b);
        return b;
    }
}
pnn split(Node *v, ll x)
{
    if (!v)
        return {0, 0};
    if (v->key < x)
    {
        pnn A = split(v->r, x);
        v->r = A.first;
        update(v);
        upd_sm(v);
        return {v, A.second};
    }
    else
    {
        pnn A = split(v->l, x);
        v->l = A.second;
        update(v);
        upd_sm(v);
        return {A.first, v};
    }
}
bool find(Node *v, ll x)
{
    if (!v)
        return false;
    else if (v->key < x)
        return find(v->r, x);
    else if (v->key > x)
        return find(v->l, x);
    else
        return true;
}
int main()
{
    cin.tie(0)->sync_with_stdio(0);
    ll n, buffer = 0;
    Node *root = nullptr;
    bool flag = 0;
    cin >> n;
    while (n--)
    {
        char s;
        cin >> s;
        if (s == '+')
        {
            ll x;
            cin >> x;
            if (flag)
            {
                x = (x + buffer) % mod;
                flag = false;
            }
            if (find(root, x))
                continue;
            pnn A = split(root, x);
            root = merge(A.first, merge(new Node(x), A.second));
        }
        else
        {
            ll u, v;
            cin >> u >> v;
            pnn A = split(root, u);
            pnn B = split(A.second, v + 1);
            ll ans = sm(B.first);
            cout << ans << "\n";
            root = merge(A.first, merge(B.first, B.second));
            buffer = ans;
            flag = true;
        }
    }
    return 0;
}