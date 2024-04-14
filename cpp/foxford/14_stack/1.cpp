#include <bits/stdc++.h>
using namespace std;

int main() {
    string str;
    getline(cin, str);
    vector<int> s;
    int num = 0;
    for (int i = 0; i < str.size(); i++) {
        char c = str[i];
        int f, sec;
        switch (c) {
            case '*':
                sec = s.back();
                s.pop_back();
                f = s.back();
                s.pop_back();
                s.push_back(f * sec);
            break;
            case '+':
                sec = s.back();
                s.pop_back();
                f = s.back();
                s.pop_back();
                s.push_back(f + sec);
            break;
            case '-':
                sec = s.back();
                s.pop_back();
                f = s.back();
                s.pop_back();
                s.push_back(f - sec);
            break;
            case '/':
                sec = s.back();
                s.pop_back();
                f = s.back();
                s.pop_back();
                s.push_back(f / sec);
            break;
            case ' ':
                continue;
            break;
            default:
                while (i < str.size() && str[i] != ' ') { 
                    num = num * 10 + str[i] - '0';
                    i++;
                }
                s.push_back(num);
                num = 0;
            break;
        }
    }

    cout << s.back() << endl;
    return 0;
}