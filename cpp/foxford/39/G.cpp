#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;
const int INF = 1e9;
int main()
{
    int n, m, k;
    cin >> n >> m >> k;
    vector<vector<int>> A(n, vector<int>(n, -INF));
    vector<vector<int>> R(n, vector<int>(n, 0));
    vector<vector<int>> Prev(n, vector<int>(n, -1));

    // Заполняем матрицу весов
    for (int i = 1; i <= m; ++i)
    {
        int u, v, w;
        cin >> u >> v >> w;
        --u;
        --v;
        if (w > A[u][v])
        {
            A[u][v] = w;
            R[u][v] = i;
            Prev[u][v] = u;
        }
    }
    // Запускаем Флойда
    for (int t = 0; t < n; ++t)
        for (int i = 0; i < n; ++i)
            for (int j = 0; j < n; ++j)
                if (A[i][t] > -INF && A[t][j] > -INF && A[i][t] + A[t][j] > A[i][j])
                {
                    A[i][j] = A[i][t] + A[t][j];
                    Prev[i][j] = Prev[t][j];
                }
    // Считываем маршрут Уотерса
    vector<int> Path(k);
    for (int i = 0; i < k; ++i)
    {
        cin >> Path[i];
        --Path[i];
    }
    // Проверяем ответ на циклы положительного веса
    for (int i = 1; i < Path.size(); ++i)
    {
        int u = Path[i - 1];
        int v = Path[i];
        for (int j = 0; j < n; ++j)
            if (A[u][j] > -INF && A[j][j] > 0 && A[j][v] > -INF)
            {
                cout << "infinitely kind" << endl;
                return 0;
            }
    }
    // Восстанавливаем путь с конца
    vector<int> Ans;
    for (int i = k - 1; i > 0; --i)
    {
        // Восстанавливаем перелеты между двумя соседними концертами
        int u = Path[i - 1];
        int v = Path[i];
        while (u != v)
        {
            Ans.push_back(R[Prev[u][v]][v]);
            v = Prev[u][v];
        }
    }
    cout << Ans.size() << endl;
    reverse(Ans.begin(), Ans.end());
    for (auto elem : Ans)
        cout << elem << " ";
    cout << endl;
}