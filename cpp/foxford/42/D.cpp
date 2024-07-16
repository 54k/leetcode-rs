#include <bits/stdc++.h>
#include "optimization.h"

using namespace std;

#define forn(i, n) for (int i = 0; i < (int)(n); i++)

const int maxN = 1e5, maxn = 2 * maxN;
const int K = 500, M = 2200;

int pn, n, mem[2][maxn], *a = mem[0], *b = mem[1];

struct Part
{
    int i, n, *b;
    Part() { b = new int[K]; }
    void init(int _i, int _n) { i = _i, n = _n, calc(); }
    void calc()
    {
        memcpy(b, a + i, sizeof(a[0]) * n);
        sort(b, b + n);
    }
    int get(int r)
    {
        return upper_bound(b, b + n, r) - b;
    }
} p[M];

void init()
{
    pn = 0;
    for (int i = 0; i < n; i += K)
        p[pn++].init(i, min(K, n - i));
}
void build()
{
    n = 0;
    forn(i, pn)
        memcpy(b + n, a + p[i].i, p[i].n * sizeof(a[0])),
        n += p[i].n;
    swap(a, b);
    init();
}

void shift_r(int j)
{
    int *b = p[pn].b;
    memmove(p + j + 1, p + j, sizeof(p[0]) * (pn++ - j));
    p[j].b = b;
}
void shift_l(int j)
{
    int *b = p[j].b;
    memmove(p + j, p + j + 1, sizeof(p[0]) * (--pn - j));
    p[pn].b = b;
}
int split(int i)
{
    int j = 0;
    while (j < pn && i >= p[j].n)
        i -= p[j++].n;
    if (i)
    {
        shift_r(j);
        p[j + 1].init(p[j].i + i, p[j].n - i);
        p[j].init(p[j].i, i);
        j++;
    }
    return j;
}

void Add(int i, int x)
{
    shift_r(i = split(i));
    a[n] = x, p[i].init(n++, 1);
}
void Del(int i)
{
    split(i + 1);
    shift_l(split(i));
}
int Get(int L, int R, int x)
{
    int res = 0;
    for (L = split(L), R = split(R + 1); L < R; L++)
        res += p[L].get(x);
    return res;
}

int main()
{
    n = readInt();
    forn(i, n)
        a[i] = readInt();
    init();
    while (!seekEof())
    {
        char type = readChar();
        if (type == '+')
        {
            int x = readInt();
            int y = readInt();
            Add(x, y);
        }
        else if (type == '-')
            Del(readInt());
        else if (type == '?')
        {
            int x = readInt();
            int y = readInt();
            int z = readInt();
            writeInt(Get(x, y, z), '\n');
        }
        else
        {
            fprintf(stderr, "Unknown option: %c\n", type);
            exit(3128);
        }
        if (pn >= M - 2)
            build();
    }
}