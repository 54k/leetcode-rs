#include <bits/stdc++.h>
using namespace std;

int main()
{
    vector<string> v;
    string s;
    int i = 0;
    while (cin >> s)
    {
        v.push_back(s);
    }

    for (int j = v.size() - 1; j >= 0; j--)
    {
        for (int i = 0; i < j; i++)
        {
            if (v[i + 1] + v[i] > v[i] + v[i + 1])
            {
                swap(v[i], v[i + 1]);
            }
        }
    }
    for (int j = 0; j < v.size(); j++)
    {
        cout << v[j];
    }
    return 0;
}