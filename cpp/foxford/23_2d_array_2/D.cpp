#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n, m;
    cin >> n >> m;
    vector<vector<int>> a(n, vector<int>(m, 0));
    for (int i = 0; i < a.size(); ++i)
    {
        for (int j = 0; j < a[i].size(); ++j)
            cout << i+j*n+1 << " ";
        cout << endl;
    }
    return 0;
}