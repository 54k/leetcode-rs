#include <bits/stdc++.h>
using namespace std;

long long _merge(vector<int> &arr, vector<int> &tmp, int left, int mid, int right)
{
    int i = left;
    int k = left;
    int j = mid;

    long long inv_count = 0;

    while (i < mid and j <= right)
    {
        if (arr[i] <= arr[j])
        {
            tmp[k++] = arr[i++];
        }
        else
        {
            tmp[k++] = arr[j++];
            inv_count += (mid * 1ll - i);
        }
    }

    while (i < mid)
    {
        tmp[k++] = arr[i++];
    }
    while (j <= right)
    {
        tmp[k++] = arr[j++];
    }
    for (i = left; i <= right; i++)
    {
        arr[i] = tmp[i];
    }
    return inv_count;
}

long long _mergeSort(vector<int> &arr, vector<int> &tmp, int left, int right)
{
    long long inv_count = 0;
    if (left < right)
    {
        int mid = (left + right) / 2;
        inv_count += _mergeSort(arr, tmp, left, mid);
        inv_count += _mergeSort(arr, tmp, mid + 1, right);
        inv_count += _merge(arr, tmp, left, mid + 1, right);
    }
    return inv_count;
}

long long mergeSort(vector<int> &arr)
{
    vector<int> tmp(arr.size());
    return _mergeSort(arr, tmp, 0, arr.size() - 1);
}

int main()
{
    int n;
    cin >> n;
    vector<int> v(n);
    for (auto &e : v)
    {
        cin >> e;
    }

    long long ans = mergeSort(v);
    cout << ans << endl;
    return 0;
}