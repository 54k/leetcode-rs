#include <bits/stdc++.h>

using namespace std;

int main()
{
    vector<pair<int, string>> v;
    int n;
    cin >> n;
    v.resize(n);

    for (auto &e : v)
    {
        cin >> e.second;
        e.first = e.second.size();
    }

    sort(v.begin(), v.end(),
         [&](const pair<int, string> &a, const pair<int, string> &b)
         {
             return a.first == b.first ? a < b : a.first < b.first;
         });

    for (const auto &p : v)
    {
        cout << p.second << endl;
    }

    return 0;
}