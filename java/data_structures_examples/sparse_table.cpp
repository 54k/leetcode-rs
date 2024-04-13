#include <bits/stdc++.h>
using namespace std;

// https://leetcode.com/problems/largest-rectangle-in-histogram/description/
class Solution
{
public:
    int largestRectangleArea(vector<int> &heights)
    {
        int n = heights.size();
        int lg = (int)log2(heights.size());
        vector<vector<int>> st(lg + 1, vector<int>(n));

        for (int l = 0; l <= lg; l++)
        {
            for (int i = 0; i + (1 << l) <= n; i++)
            {
                if (l == 0)
                {
                    st[l][i] = i;
                }
                else
                {
                    int li = st[l - 1][i], ri = st[l - 1][i + (1 << (l - 1))];
                    st[l][i] = heights[li] <= heights[ri] ? li : ri;
                }
            }
        }

        auto ff = [&](int l, int r)
        {
            int lg = log2(r - l + 1);
            int li = st[lg][l], ri = st[lg][r - (1 << lg) + 1];
            return heights[li] <= heights[ri] ? li : ri;
        };

        function<int(int, int)> dc = [&](int l, int r)
        {
            if (l > r)
            {
                return 0;
            }
            int minindex = ff(l, r);
            return max(heights[minindex] * (r - l + 1), max(dc(l, minindex - 1), dc(minindex + 1, r)));
        };

        return dc(0, n - 1);
    }
};