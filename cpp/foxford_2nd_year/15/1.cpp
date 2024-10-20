#include <bits/stdc++.h>
#include <climits>
using namespace std;

int main()
{
    int n, k;
    cin >> n >> k;
    long long ans = LONG_LONG_MAX;

    vector<long long> v(n);
    for (auto &x : v)
    {
        cin >> x;
    }

    vector<long long> pref(n - k + 1);
    vector<long long> max_left(n - k + 1);
    vector<long long> max_right(n - k + 1);

    // Вычисляем суммы отрезков длины k
    for (int i = 0; i < k; i++)
    {
        pref[0] += v[i];
    }
    for (int i = 1; i < n - k + 1; i++)
    {
        pref[i] = pref[i - 1] - v[i - 1] + v[i + k - 1];
    }

    // Вычисляем максимумы слева
    for (int i = k; i < n - k + 1; i++)
    {
        max_left[i] = max(max_left[i - 1], pref[i - k]);
    }

    // Вычисляем максимумы справа
    for (int i = n - 2 * k; i >= 0; i--)
    {
        max_right[i] = max(max_right[i + 1], pref[i + k]);
    }

    // Теперь ищем минимальное значение
    for (int i = 0; i < n - k + 1; i++)
    {
        ans = min(ans, max(max_left[i], max_right[i]));
    }

    cout << ans << endl;
    return 0;
}
