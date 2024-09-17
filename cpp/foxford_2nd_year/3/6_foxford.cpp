#include <iostream>
#include <string>
#include <algorithm>
#include <vector>
#include <iomanip>
#include <cmath>
#include <set>
#include <map>
#include <queue>
#include <fstream>
using namespace std;

int h1, h2, m1, m2, t1, t2, sum1;

int main()
{
    cin >> h1 >> m1;
    cin >> h2 >> m2;
    t1 = h1 * 60 + m1;
    t2 = h2 * 60 + m2;
    if (t2 < t1)
    {
        t2 += 1440;
    }
    for (int i = t1; i <= t2; i++)
    {
        if (i % 30 == 0)
        {
            if (i % 60 == 0)
            {
                int h = 0;
                h = (i / 60) % 12;
                if (h == 0)
                {
                    h = 12;
                }
                sum1 += h;
            }
            else
            {
                sum1++;
            }
        }
    }
    cout << sum1 << endl;
    return 0;
}