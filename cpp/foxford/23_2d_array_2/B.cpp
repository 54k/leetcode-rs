// #include <bits/stdc++.h>
// using namespace std;

void pascals_triangle(vector<vector<int>> &a, int n)
{ /* … */
    a.resize(n);
    for (int i = 0; i < n; i++)
    {
        a[i] = vector<int>(i + 1);
        for (int j = 0; j <= i; j++)
        {
            if (j == 0 or j == i)
            {
                a[i][j] = 1;
            }
            else
            {
                a[i][j] = a[i - 1][j - 1] + a[i - 1][j];
            }
        }
    }
}

// int main()
// {
//     while (1)
//     {
//         int n;
//         cin >> n;
//         vector<vector<int>> a;
//         pascals_triangle(a, n);
//         for (int i = 0; i < a.size(); ++i)
//         {
//             for (int j = 0; j < a[i].size(); ++j)
//                 cout << a[i][j] << " ";
//             cout << endl;
//         }
//     }
//     return 0;
// }