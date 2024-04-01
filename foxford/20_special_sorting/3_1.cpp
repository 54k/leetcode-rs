#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

vector<int> a;
long long ans = 0;

void sort(int l, int r)
{
    if (l == r - 1)
    {
        return;
    }
    int m = (l + r) / 2;
    sort(l, m);
    sort(m, r);

    vector<int> tmp(r - l);
    int it1 = l, it2 = m;
    while (it1 != m or it2 != r)
    {
        int ind = it1 + it2 - m - l;
        if (it1 == m)
        {
            tmp[ind] = a[it2];
            ++it2;
        }
        else if (it2 == r)
        {
            tmp[ind] = a[it1];
            ++it1;
        }
        else if (a[it1] <= a[it2])
        {
            tmp[ind] = a[it1];
            ++it1;
        }
        else
        {
            tmp[ind] = a[it2];
            ans += (m - it1);
            ++it2;
        }
    }
}

int main()
{
    ios_base::sync_with_stdio(false);
    cin.tie(0);
    int n;
    cin >> n;
    a.resize(n);
    for (int i = 0; i < n; i++)
    {
        cin >> a[i];
    }
    sort(0, n);
    cout << ans << endl;
    return 0;
}