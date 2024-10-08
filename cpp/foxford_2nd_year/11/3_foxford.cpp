#include <vector>
#include <iostream>
using namespace std;
int main()
{
    int n, k;
    cin >> n >> k;
    vector<int> a;
    for (int i = 1; i <= n; ++i)
        a.push_back(i);
    int last = 0;
    while (a.size() > 1)
    {
        last = (last + k - 1) % a.size();
        a.erase(a.begin() + last);
    }
    cout << a[0] << endl;
}