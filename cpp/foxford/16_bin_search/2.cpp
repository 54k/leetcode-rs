#include <bits/stdc++.h>
using namespace std;

int main() {
    int n, even = 0, odd = 0;
    cin>>n;
    while (cin >> n) {
        if (n % 2 == 0)  {
            even++;
        } else {
            odd++;
        }
    }
    long long ans = ((even * 1ll - 1) * even + (odd * 1ll - 1) * odd) / 2;
    cout << ans << endl;
    return 0;
}