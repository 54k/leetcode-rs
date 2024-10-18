#include <iostream>
#include <deque>
using namespace std;
int main()
{
    deque<int> first;
    deque<int> second;
    int n;
    cin >> n;
    for (int i = 0; i < n; ++i)
    {
        char op;
        int id;
        cin >> op;
        if (op == '-')
        {
            cout << first.front() << endl;
            first.pop_front();
        }
        if (op == '+')
        {
            cin >> id;
            second.push_back(id);
        }
        if (op == '*')
        {
            cin >> id;
            second.push_front(id);
        }
        if (second.size() > first.size())
        {
            first.push_back(second.front());
            second.pop_front();
        }
    }
}