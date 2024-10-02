#include <iostream>
#include <vector>
using namespace std;

int main() {
    int n;
    cin >> n;
    vector<int> v(n);
    for (auto &x : v) {
        cin >> x;
    }
    int ans = 0;
    for (int i = 1; i < v.size()-1; i++) {
        if (v[i-1] < v[i] && v[i+1] < v[i]) {
            ans++;
        }
    }
	cout<<ans<<endl;
	return 0;
}