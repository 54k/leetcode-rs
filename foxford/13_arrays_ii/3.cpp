#include <bits/stdc++.h>

using namespace std;

int main()
{
    int n, k;
    cin >> n >> k;
    vector<int> v(n);
    iota(v.begin(), v.end(), 1);
    int last = 1;
    while (n > 1)
    {
        last = (last + k) % n;
        n--;
    }
    cout << v[last - 1] << "\n";
    return 0;
}