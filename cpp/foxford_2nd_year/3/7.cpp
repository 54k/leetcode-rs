#include <iostream>
#include <cmath>

bool isPerfectSquare(int num)
{
    int root = static_cast<int>(std::sqrt(num));
    return root * root == num;
}

int main()
{
    int n;
    std::cin >> n;

    // Перебираем все возможные значения a, b, c
    for (int a = 0; a * a <= n; ++a)
    {
        for (int b = a; a * a + b * b <= n; ++b)
        {
            for (int c = b; a * a + b * b + c * c <= n; ++c)
            {
                int remaining = n - (a * a + b * b + c * c);
                if (remaining >= 0 && isPerfectSquare(remaining))
                {
                    int d = static_cast<int>(std::sqrt(remaining));
                    std::cout << a << " " << b << " " << c << " " << d << std::endl;
                    return 0;
                }
            }
        }
    }

    return 0;
}
