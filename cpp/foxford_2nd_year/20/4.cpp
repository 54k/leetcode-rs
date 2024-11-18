#include <bits/stdc++.h>
using namespace std;

int n, m;
vector<int> a;
vector<int> b;

int main()
{
    cin >> n;
    a.resize(n);
    for (auto &i : a)
    {
        cin >> i;
    }

    cin >> m;
    b.resize(m);
    for (auto &i : b)
    {
        cin >> i;
    }

    sort(a.begin(), a.end());
    sort(b.begin(), b.end());

    int i = 0, j = 0;
    while (i < a.size() && j < b.size())
    {
        if (a[i] == b[j])
        {
            int v = a[i];
            while (a[i] == v)
            {
                i++;
            }
            while (b[j] == v)
            {
                j++;
            }
        }
        else
        {
            break;
        }
    }
    cout << (i == a.size() and j == b.size() ? "YES" : "NO") << endl;
    return 0;
}