#include <iostream>
#include <vector>

using namespace std;

int main()
{
    int n, k;
    cin >> n >> k;
    vector<int> v(n);
    for (int i = 0; i < v.size(); i++)
    {
        v[i] = (i + 1);
    }
    int to_rem = 0;
    while (v.size() > 1)
    {
        to_rem = (to_rem + (k - 1)) % v.size();
        // cout << "remove " << v[to_rem] << endl;
        v.erase(v.begin() + to_rem);
    }
    cout << v[0] << endl;
    return 0;
}