#include <bits/stdc++.h>
using namespace std;

int main() {
    string str;
    getline(cin, str);
    vector<int> s;
    unordered_map<char, char> map;
    map['('] = ')';
    map['['] = ']';
    map['{'] = '}';
    for (auto ch : str) {
        if (map.find(ch) != map.end()) {
            s.push_back(map[ch]);
        } else if (!s.empty() && s.back() == ch) {
            s.pop_back();
        } else {
            cout << "NO" << endl;
            return 0;
        }
    }
    cout << (s.empty() ? "YES": "NO") << endl;
    return 0;
}