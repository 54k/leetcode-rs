#include <bits/stdc++.h>
using namespace std;

const int N = 0;
int timer = 0;
vector<int> G[N];
int sz[N]; // размер компонентов
int tin[N], tout[N];
int up[N];  // голова тяжелого пути для вершины
int par[N]; // массив предков

#define INF 1000000000;

// g[v][0] -- тяжелое ребро
// ребра в предков удалены
void dfs(int v, int p)
{
    sz[v] = 1;
    // удалили ребро в предка
    // !! не забыть & чтобы свап работал
    for (auto &to : G[v])
    {
        if (to == p)
        {
            swap(G[v].back(), to);
            G[v].pop_back();
            break;
        }
    }

    for (auto &to : G[v])
    {
        dfs(to, v);
        sz[v] += sz[to];
        if (sz[to] > sz[G[v][0]])
        {
            swap(G[v][0], to);
        }
    }
}

void dfsHdl(int v, int p)
{
    tin[v] = timer++; // !! нужно считать именно в этом дфс
    par[v] = p;
    for (auto to : G[v])
    {
        if (to == G[v][0])
        {
            up[to] = up[v];
        }
        else
        {
            up[to] = to;
        }

        dfsHdl(to, v);
    }
    tout[v] = timer; // не увеличиваем timer
}

int get(int a, int b);
// u -- верхняя вершина, v -- нижняя вершина
// считаем что функция get возвращает минимум в дереве отрезков на интервале
int getHld(int u, int v)
{
    int ans = INF;
    while (true)
    {
        if (up[u] != up[v])
        {
            // u и v -- в разных тяжелых путях
            ans = min(ans, get(tin[up[v]], tin[v] + 1));
            v = par[up[v]];
        }
        else
        {
            // u и v -- в одном пути
            ans = min(ans, get(tin[u], tin[v] + 1));
            break;
        }
    }
    return ans;
}

int main()
{
    return 0;
}