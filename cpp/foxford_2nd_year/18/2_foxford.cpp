#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;
int main()
{
    int n;
    cin >> n;
    vector<pair<int, int>> a(n);
    for (int i = 0; i < n; ++i)
    {
        cin >> a[i].first;
        a[i].second = i + 1;
    }
    sort(a.rbegin(), a.rend());
    long long sum = 0;
    for (long long i = 0; i < n; ++i)
        sum += (i + 1) * a[i].first;
    cout << sum << endl;
    for (int i = 0; i < n; ++i)
        cout << a[i].second << " ";
    cout << endl;
    return 0;
}