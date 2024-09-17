#include <iostream>
#include <cmath>
using namespace std;

int main()
{
    int a;
    cin >> a;
    for (int i = 1; i <= int(sqrt(a)); i++)
        for (int j = 0; j <= int(sqrt(a)); j++)
            for (int t = 0; t <= int(sqrt(a)); t++)
                for (int l = 0; l <= int(sqrt(a)); l++)
                    if ((i * i + j * j + t * t + l * l) == a)
                    {
                        cout << l << " ";
                        if (t != 0)
                            cout << t << " ";
                        if (j != 0)
                            cout << j << " ";
                        if (i != 0)
                            cout << i << " ";
                        return (0);
                    }
    return 0;
}