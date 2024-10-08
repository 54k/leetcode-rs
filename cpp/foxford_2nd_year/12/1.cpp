#include <iostream>
using namespace std;

int main()
{
    char c;
    cin >> c;
    if (c >= 'a' && c <= 'z')
        cout << char(c - 'a' + 'A') << endl;
    else if (c >= 'A' && c <= 'Z')
        cout << char(c - 'A' + 'a') << endl;
    else
        cout << c << endl;
    return 0;
}