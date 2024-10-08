#include <iostream>
using namespace std;

int main()
{
    long long n, a = 0, b = 0, ans = 0;
    cin >> n;
    while (n-- > 0)
    {
        long long num;
        cin >> num;
        if (num % 3 == 0)
        {
            ans += a + b;
            a++;
        }
        else
        {
            b++;
            ans += a;
        }
    }
    cout << ans << endl;
    return 0;
}