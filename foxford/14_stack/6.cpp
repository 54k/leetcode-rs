#include <bits/stdc++.h>
using namespace std;

int main() {
    int N;
    cin >> N;

    deque<int> left;
    deque<int> right;

    while (N--) {
        string cmd;
        int arg;
        cin >> cmd;

        if (cmd == "+") {
            cin >> arg;
            left.push_front(arg);
        } else if (cmd == "*") {
            cin >> arg;
            left.push_back(arg);
        } else {
            cout << right.back() << "\n";
            right.pop_back(); 
        }

        if (left.size() > right.size()) {
            right.push_front(left.back());
            left.pop_back();
        }
    }

    return 0;
}