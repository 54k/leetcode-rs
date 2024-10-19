#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n, k, p, total = 0;
    cin >> n >> k >> p;
    vector<stack<int>> q(k + 1);
    bool err = false;
    int mx = 0;
    for (int i = 0; i < n; i++)
    {
        char op;
        int a, b;
        cin >> op >> a >> b;
        if (op == '+')
        {
            q[a].push(b);
            total++;
            if (total > p)
            {
                err = true;
                break;
            }
        }
        else
        {
            if (q[a].top() != b)
            {
                err = true;
                break;
            }
            q[a].pop();
            total--;
        }
        mx = max(mx, total);
    }

    if (err || total > 0)
    {
        cout << "Error" << endl;
    }
    else
    {
        cout << mx << endl;
    }
    return 0;
}