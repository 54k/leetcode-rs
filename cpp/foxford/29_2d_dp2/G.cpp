#include <bits/stdc++.h>
using namespace std;

int main()
{
    int m, n;

    cin >> m;
    vector<int> a(m);
    for (auto &x : a)
    {
        cin >> x;
    }

    cin >> n;
    vector<int> b(n);
    for (auto &x : b)
    {
        cin >> x;
    }

    vector<int> prev(n, -1);
    vector<vector<int>> dp(m, vector<int>(n, 0));

    for (int i = 0; i < m; i++)
    {
        for (int j = 0; j < n; j++)
        {
            if (a[i] == b[j])
            {
                dp[i][j] = 1;
                for (int k = 0; k < i; k++)
                {
                    if (a[k] < a[i] && dp[i][j] < dp[k][j] + 1)
                    {
                        dp[i][j] = dp[k][j] + 1;
                        prev[i] = k;
                    }
                }
            }
            else
            {
                dp[i][j] = dp[i][j - 1];
            }
        }
    }

    int pos = 0;
    for (int i = 0; i < m; i++)
    {
        if (dp[pos][n - 1] < dp[i][n - 1])
        {
            pos = i;
        }
    }

    vector<int> ans;
    while (pos != -1)
    {
        ans.push_back(a[pos]);
        pos = prev[pos];
    }

    cout << ans.size() << "\n";
    for (int i = ans.size() - 1; i >= 0; i--)
    {
        cout << ans[i] << " ";
    }
    cout << endl;
}

// vector<int> LCIS(a: int[n], b: int[m])
//   for i = 1 to n
//     for j = 1 to m
//       if a[i] == b[j]
//         d[i][j] = 1 // НОВП как минимум 1, состоит из одного элемента a[i] <-> b[j]
//         for k = 1 to i - 1
//           if a[k] < a[i] and d[i][j] < d[k][j] + 1
//             d[i][j] = d[k][j] + 1
//             prev[i] = k
//       else
//         d[i][j] = d[i][j - 1]
//   // восстановление
//   pos = 1 // ищем элемент c максимальным d[pos][m]
//   for i = 1 to n
//     if d[pos][m] < d[i][m]
//       pos = i
//   // проходим по массиву a, выписывая элементы НОВП
//   answer: vector<int>
//   while pos ≠
//  0
//     answer.pushBack(a[pos])
//     pos = prev[pos]
//   return answer