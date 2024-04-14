#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n;
    cin >> n;
    vector<pair<double, pair<int, int>>> v(n);
    for (auto &x : v)
    {
        string s;
        cin >> s;
        int i = s.find('/');
        x.second.first = stoi(s.substr(0, i));
        x.second.second = stoi(s.substr(i + 1));
        x.first = double(x.second.first) / x.second.second;
    }

    sort(v.begin(), v.end());

    for (auto &x : v)
    {
        cout << x.second.first << "/" << x.second.second << "\n";
    }

    cout << endl;
    return 0;
}