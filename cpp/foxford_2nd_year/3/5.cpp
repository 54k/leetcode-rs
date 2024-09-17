#include <iostream>
#include <algorithm>

int main()
{
    // Координаты первого прямоугольника
    int x1, y1, x2, y2;
    // Координаты второго прямоугольника
    int x3, y3, x4, y4;

    // Ввод координат
    std::cin >> x1 >> y1 >> x2 >> y2;
    std::cin >> x3 >> y3 >> x4 >> y4;

    // Определяем границы пересечения
    int left = std::max(x1, x3);
    int right = std::min(x2, x4);
    int bottom = std::max(y1, y3);
    int top = std::min(y2, y4);

    // Вычисляем размеры пересечения
    int width = right - left;
    int height = top - bottom;

    // Если пересечение существует, вычисляем площадь, иначе площадь = 0
    int area = (width > 0 && height > 0) ? width * height : 0;

    // Выводим площадь пересечения
    std::cout << area << std::endl;

    return 0;
}
