#include <bits/stdc++.h>
using namespace std;

int main() {
    int e;
    vector<int> v;
    while (cin >> e) {
        v.push_back(e);
    }    

    int n = v.size();
    int t = 0, nxt = v[0];
    
    for (int i = n-1; i >= 0; i--) {
        t = nxt;
        nxt = v[i];
        v[i] = t;
    }
    
    for (auto e : v) {
        cout << e << " ";
    }
    cout << endl;
    return 0;
}