#include <iostream>
#include <vector>
#include <string>
#include <algorithm>

using namespace std;

bool cmp(const string &s1, const string &s2)
{
    return s1 + s2 > s2 + s1;
}

int main()
{
    vector<string> s;
    string s1;
    while (cin >> s1)
    {
        s.push_back(s1);
    }
    sort(s.begin(), s.end(), cmp);
    for (auto &c : s)
    {
        cout << c;
    }
    cout << endl;
    return 0;
}