#include <iostream>
#include <vector>
using namespace std;

int main()
{
    int ans = (1 << 30), n, m;
    cin >> n >> m;
    vector<int> a(n);
    vector<int> b(m);
    for (auto &x : a)
    {
        cin >> x;
    }
    for (auto &x : b)
    {
        cin >> x;
    }

    int i = 0, j = 0;
    while (i < n && j < m)
    {
        while (i < n && a[i] <= b[j])
        {
            ans = min(ans, b[j] - a[i]);
            i++;
        }
        while (j < m && b[j] <= a[i])
        {
            ans = min(ans, a[i] - b[j]);
            j++;
        }
    }
    cout << ans << endl;
    return 0;
}