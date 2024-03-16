#include <bits/stdc++.h>
using namespace std;

int main() {
    string s;
    unordered_map<string, int> ss;
    while (cin >> s) {
        cout << ss[s] << " ";
        ss[s]++;
    }
    cout << endl;
    return 0;
}