#include <bits/stdc++.h>

using namespace std;

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

int main()
{
    int n, m;
    cin >> n >> m;

    UnionFind uf(n);
    int built_bridges = 0;
    vector<pair<int, int>> edges;

    for (int i = 0; i < m; ++i)
    {
        int x, y;
        cin >> x >> y;
        edges.push_back(make_pair(x, y));
    }

    for (auto &edge : edges)
    {
        int x, y;
        x = edge.first;
        y = edge.second;
        if (uf.unite(x, y))
        {
            built_bridges++;
        }
        if (built_bridges == n - 1)
        {
            cout << built_bridges << endl;
            return 0;
        }
    }

    cout << 0 << endl; // Если по каким-то причинам не удалось соединить все острова
    return 0;
}
