#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;
int main()
{
    int n;
    cin >> n;
    vector<pair<int, int>> dist(n);
    vector<pair<int, int>> price(n);
    for (int i = 0; i < n; ++i)
    {
        cin >> dist[i].first;
        dist[i].second = i + 1;
    }
    for (int i = 0; i < n; ++i)
    {
        cin >> price[i].first;
        price[i].second = i + 1;
    }
    sort(dist.begin(), dist.end());
    sort(price.rbegin(), price.rend());
    vector<int> ans(n);
    long long s = 0;
    for (int i = 0; i < n; ++i)
    {
        s += (long long)dist[i].first * price[i].first;
        ans[dist[i].second - 1] = price[i].second;
    }
    cout << s << endl;
    for (int i = 0; i < ans.size(); ++i)
        cout << ans[i] << " ";
    cout << endl;
}