#include <iostream>
#include <vector>
using namespace std;

int main()
{
    int n, k;
    cin >> n >> k;
    vector<int> v(n);
    for (auto &x : v)
    {
        cin >> x;
    }
    while (k-- > 0)
    {
        int q;
        cin >> q;

        bool found = false;
        int lo = 0;
        int hi = n - 1;
        while (lo <= hi)
        {
            int mid = (lo + hi) / 2;
            if (v[mid] == q)
            {
                found = true;
                break;
            }
            else if (v[mid] > q)
            {
                hi = mid - 1;
            }
            else
            {
                lo = mid + 1;
            }
        }
        cout << (found ? "YES" : "NO") << "\n";
    }
    cout << endl;
    return 0;
}