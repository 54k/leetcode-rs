#include <bits/stdc++.h>
using namespace std;

struct Point
{
    int x, y;
};

bool cmp(const Point a, const Point b)
{
    return a.x * a.x + a.y * a.y < b.x * b.x + b.y * b.y;
}

int main()
{
    int n;
    cin >> n;
    vector<Point> v(n);
    for (auto &x : v)
    {
        cin >> x.x >> x.y;
    }
    sort(v.begin(), v.end(), cmp);
    for (auto &x : v)
    {
        cout << x.x << " " << x.y << "\n";
    }
    cout << endl;
    return 0;
}