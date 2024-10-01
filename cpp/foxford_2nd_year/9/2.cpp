#include <iostream>

using namespace std;

int extended_gcd(int a, int b, int &x, int &y)
{
    if (a == 0)
    {
        x = 0;
        y = 1;
        return b;
    }
    int x1, y1;
    int d = extended_gcd(b % a, a, x1, y1);
    x = y1 - (b / a) * x1;
    y = x1;
    return d;
}

int main()
{
    int a, b, c;
    cin >> a >> b >> c;
    int x, y;
    int g = extended_gcd(a, b, x, y);
    if (c % g != 0)
    {
        cout << "Impossible" << endl;
    }
    else
    {
        cout << g << " " << x * (c / g) << " " << y * (c / g) << endl;
    }
}