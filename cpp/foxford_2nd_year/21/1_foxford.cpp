#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;
int main()
{
    int n;
    cin >> n;
    vector<int> a(n);
    for (int i = 0; i < n; ++i)
        cin >> a[i];
    sort(a.begin(), a.end());
    double y;
    cin >> y;
    int i = 0;
    while (i < n && a[i] < y)
        ++i;
    while (i < n)
    {
        y = (y + a[i]) / 2;
        ++i;
    }
    cout << y << endl;
    return 0;
}