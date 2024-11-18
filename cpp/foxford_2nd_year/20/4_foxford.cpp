#include <vector>
#include <iostream>
#include <algorithm>

using namespace std;

int main()
{
    int n, m;
    cin >> n;
    vector<int> a(n);
    for (int i = 0; i < n; ++i)
    {
        cin >> a[i];
    }
    cin >> m;
    vector<int> b(m);
    for (int i = 0; i < m; ++i)
    {
        cin >> b[i];
    }

    sort(a.begin(), a.end());
    sort(b.begin(), b.end());

    a.resize(unique(a.begin(), a.end()) - a.begin());
    b.resize(unique(b.begin(), b.end()) - b.begin());

    cout << (a == b ? "YES" : "NO");
}