#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;
int main()
{
    int n;
    cin >> n;
    vector<pair<int, string>> a(n);
    for (int i = 0; i < n; ++i)
    {
        cin >> a[i].second;
        a[i].first = a[i].second.size();
    }
    sort(a.begin(), a.end());
    for (int i = 0; i < a.size(); ++i)
        cout << a[i].second << endl;
}