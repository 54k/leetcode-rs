#include <iostream>
#include <cmath>
#include <vector>
#include <string>
using namespace std;
int main()
{
    int n, k = 1;
    int A[9] = {0, 0, 0, 0, 0, 0, 0, 0, 0};
    while (k != 0)
    {
        cin >> n;
        if (n != 0)
            A[n - 1]++;
        else
            k = 0;
    }
    for (auto x : A)
        cout << x << " ";
    return 0;
}