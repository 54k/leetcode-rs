#include <bits/stdc++.h>
using namespace std;

int main() {
    int n, k;
    cin >> n >> k;
    vector<int> v(n);

    for (auto & e: v) {
        cin >> e;
    }

    while (k-- > 0) {
        int x;
        cin >> x;
        int lo = -1;
        int hi = n;
        while (lo + 1 != hi) {
            int mid = (lo + hi) / 2;
            if (v[mid] < x) {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        if (hi == n) {
            cout << v[lo] << "\n";
        } else if (lo == -1) {
            cout << v[hi] << "\n";
        } else {
            cout << (abs(x - v[lo]) <= abs(x - v[hi]) ? v[lo] : v[hi]) << "\n";
        }
    }
    cout << endl;
    return 0;
}