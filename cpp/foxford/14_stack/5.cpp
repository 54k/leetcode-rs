#include <bits/stdc++.h>
using namespace std;

int main() {
    string str;
    getline(cin, str);
    vector<int> s;
    int ans = 0;

    for (int i = 0; i < str.size(); i+=2) {
        int c = str[i] - '0';
        s.push_back(c);

        while (s.size() >= 3 and s[s.size()-1] == s[s.size()-2] and s[s.size()-2] == s[s.size()-3]) {
            while (i + 2 < str.size() and str[i+2]-'0' == s[s.size()-1]) {
                i+=2;
                ans++;
            }
            while (s[s.size()-1] == c) {
                s.pop_back();
                ans++;
            }
        }
    }

    cout << ans << endl;
    return 0;
}