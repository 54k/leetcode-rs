#include <iostream>
using namespace std;
int main()
{
    string s;
    int k, x = 0, y = 0;
    while (cin >> s)
    {
        cin >> k;
        if (s == "East")
        {
            x += k;
        }
        else if (s == "West")
        {
            x -= k;
        }
        else if (s == "North")
        {
            y += k;
        }
        else
        {
            y -= k;
        }
    }
    cout << x << " " << y;
    return 0;
}