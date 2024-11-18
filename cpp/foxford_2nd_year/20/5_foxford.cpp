#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int main()
{
    int n;
    cin >> n;
    vector<int> p;
    p.reserve(n);
    for (int i = 1; i <= n; ++i)
    {
        p.push_back(i);
    }
    vector<int> start(p);
    do
    {
        for (int i : p)
        {
            cout << i;
        }
        cout << endl;
        next_permutation(p.begin(), p.end());
    } while (!equal(p.begin(), p.end(), start.begin(), start.end()));
    return 0;
}