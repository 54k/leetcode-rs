#include <iostream>
using namespace std;

int main() {
	int _n, x, ans = 0;
	cin >> _n;
	while (cin >> x) {
	    if (x == 1) {
	        ans++;
	    }
	}
	cout << ans << endl;
	return 0;
}