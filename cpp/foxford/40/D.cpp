#include <bits/stdc++.h>
using namespace std;

class UnionFind
{
public:
    UnionFind(int n) : parent(n), rank(n, 0)
    {
        for (int i = 0; i < n; ++i)
        {
            parent[i] = i;
        }
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
    int n, m, k;
    cin >> n >> m >> k;
    n++; // Because nodes are 1-indexed

    vector<pair<int, int>> edges(m);
    vector<pair<string, pair<int, int>>> operations(k);
    vector<string> result;

    // Read edges
    for (int i = 0; i < m; ++i)
    {
        int u, v;
        cin >> u >> v;
        edges[i] = {u, v};
    }

    // Read operations
    for (int i = 0; i < k; ++i)
    {
        string op;
        int u, v;
        cin >> op >> u >> v;
        operations[i] = {op, {u, v}};
    }

    UnionFind uf(n);

    // Process operations in reverse order
    stack<string> output;
    for (int i = k - 1; i >= 0; --i)
    {
        string op = operations[i].first;
        int u = operations[i].second.first;
        int v = operations[i].second.second;

        if (op == "cut")
        {
            uf.unite(u, v);
        }
        else if (op == "ask")
        {
            if (uf.find(u) == uf.find(v))
            {
                output.push("YES");
            }
            else
            {
                output.push("NO");
            }
        }
    }

    // Print results in the correct order
    while (!output.empty())
    {
        cout << output.top() << endl;
        output.pop();
    }

    return 0;
}
