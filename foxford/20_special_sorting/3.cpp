#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n;
    cin >> n;
    vector<int> v(n);
    for (auto &e : v)
    {
        cin >> e;
    }
    long long ans = 0;
    for (int i = 0; i < n; i++)
        for (int j = i + 1; j < n; j++)
            if (v[i] > v[j])
            {
                ans++;
            }
    cout << ans << endl;
    return 0;
}