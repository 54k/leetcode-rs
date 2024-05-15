#include <bits/stdc++.h>
using namespace std;

int ans = 0, n;

unordered_set<int> cols;
unordered_set<int> diag1;
unordered_set<int> diag2;

void generate(int row)
{
    if (row == 0)
    {
        ans++;
        return;
    }

    for (int col = 0; col < n; col++)
    {
        if (cols.find(col) != cols.end())
        {
            continue;
        }

        if (diag1.find(row - col) != diag1.end())
        {
            continue;
        }

        if (diag2.find(row + col) != diag2.end())
        {
            continue;
        }

        diag1.insert(row - col);
        diag2.insert(row + col);
        cols.insert(col);

        generate(row - 1);

        cols.erase(col);
        diag1.erase(row - col);
        diag2.erase(row + col);
    }
}

int main()
{
    cin >> n;
    generate(n);
    cout << ans;
}