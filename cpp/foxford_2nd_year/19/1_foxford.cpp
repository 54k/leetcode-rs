#define _CRT_SECURE_NO_WARNINGS // чтобы использовать scanf и printf и другие небезопасные без ошибок
#include <iostream>
#include <stdio.h>
#include <cmath>
#include <algorithm>
using namespace std;
struct Point
{
    int x;
    int y;
    double r;
};
bool comparator(Point x1, Point x2)
{
    return x1.r < x2.r;
}
int main()
{
    int n;
    scanf("%d", &n);
    Point *points = new Point[n];
    for (int i = 0; i < n; i++)
    {
        int x, y;
        scanf("%d %d", &x, &y);
        double r = sqrt(x * x + y * y);
        points[i].x = x;
        points[i].y = y;
        points[i].r = r;
    }
    sort(points, points + n, comparator);
    for (int i = 0; i < n; i++)
    {
        printf("%d %d\n", points[i].x, points[i].y);
    }
}