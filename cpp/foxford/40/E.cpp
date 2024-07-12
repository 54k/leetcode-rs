#include <bits/stdc++.h>
using namespace std;

struct Edge
{
    int u, v, weight;
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

    bool unite(int x, int y)
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
            return true;
        }
        return false;
    }

private:
    vector<int> parent;
    vector<int> rank;
};

int kruskal(int n, vector<Edge> &edges, vector<Edge> &mst)
{
    UnionFind uf(n);
    sort(edges.begin(), edges.end());
    int mst_weight = 0;
    for (const auto &edge : edges)
    {
        if (uf.unite(edge.u, edge.v))
        {
            mst_weight += edge.weight;
            mst.push_back(edge);
            if (mst.size() == n - 1)
            {
                break;
            }
        }
    }
    return mst_weight;
}

int find_second_mst(int n, vector<Edge> &edges, vector<Edge> &first_mst)
{
    int second_mst_weight = numeric_limits<int>::max();
    for (const auto &edge_to_remove : first_mst)
    {
        UnionFind uf(n);
        int mst_weight = 0;
        int edges_used = 0;
        for (const auto &edge : edges)
        {
            if (edge.u == edge_to_remove.u && edge.v == edge_to_remove.v && edge.weight == edge_to_remove.weight)
            {
                continue;
            }
            if (uf.unite(edge.u, edge.v))
            {
                mst_weight += edge.weight;
                edges_used++;
                if (edges_used == n - 1)
                {
                    break;
                }
            }
        }
        if (edges_used == n - 1)
        {
            second_mst_weight = min(second_mst_weight, mst_weight);
        }
    }
    return second_mst_weight;
}

int main()
{
    int n, m;
    cin >> n >> m;

    vector<Edge> edges(m);
    for (int i = 0; i < m; ++i)
    {
        cin >> edges[i].u >> edges[i].v >> edges[i].weight;
        edges[i].u--;
        edges[i].v--;
    }

    vector<Edge> first_mst;
    int first_mst_weight = kruskal(n, edges, first_mst);
    int second_mst_weight = find_second_mst(n, edges, first_mst);

    cout << first_mst_weight << " " << second_mst_weight << endl;

    return 0;
}
