#include <iostream>
using namespace std;
int main()
{
    int n, i;
    long long ans = 0;
    cin >> n;
    for (i = 0; i <= n; i += 5)
        ans += (long long)(1 + i / 10) * (1 + (n - i) / 2);
    cout << ans << endl;
    return 0;
}