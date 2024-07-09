#include <bits/stdc++.h>

using namespace std;

struct Position
{
    int x, y;
    Position(int x, int y) : x(x), y(y) {}
};

bool is_valid(int x, int y)
{
    return x >= 0 && x < 8 && y >= 0 && y < 8;
}

vector<string> bfs(string start, string end)
{
    vector<vector<int>> directions = {{2, 1}, {2, -1}, {-2, 1}, {-2, -1}, {1, 2}, {1, -2}, {-1, 2}, {-1, -2}};
    unordered_map<string, string> prev;
    queue<Position> q;

    int start_x = start[0] - 'a';
    int start_y = start[1] - '1';
    int end_x = end[0] - 'a';
    int end_y = end[1] - '1';

    q.push(Position(start_x, start_y));
    prev[start] = "";

    while (!q.empty())
    {
        Position pos = q.front();
        q.pop();
        string current = string(1, pos.x + 'a') + string(1, pos.y + '1');

        if (current == end)
        {
            vector<string> path;
            for (string at = end; at != ""; at = prev[at])
            {
                path.push_back(at);
            }
            reverse(path.begin(), path.end());
            return path;
        }

        for (const auto &dir : directions)
        {
            int new_x = pos.x + dir[0];
            int new_y = pos.y + dir[1];
            if (is_valid(new_x, new_y))
            {
                string neighbor = string(1, new_x + 'a') + string(1, new_y + '1');
                if (prev.find(neighbor) == prev.end())
                {
                    q.push(Position(new_x, new_y));
                    prev[neighbor] = current;
                }
            }
        }
    }
    return {};
}

int main()
{
    string start, end;
    cin >> start >> end;

    vector<string> path = bfs(start, end);

    for (const string &cell : path)
    {
        cout << cell << endl;
    }

    return 0;
}
