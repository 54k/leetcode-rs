#include <iostream>

using namespace std;

const int N = 1e5;

int n, k;
int a[N];
long long sum[N];
long long pre[N], suf[N];

int main()
{
    ios_base::sync_with_stdio(false);
    cout.tie(0);
    cin.tie(0);

    cin >> n >> k;
    for (int i = 0; i < n; ++i)
        cin >> a[i];

    for (int i = 0; i < k; ++i)
        sum[0] += a[i];

    for (int i = 1; i < n - k + 1; ++i)
        sum[i] = sum[i - 1] + a[i + k - 1] - a[i - 1];

    pre[0] = sum[0];
    for (int i = 1; i < n - k + 1; ++i)
        pre[i] = max(sum[i], pre[i - 1]);

    suf[n - k] = sum[n - k];
    for (int i = n - k - 1; i >= 0; --i)
        suf[i] = max(sum[i], suf[i + 1]);

    long long ans = min(pre[n - 3 * k + 1], suf[2 * k - 1]);
    for (int i = 0; i < n - 3 * k + 1; ++i)
        ans = min(ans, max(pre[i], suf[i + 2 * k]));

    cout << ans << endl;
}