#include <iostream>
#include <vector>
#include <tuple>
#include <algorithm>
#include <climits>

using namespace std;

const int INF = INT_MAX;

int main()
{
    int n, m, k, s, t;
    cin >> n >> m >> k >> s >> t;
    s--;
    t--; // Перевод в 0-индексацию

    vector<tuple<int, int, int>> flights(m);
    for (int i = 0; i < m; ++i)
    {
        int u, v, w;
        cin >> u >> v >> w;
        flights[i] = make_tuple(u - 1, v - 1, w);
    }

    // Массив для хранения минимальных стоимостей: dist[количество ночей][город]
    vector<vector<int>> dist(k + 1, vector<int>(n, INF));
    dist[0][s] = 0;

    // Запуск алгоритма Форда-Беллмана для каждого количества ночей от 1 до k
    for (int night = 1; night <= k; ++night)
    {
        for (const auto &flight : flights)
        {
            int u, v, w;
            tie(u, v, w) = flight;
            if (dist[night - 1][u] != INF)
            {
                dist[night][v] = min(dist[night][v], dist[night - 1][u] + w);
            }
        }
    }

    // Результат - минимальная стоимость добраться до города t за не более чем k ночей
    int result = INF;
    for (int night = 1; night <= k; ++night)
    {
        result = min(result, dist[night][t]);
    }

    if (result == INF)
    {
        cout << -1 << endl;
    }
    else
    {
        cout << result << endl;
    }

    return 0;
}
