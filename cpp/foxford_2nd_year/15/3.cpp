#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

int main()
{
    int n;
    cin >> n;
    vector<int> v(n);
    for (auto &x : v)
    {
        cin >> x;
    }
    int ans = 0;
    for (int i = 0; i < n - 1; i++)
    {
        if (v[i] + 1 != v[i + 1])
        {
            ans++;
        }
    }
    cout << ans << endl;
    return 0;
}
