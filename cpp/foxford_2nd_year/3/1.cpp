#include <iostream>
using namespace std;

int main()
{
    int cyclesNum, A, B, C;
    cin >> A >> B >> C;

    cyclesNum = min(B / 2, min(A, C));
    A -= cyclesNum;
    B -= 2 * cyclesNum;
    C -= cyclesNum;

    int sweetsNum = cyclesNum * 4;
    if (A > 0)
    {
        ++sweetsNum;
        if (B > 0)
        {
            ++sweetsNum;
            if (C > 0)
                ++sweetsNum;
        }
    }
    cout << sweetsNum << endl;
    return 0;
}