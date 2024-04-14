#include <bits/stdc++.h>
using namespace std;

int main() {
    unordered_set<int> s;
    int e;
    while (cin >> e) {
        s.insert(e);
    }

    cout << s.size() << endl;
    return 0;
}