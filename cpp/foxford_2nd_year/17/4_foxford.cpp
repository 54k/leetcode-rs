#include<iostream>
#include<algorithm>
#include<vector>
using namespace std;
int main()
{
    int n, m, i;
    long long ans = 0;
    cin >> n >> m;
    vector<int> a(n);
    for (i = 0; i < n; ++i)
        cin >> a[i];
    sort(a.begin(), a.end());
    for (i = 0; i < n; ++i)
    {
        int j = upper_bound(a.begin(), a.end(), a[i] + m) - a.begin();
        ans += j - 1 - i;
    }
    cout << ans << endl;
    return 0;
}