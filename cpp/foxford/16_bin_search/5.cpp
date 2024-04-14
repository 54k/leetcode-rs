#include <bits/stdc++.h>

using namespace std;

void search(vector<int> & v) {
    int t;
    cin >> t;

    int lo = 0;
    int hi = v.size()-1;
    while (lo <= hi) {
        int mid = (lo + hi) >> 1;

        if (v[mid] == t) {
            cout << "YES" << endl;
            return;
        } else if (v[mid] < t) {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    cout << "NO" << endl;
}

int main() {
    int n, m;
    cin >> n >> m;
    vector<int> v(n);
    for (auto & e: v) {
        cin >> e;
    }

    outerLoop: 
    while (m-- > 0) {
        search(v);
    }
    return 0;
}