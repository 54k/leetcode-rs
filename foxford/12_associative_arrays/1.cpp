#include <bits/stdc++.h>
using namespace std;

int main()
{
    char ch;
    cin >> ch;
    if (islower(ch))
    {
        cout << (char)toupper(ch) << endl;
    }
    else
    {

        cout << (char)tolower(ch) << endl;
    }
    return 0;
}

// #2
void solve()
{
    char c;
    cin >> c;
    if (c >= 'A' and c <= 'Z')
    {
        cout << (c = c - 'A' + 'a');
    }
    else if (c >= 'a' and c <= 'z')
    {
        cout << (c = c - 'a' + 'A');
    }
    else
    {
        cout << c;
    }
}