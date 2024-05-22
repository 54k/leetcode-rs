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
    vector<vector<int>> dp(m + 1, vector<int>(n + 1, 0));

    for (int i = 1; i < m; i++)
    {
        int ind = -1;
        int best = 0;
        for (int j = 0; j < n; j++)
        {
            dp[i][j] = dp[i - 1][j];
            if (a[i] == b[j] && dp[i - 1][j] < best + 1)
            {
                dp[i][j] = best + 1;
                prev[j] = ind;
            }
            if (a[i - 1] > b[j - 1] && dp[i - 1][j] > best)
            {
                best = dp[i - 1][j];
                ind = j;
            }
        }
    }

    int pos = 0;
    for (int j = 0; j < n; j++)
    {
        if (dp[m - 1][pos] < dp[m - 1][j])
        {
            pos = j;
        }
    }

    vector<int> ans;
    while (pos != -1)
    {
        ans.push_back(b[pos]);
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
//     ind = 0 // позиция "лучшего" элемента в массиве b
//     best = 0 // значение динамики для "лучшего" элемента
//     for j = 1 to m
//       d[i][j] = d[i - 1][j] // НОВП на a[1..i - 1] и b[1..j] (без элемента a[i])
//       if a[i] == b[j] and d[i - 1][j] < best + 1 // используем a[i]-й элемент для увеличения НОВП
//         d[i][j] = best + 1
//         prev[j] = ind
//       if a[i] > b[j] and d[i - 1][j] > best // при следующем равенстве a[i] == b[j']
//         best = d[i - 1][j] // в best будет храниться "лучший" элемент
//         ind = j // b[ind] < b[j'] и d[i-1][ind] →
//  max
//   // восстановление (по массиву b)
//   pos = 1 // ищем лучший элемент d[n][pos] →
//  max
//   for j = 1 to m
//     if d[n][pos] < d[n][j]
//       pos = j
//   // проходим по массиву b, выписывая элементы НОВП
//   answer: vector<int>
//   while pos ≠
//  0
//     answer.pushBack(b[pos])
//     pos = prev[pos]
//   return answer