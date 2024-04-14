#include <bits/stdc++.h>
using namespace std;

int main() {
    set<string> ss;
    string s;
    while (cin >> s) {
        ss.insert(s);
    }
    cout << ss.size() << endl;
    return 0;
}