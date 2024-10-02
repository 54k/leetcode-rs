#include <iostream>
#include <vector>
using namespace std;

int main()
{
    int n, f;
    cin >> n >> f;
    int ans = f;
    n--;
    while (n-- > 0)
    {
        int x;
        cin >> x;
        ans = max(ans, x);
    }
    cout << ans << endl;
    return 0;
}