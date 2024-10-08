#include <iostream>
#include <vector>
#include <cstdlib>
using namespace std;
int main()
{
    int ans = 0;
    int n;
    cin >> n;
    vector<int> a(n);
    for (int i = 0; i < n; ++i)
        cin >> a[i];
    for (int i = 0; i < n; ++i)
        if (a[i] == 1)
        {
            int mind = n;
            for (int j = 0; j < n; ++j)
                if (a[j] == 2)
                    mind = min(mind, abs(i - j));
            ans = max(ans, mind);
        }
    cout << ans << endl;
}