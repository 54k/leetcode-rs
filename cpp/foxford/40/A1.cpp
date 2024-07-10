#include <bits/stdc++.h>

using namespace std;

struct Edge
{
    int u, v;
    int weight;
    bool operator<(const Edge &other) const
    {
        return weight < other.weight;
    }
};

class UnionFind
{
public:
    UnionFind(int n) : parent(n), rank(n, 0)
    {
        iota(parent.begin(), parent.end(), 0);
    }

    int find(int x)
    {
        if (parent[x] != x)
        {
            parent[x] = find(parent[x]);
        }
        return parent[x];
    }

    void unite(int x, int y)
    {
        int rootX = find(x);
        int rootY = find(y);
        if (rootX != rootY)
        {
            if (rank[rootX] > rank[rootY])
            {
                parent[rootY] = rootX;
            }
            else if (rank[rootX] < rank[rootY])
            {
                parent[rootX] = rootY;
            }
            else
            {
                parent[rootY] = rootX;
                rank[rootX]++;
            }
        }
    }

private:
    vector<int> parent;
    vector<int> rank;
};

int main()
{
    int n, m;
    cin >> n >> m;

    vector<Edge> edges(m);
    for (int i = 0; i < m; ++i)
    {
        cin >> edges[i].u >> edges[i].v >> edges[i].weight;
        edges[i].u--; // Переводим в 0-индексацию
        edges[i].v--; // Переводим в 0-индексацию
    }

    sort(edges.begin(), edges.end());

    UnionFind uf(n);
    long long mst_weight = 0;

    for (const auto &edge : edges)
    {
        if (uf.find(edge.u) != uf.find(edge.v))
        {
            uf.unite(edge.u, edge.v);
            mst_weight += edge.weight;
        }
    }

    cout << mst_weight << endl;

    return 0;
}
