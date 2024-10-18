#include <bits/stdc++.h>
using namespace std;
int main()
{
    unordered_map<string, int> m;
    string x;
    while (cin >> x)
    {
        cout << m[x] << " ";
        m[x]++;
    }
    cout << endl;
    return 0;
}