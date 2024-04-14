#include <bits/stdc++.h>
using namespace std;

int main() {
    int n;
    cin >> n;
    vector<int> v(n);
    for (auto & e : v) {
        cin >> e;
    }
    vector<bool> ans(n, false); 
    vector<int> stack;
    for (int i = 0; i < n; i++) {
        while (!stack.empty() and v[stack.back()] > v[i % n]) {
            v[stack.back()] = i % n;
            ans[stack.back()] = true;
            stack.pop_back();
        }
        stack.push_back(i % n);
    }
    int i = 0;
    for (auto &e: v) {
        if (!ans[i]) cout << -1 << " ";
        else cout << e << " ";
        i++;
    }
    cout << endl;
    return 0;
}