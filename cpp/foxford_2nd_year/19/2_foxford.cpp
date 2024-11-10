#include <iostream>
using namespace std;
int n, a, b, c, s;
int s1, s2, s3, p;
struct Student
{
    char name[101], surname[101];
    int sapp;
};
Student arr[100002];
int main()
{
    cin >> n;
    for (int i = 0; i < n; i++)
    {
        cin >> arr[i].surname >> arr[i].name >> a >> b >> c;
        s = a + b + c;
        arr[i].sapp = s;
    }
    int s1 = 0, s2 = 0, s3 = 0;
    for (int i = 0; i < n; i++)
    {
        if (arr[i].sapp > s1)
        {
            p = s1;
            s1 = arr[i].sapp;
            s3 = s2;
            s2 = p;
        }
        else if (arr[i].sapp > s2)
        {
            s3 = s2;
            s2 = arr[i].sapp;
        }
        else if (arr[i].sapp > s3)
        {
            s3 = arr[i].sapp;
        }
    }
    for (int i = 0; i < n; i++)
    {
        if (arr[i].sapp >= s3)
        {
            cout << arr[i].surname << " " << arr[i].name << "\n";
        }
    }
}