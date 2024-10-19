#include <iostream>
using namespace std;
int main()
{
    string s;
    cin >> s;
    int balance = 0, min_balance = 0;
    for (auto c : s)
    {
        if (c == '(')
            ++balance;
        else
        {
            --balance;
            min_balance = min(min_balance, balance);
        }
    }
    if (balance == 0)
        cout << (-min_balance + 1) / 2 << endl;
    else
        cout << -1 << endl;
}