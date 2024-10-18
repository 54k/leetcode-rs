#include <bits/stdc++.h>
using namespace std;
int main()
{
    vector<long long> v;
    long long x;
    while (cin >> x)
    {
        v.push_back(x);
    }
    long long ans = 0;
    for (int i = 1; i < v.size(); i++)
    {
        ans += abs(v[i] - v[i - 1]);
    }
    cout << ans << endl;
    return 0;
}