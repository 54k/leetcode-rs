#include <cmath>
#include <iostream>
#include <sstream>
using namespace std;

int main() {
  string s, r = "", osta = "", s1 = "", s2 = "", text;
  char c, c1;
  int i = 0, x = 0, y = 0, d = 0;

  getline(cin, text);
  while (d < text.size()) {
    c1 = text[d];

    if (c1 > '0' && c1 <= '9') {
      while (c1 >= '0' && c1 <= '9') {
        r = r + c1;
        d++;
        c1 = text[d];
      }
      text.erase(d - r.size(), r.size());
      d = d - r.size();
      while (r != "0" && r != "1") {
        x = 0;
        i = 0;
        s = r;
        r = "";
        while (i < s.size()) {
          x = x * 10 + s[i] - '0';
          c = char(x / 2 + '0');
          r = r + c;
          x = x % 2;
          i++;
        }
        if (r.find("0") == 0 && r.size() > 1) {
          r.erase(0, 1);
        }
        c = char(x + '0');
        osta = osta + c;
      }
      osta = osta + r;
      // reverse osta
      i = 0;
      y = osta.size() - 1;
      while (i < osta.size() / 2) {
        s2 = osta[i];
        s1 = osta[y];
        osta.erase(y, 1);
        osta.erase(i, 1);
        osta.insert(i, s1);
        osta.insert(y, s2);
        i++;
        y--;
      }
      // end reverse osta
      text.insert(d, osta);
      d = d + osta.size();
    }
    r = "";
    s = "";
    osta = "";
    i = 0;
    x = 0;
    y = 0;
    d++;
  }
  cout << text;
  return 0;
}