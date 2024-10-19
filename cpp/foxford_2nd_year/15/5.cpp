#include <iostream>
#include <string>
using namespace std;

int main()
{
    string s;
    cin >> s;
    int bal = 0;
    int total = 0;
    for (int i = 0; i < s.size(); i++)
    {
        if (s[i] == '(')
        {
            bal++;
            total++;
        }
        else
        {
            if (bal > 0)
            {
                bal--;
            }
            total--;
        }
    }
    if (total != 0)
    {
        cout << -1 << endl;
    }
    else
    {
        cout << (bal + 1) / 2 << endl;
    }
    return 0;
}