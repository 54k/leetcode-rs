#include <bits/stdc++.h>
using namespace std;

int main()
{
    int x = 0, y = 0;
    string s;
    int n;
    while (cin >> s)
    {
        cin >> n;

        if ("North" == s or "South" == s)
        {
            y += "North" == s ? n : -n;
        }
        else if ("East" == s)
        {
            x += n;
        }
        else
        {
            x -= n;
        }
    }
    cout << x << " " << y << endl;
    return 0;
}