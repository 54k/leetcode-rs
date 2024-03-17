#include <bits/stdc++.h>
using namespace std;
int main() {
    string str;
    cin >> str;
    vector<char> s;

    for (auto ch : str) {
        while (s.size() >= 2 and s[s.size() - 1] == s[s.size() - 2]) {
            s.pop_back();
            s.pop_back();
        }
        s.push_back(ch);
    }

    for (auto ch : s) {
        cout<<ch;
    }

    cout << endl;
    return 0;
}