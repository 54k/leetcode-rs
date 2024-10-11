#include <iostream>
#include <vector>
#include <stack>

using namespace std;

int main()
{
    stack<int> nums;
    nums.push(-1);
    stack<int> costs;
    costs.push(-1);
    int n;
    cin >> n;
    vector<int> ans(n, -1);
    for (int i = 0; i < n; i++)
    {
        int cost;
        cin >> cost;

        while (cost < costs.top())
        {
            ans[nums.top()] = i;
            costs.pop();
            nums.pop();
        }

        costs.push(cost);
        nums.push(i);
    }

    for (int i = 0; i < n; i++)
    {
        cout << ans[i] << " ";
    }
    cout << endl;
    return 0;
}