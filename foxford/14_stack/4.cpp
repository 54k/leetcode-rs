#include <bits/stdc++.h>
using namespace std;

int main() {
    int INF = 1<<30;
    int n;
    cin >> n;
    stack<pair<int, int> > s;
    while (n--) {
        string cmd;
        int arg;
        cin >> cmd;
        if (cmd == "push") {
            cin >> arg;
            s.push(make_pair(arg, min(arg, s.size() > 0 ? s.top().second : INF)));
        } else if (cmd == "pop") {
            s.pop();
        } else if (cmd == "min") {
            cout << s.top().second << endl;
        }
    }
    return 0;
}