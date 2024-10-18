#include <bits/stdc++.h>
using namespace std;
int main()
{
    int x;
    unordered_set<int> s;
    while (cin >> x)
    {
        s.insert(x);
    }
    cout << s.size() << endl;
    return 0;
}