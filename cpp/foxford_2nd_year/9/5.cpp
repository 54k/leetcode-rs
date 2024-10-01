#include <iostream>
#include <cmath>
#include <algorithm>

using namespace std;

// Функция для нахождения НОД (наибольшего общего делителя)
int gcd(int a, int b)
{
    if (b == 0)
        return a;
    return gcd(b, a % b);
}

int count_cells(int x1, int y1, int x2, int y2)
{
    // Вычисляем изменения по осям x и y
    int dx = abs(x2 - x1);
    int dy = abs(y2 - y1);

    // Количество клеток = dx + dy - НОД(dx, dy)
    return dx + dy - gcd(dx, dy);
}

int main()
{
    int x1, y1, x2, y2;

    // Чтение входных данных
    cin >> x1 >> y1 >> x2 >> y2;

    // Вывод результата
    cout << count_cells(x1, y1, x2, y2) << endl;

    return 0;
}
