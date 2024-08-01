#define NOFOOTER
#define __USE_MINGW_ANSI_STDIO 1

#ifdef BURUNDUK
#include "my_testlib.h"
#else
#include "testlib.h"
#endif

#include <string>
#include <vector>

using namespace std;

#define forn(i, n) for (int i = 0; i < (int)(n); i++)
#define sz(v) (int)(v).size()

typedef long long ll;

string str(ll x)
{
    char buf[99];
    sprintf(buf, I64, x);
    return string(buf);
}

int main(int argc, char *argv[])
{
    registerTestlibCmd(argc, argv);

    int pos = 0;
    vector<ll> v;

    while (!ans.seekEof())
    {
        ll a = ouf.readLong();
        ll b = ans.readLong();
        pos++;
        if (a != b)
            quitf(_wa, "pos = %d : contestant(" I64 ") != jury(" I64 ")", pos, a, b);

        v.push_back(a);
    }
    if (pos == 0)
        quitf(_ok, "file is empty");

    string sv = str(v[0]);
    int M = 3;
    for (int i = 1; i < M && i < sz(v); i++)
        sv += ", " + str(v[i]);
    if (sz(v) > M)
        sv += ", …";

    quitf(_ok, "OK (n = %d, v = %s)", pos, sv.c_str());
    return 0;
}