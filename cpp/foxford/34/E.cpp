#include <iostream>
#include <vector>
#include <queue>
#include <tuple>
#include <climits>

using namespace std;

const int INF = INT_MAX;

struct Edge
{
    int to, cost;
};

struct State
{
    int city, fuel, cost;
    bool operator>(const State &other) const
    {
        return cost > other.cost;
    }
};

int dijkstra(int n, const vector<int> &prices, const vector<vector<Edge>> &graph)
{
    vector<vector<int>> dist(n, vector<int>(3, INF));
    priority_queue<State, vector<State>, greater<State>> pq;

    pq.push({0, 0, 0});
    dist[0][0] = 0;

    while (!pq.empty())
    {
        State current = pq.top();
        pq.pop();

        int city = current.city;
        int fuel = current.fuel;
        int cost = current.cost;

        if (city == n - 1 && fuel == 0)
        {
            return cost;
        }

        if (cost > dist[city][fuel])
            continue;

        // Переезды
        if (fuel > 0)
        {
            for (const auto &edge : graph[city])
            {
                int next_city = edge.to;
                int next_fuel = fuel - 1;
                if (cost < dist[next_city][next_fuel])
                {
                    dist[next_city][next_fuel] = cost;
                    pq.push({next_city, next_fuel, cost});
                }
            }
        }

        // Заправка одного бака
        if (fuel < 2)
        {
            int next_fuel = fuel + 1;
            int next_cost = cost + prices[city];
            if (next_cost < dist[city][next_fuel])
            {
                dist[city][next_fuel] = next_cost;
                pq.push({city, next_fuel, next_cost});
            }
        }

        // Заправка бака и канистры
        if (fuel < 1)
        {
            int next_fuel = 2;
            int next_cost = cost + 2 * prices[city];
            if (next_cost < dist[city][next_fuel])
            {
                dist[city][next_fuel] = next_cost;
                pq.push({city, next_fuel, next_cost});
            }
        }
    }

    return -1;
}

int main()
{
    int n, m;
    cin >> n;

    vector<int> prices(n);
    for (int i = 0; i < n; ++i)
    {
        cin >> prices[i];
    }

    cin >> m;
    vector<vector<Edge>> graph(n);
    for (int i = 0; i < m; ++i)
    {
        int u, v;
        cin >> u >> v;
        u--;
        v--;
        graph[u].push_back({v, 1});
        graph[v].push_back({u, 1});
    }

    int result = dijkstra(n, prices, graph);
    cout << result << endl;

    return 0;
}
