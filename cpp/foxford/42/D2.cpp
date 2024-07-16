#include <iostream>
#include <vector>
#include <algorithm>
#include <cmath>
using namespace std;
typedef long long ll;
struct Block
{
    int len;
    vector<ll> arr, srt;
    void build()
    {
        len = arr.size();
        srt = arr;
        sort(srt.begin(), srt.end());
    }
    int partask(int l, int r, ll val)
    {
        int ans = 0;
        for (int i = l; i <= r; ++i)
        {
            if (arr[i] <= val)
            {
                ans++;
            }
        }
        return ans;
    }
    int fullask(ll val)
    {
        return upper_bound(srt.begin(), srt.end(), val) - srt.begin();
    }
    void append(int pos, int val)
    {
        arr.insert(arr.begin() + pos, val);
        srt.insert(upper_bound(srt.begin(), srt.end(), val), val);
        len++;
    }
    void del(int pos)
    {
        srt.erase(lower_bound(srt.begin(), srt.end(), arr[pos]));
        arr.erase(arr.begin() + pos);
        len--;
    }
};
struct SQRT
{
    int len, sz = 0;
    vector<Block> blcs;
    SQRT(){};
    SQRT(const vector<ll> &a)
    {
        int n = a.size();
        len = ll(sqrt(n));
        for (int i = 0; i < n; ++i)
        {
            int bl = i / len;
            if (bl == sz)
            {
                blcs.push_back(Block());
                sz++;
            }
            blcs[bl].arr.push_back(a[i]);
        }
        for (int i = 0; i < sz; ++i)
        {
            blcs[i].build();
        }
    }
    void print()
    {
        for (int i = 0; i < sz; ++i)
        {
            for (int j = 0; j < blcs[i].len; ++j)
            {
                cout << blcs[i].arr[j] << ' ';
            }
        }
    }
    void split(int block)
    {
        Block bl1, bl2;
        for (int i = 0; i < blcs[block].len; ++i)
        {
            if (i < blcs[block].len / 2)
            {
                bl1.arr.push_back(blcs[block].arr[i]);
            }
            else
            {
                bl2.arr.push_back(blcs[block].arr[i]);
            }
        }
        bl1.build();
        bl2.build();
        blcs.erase(blcs.begin() + block);
        blcs.insert(blcs.begin() + block, bl2);
        blcs.insert(blcs.begin() + block, bl1);
        sz++;
    }
    pair<int, int> Getpos(int pos)
    {
        for (int i = 0; i < sz; ++i)
        {
            if (blcs[i].len > pos)
            {
                return {i, pos};
            }
            pos -= blcs[i].len;
        }
        return {sz - 1, blcs[sz - 1].len};
    }
    void append(int pos, int val)
    {
        int block, inblock;
        pair<int, int> tmp = Getpos(pos);
        block = tmp.first;
        inblock = tmp.second;
        blcs[block].append(inblock, val);
        if (blcs[block].len > 2 * len)
        {
            split(block);
        }
    }
    void del(int pos)
    {
        int block, inblock;
        pair<int, int> tmp = Getpos(pos);
        block = tmp.first;
        inblock = tmp.second;
        blcs[block].del(inblock);
        if (blcs[block].len == 0)
        {
            blcs.erase(blcs.begin() + block);
            sz--;
        }
    }
    ll ast(int l, int r, ll val)
    {
        int lblock, inlblock;
        pair<int, int> tmp = Getpos(l);
        lblock = tmp.first;
        inlblock = tmp.second;
        int rblock, inrblock;
        tmp = Getpos(r);
        rblock = tmp.first;
        inrblock = tmp.second;
        if (lblock == rblock)
            return blcs[lblock].partask(inlblock, inrblock, val);
        ll ans = 0;
        if (inlblock != 0)
        {
            ans += blcs[lblock].partask(inlblock, blcs[lblock].len - 1, val);
            lblock++;
        }
        if (inrblock != blcs[rblock].len - 1)
        {
            ans += blcs[rblock].partask(0, inrblock, val);
            rblock--;
        }
        for (int i = lblock; i <= rblock; ++i)
        {
            ans += blcs[i].fullask(val);
        }
        return ans;
    }
};
int main()
{
    ios::sync_with_stdio(false);
    cin.tie(0);
    int n;
    cin >> n;
    vector<ll> arr(n);
    for (ll &i : arr)
        cin >> i;
    SQRT s(arr);
    char oper;
    while (cin >> oper)
    {
        if (oper == '+')
        {
            ll pos, val;
            cin >> pos >> val;
            s.append(pos, val);
        }
        else if (oper == '?')
        {
            ll l, r, val;
            cin >> l >> r >> val;
            cout << s.ast(l, r, val) << '\n';
        }
        else
        {
            ll pos;
            cin >> pos;
            s.del(pos);
        }
    }
}