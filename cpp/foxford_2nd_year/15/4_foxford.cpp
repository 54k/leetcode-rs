#include <iostream>
#include <vector>
#include <stack>
using namespace std;

int main()
{
    int n, k, p;
    int ans = 0;
    int count = 0;
    cin >> n >> k >> p;
    vector<stack<int>> q(k + 1);
    for (int i = 0; i < n; ++i)
    {
        char op;
        int a, b;
        cin >> op >> a >> b;
        if (op == '+')
        {
            q[a].push(b);
            ++count;
            if (count > p)
            {
                cout << "Error" << endl;
                return 0;
            }
            else
                ans = max(ans, count);
        }
        else
        {
            if (q[a].empty() || q[a].top() != b)
            {
                cout << "Error" << endl;
                return 0;
            }
            else
            {
                q[a].pop();
                --count;
            }
        }
    }
    if (count != 0)
        cout << "Error" << endl;
    else
        cout << ans << endl;
}