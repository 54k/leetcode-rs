#include <iostream>
#include <vector>
using namespace std;

int main()
{
    int n, ans = 0;
    cin >> n;
    vector<int> v(n);
    for (auto &x : v)
    {
        cin >> x;
    }
    vector<int> d(n, n);
    for (int i = 0; i < v.size(); i++)
    {
        if (v[i] == 2)
        {
            d[i] = 0;
        }
        else if (i > 0)
        {
            d[i] = d[i - 1] + 1;
        }
    }
    for (int i = v.size() - 1; i >= 0; i--)
    {
        if (v[i] == 2)
        {
            d[i] = 0;
        }
        else if (i < v.size() - 1)
        {
            d[i] = min(d[i], d[i + 1] + 1);
        }
        if (v[i] == 1)
        {
            ans = max(ans, d[i]);
        }
    }
    cout << ans << endl;
    return 0;
}