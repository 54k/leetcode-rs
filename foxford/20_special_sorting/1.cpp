#include <bits/stdc++.h>
using namespace std;

int main()
{
    int buck[10] = {0};
    int n;

    while (cin >> n)
    {
        if (n != 0)
            buck[n]++;
    }

    for (int i = 1; i <= 9; i++)
    {
        cout << buck[i] << " ";
    }
    cout << endl;
    return 0;
}
