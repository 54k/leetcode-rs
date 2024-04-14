#include <bits/stdc++.h>
using namespace std;

int main() {
    int n, z = 0, o = 0, t = 0;
    cin>>n;
    
    while (cin >> n) {
        if (n % 3 == 0)  {
            z++;
        } else if (n % 3 == 1) {
            o++;
        } else {
            t++;
        }
    }

    long long ans = 0;
    ans += z * 1ll * (z - 1) / 2;
    ans += o * 1ll * t;
    cout << ans << endl;
    return 0;
}