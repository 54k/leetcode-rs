#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n, y;
    cin >> n;
    vector<int> v(n);
    for (auto &x : v)
    {
        cin >> x;
    }
    cin >> y;
    sort(v.begin(), v.end());
    double res = double(y);

    for (int i = 0; i < n; i++)
    {
        if (v[i] > res)
        {
            res += (double)v[i];
            res /= 2.0;
        }
    }
    // cout << fixed << setprecision(1) << res << endl;
    cout << res << endl;
    return 0;
}