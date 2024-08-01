#include <cstdio>
#include <algorithm>
#include <vector>
#include <stack>
using namespace std;
struct node
{
    int x, y, left, right, parent;
};
vector<node> nodes;
bool cmp(int a, int b)
{
    return nodes[a].x < nodes[b].x;
}
int main()
{
    int n;
    scanf("%d", &n);
    nodes.resize(n + 1);
    vector<int> order(n);
    for (int i = 1; i <= n; ++i)
    {
        scanf("%d%d", &nodes[i].x, &nodes[i].y);
        nodes[i].left = nodes[i].right = nodes[i].parent = 0;
        order[i - 1] = i;
    }
    sort(order.begin(), order.end(), cmp);
    stack<int> st;
    st.push(order[0]);
    for (int i = 1; i < n; ++i)
    {
        // Добавляем вершину из списка order[i] в дерево
        int curr = order[i];
        int last = 0;
        while (!st.empty() && nodes[st.top()].y > nodes[curr].y)
        {
            last = st.top();
            st.pop();
        }
        nodes[curr].left = last;
        if (last)
            nodes[last].parent = curr;
        if (!st.empty())
        {
            nodes[st.top()].right = curr;
            nodes[curr].parent = st.top();
        }
        st.push(curr);
    }
    for (int i = 1; i <= n; ++i)
        printf("%d %d %d\n", nodes[i].parent, nodes[i].left, nodes[i].right);
}