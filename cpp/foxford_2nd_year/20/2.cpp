#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n;
    cin >> n;
    int buck[1001] = {0};
    while (n-- > 0)
    {
        int num;
        cin >> num;
        buck[num]++;
    }

    for (int i = 1; i < 1001; i++)
    {
        while (buck[i]-- > 0)
        {
            cout << i << " ";
        }
    }

    cout << endl;
    return 0;
}
