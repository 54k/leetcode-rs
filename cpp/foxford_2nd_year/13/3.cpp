#include <bits/stdc++.h>
using namespace std;

int main()
{
    vector<int> v;
    int x;
    while (cin >> x)
    {
        v.push_back(x);
    }

    int nxt = v[0], tmp;
    for (int i = v.size() - 1; i >= 0; i--)
    {
        tmp = nxt;
        nxt = v[i];
        v[i] = tmp;
    }
    for (auto &x : v)
    {
        cout << x << " ";
    }
    cout << endl;
    return 0;
}
