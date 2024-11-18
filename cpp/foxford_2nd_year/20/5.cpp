#include <bits/stdc++.h>
using namespace std;

int n;
vector<int> cur;

void permute()
{
    if (cur.size() == n)
    {
        for (auto &x : cur)
        {
            cout << x;
        }
        cout << "\n";
        return;
    }

    for (int i = 1; i <= n; i++)
    {
        if (find(cur.begin(), cur.end(), i) == cur.end())
        {
            cur.push_back(i);
            permute();
            cur.pop_back();
        }
    }
}

int main()
{
    cin >> n;
    permute();
    return 0;
}