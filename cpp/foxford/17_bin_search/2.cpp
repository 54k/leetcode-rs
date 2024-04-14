#include <bits/stdc++.h>
using namespace std;

struct tzy {
    long long t,z,y;
};

int main() {
    long long m, n;
    cin >> m >> n;
    vector<tzy> vec(n);
    for (auto &t : vec) {
        cin >> t.t >> t.z >> t.y;
    }

    function<long long(long long)> f = [&](long long x) {
        long long tot = 0;
        for (auto &t : vec) {
            long long b = 0;
            long long tt = 0;
            while (tt < x) {
                tt += t.t;
                if (tt > x) {
                    continue;
                }
                if (b == t.z) {
                    tt += t.y;
                    b = 0;
                }
                if (tt > x) {
                    continue;
                }
                b++;
                tot++; 
            }
        };
        return tot;
    };

    long long lo = -1;
    long long hi = 1000*200;

    while (lo + 1 != hi) {
        long long mid = (lo + hi) / 2;

        if (f(mid) < m) {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    cout << hi << endl;
    return 0;
}