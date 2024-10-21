#include <iostream>
#include <vector>
using namespace std;

int main() {
	int n, k;
	cin >> n >> k;
	vector<long long> v(n);
	for (auto & x : v) {
	    cin >> x;
	}
	
	int x;
	while (k-- > 0) {
	    cin >> x;
	    bool found = false;
	    int lo = 0;
	    int hi = v.size() - 1;
	    while (lo <= hi) {
	        int mid = (lo + hi) / 2;
	        if (v[mid] == x) {
	            found = true;
	            break;
	        } else if (v[mid] > x) {
	            hi = mid - 1;
	        } else {
	            lo = mid + 1;
	        }
	    }
	    if (found) {
	        cout << "YES\n";
	    } else {
	        cout << "NO\n";
	    }
	}
    cout << endl;
	return 0;
}