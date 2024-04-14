#include <bits/stdc++.h>
using namespace std;

int main() {
    int ans = 0, prev = -1, e;
    while (cin >> e) {
        if (prev == -1) prev = e;
        ans += abs(prev - e);
        prev = e;
    }
    cout << ans << endl;
    return 0;
}