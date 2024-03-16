#include <bits/stdc++.h>
using namespace std;

int main() {
    int n;
    cin >> n;
    vector<int> v(n);
    for (auto & e: v) {
        cin >> e;
    }
    int ans = 0;
    int mx = 0;
    for (int i = n - 1; i >= 0; i--) {
        if (v[i] > mx) {
            ans++;
            mx = v[i];
        }
    }
    cout << ans << endl;
    return 0;
}