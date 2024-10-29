#include <iostream>
#include <algorithm>
#include <vector>
using namespace std;
int main()
{
    int n;
    char d;
    cin >> n;
    vector<pair<double, pair<int, int>>> a(n);
    for (int i = 0; i < n; ++i)
    {
        cin >> a[i].second.first >> d >> a[i].second.second;
        a[i].first = (double)a[i].second.first / a[i].second.second;
    }
    sort(a.begin(), a.end());
    for (int i = 0; i < n; ++i)
        cout << a[i].second.first << "/" << a[i].second.second << endl;
    return 0;
}