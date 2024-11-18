void pascals_triangle(vector<vector<int>> &a, int n)
{
    a.resize(n);
    for (int i = 0; i < n; ++i)
    {
        a[i].resize(i + 1);
        a[i][0] = 1;
        for (int j = 1; j < i; ++j)
            a[i][j] = a[i - 1][j - 1] + a[i - 1][j];
        a[i][i] = 1;
    }
}