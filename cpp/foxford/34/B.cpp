#include <iostream>
#include <queue>
#include <unordered_set>

using namespace std;

int sum_of_digits(int num)
{
    int sum = 0;
    while (num > 0)
    {
        sum += num % 10;
        num /= 10;
    }
    return sum;
}

int min_steps(int start, int target)
{
    if (start == target)
    {
        return 0;
    }

    queue<pair<int, int>> q;    // пара (текущее значение, количество шагов)
    unordered_set<int> visited; // множество посещенных значений

    q.push({start, 0});
    visited.insert(start);

    while (!q.empty())
    {
        auto pair = q.front();
        auto current = pair.first;
        auto steps = pair.second;
        q.pop();

        int next1 = current * 3;
        int next2 = current + sum_of_digits(current);
        int next3 = current - 2;

        if (next1 == target || next2 == target || next3 == target)
        {
            return steps + 1;
        }

        if (next1 >= 0 && next1 <= 9999 && visited.find(next1) == visited.end())
        {
            q.push({next1, steps + 1});
            visited.insert(next1);
        }

        if (next2 >= 0 && next2 <= 9999 && visited.find(next2) == visited.end())
        {
            q.push({next2, steps + 1});
            visited.insert(next2);
        }

        if (next3 >= 0 && next3 <= 9999 && visited.find(next3) == visited.end())
        {
            q.push({next3, steps + 1});
            visited.insert(next3);
        }
    }

    return -1; // на случай, если путь не найден (теоретически, этого не произойдет для данной задачи)
}

int main()
{
    int start, target;
    cin >> start >> target;

    int result = min_steps(start, target);
    cout << result << endl;

    return 0;
}
