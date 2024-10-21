#include <iostream>
#include <vector>
using namespace std;

int main() {
	int n, ans = 0;
	cin >> n;
	vector<long long> v(n);
	for (auto & x : v) {
	    cin >> x;
	}
	for (int i = 0; i < v.size() - 1; i++)
	    for (int j = i + 1; j < v.size(); j++) {
	        if ((v[i] + v[j]) % 3 == 0) {
	            ans++;
	        }
	    }
	cout << ans << endl;
	return 0;
}