#include <iostream>

int countBells(int startHour, int startMinute, int endHour, int endMinute)
{
    // Переводим начальное и конечное время в минуты от начала суток
    int startMinutes = startHour * 60 + startMinute;
    int endMinutes = endHour * 60 + endMinute;

    // Если конечное время меньше начального, значит оно находится на следующий день
    if (endMinutes <= startMinutes)
    {
        endMinutes += 24 * 60; // Добавляем 24 часа к конечному времени
    }

    int totalBells = 0;

    // Находим ближайший 30-минутный интервал от начального времени
    int currentMinutes = startMinutes;
    if (currentMinutes % 30 != 0)
    {
        currentMinutes += 30 - (currentMinutes % 30);
    }

    // Перебираем каждый 30-минутный отрезок
    for (; currentMinutes < endMinutes; currentMinutes += 30)
    {
        int currentHour = (currentMinutes / 60) % 24; // Часы текущего времени (0-23)
        int currentMinute = currentMinutes % 60;      // Минуты текущего времени (0 или 30)

        if (currentMinute == 0)
        {
            // Начало часа: часы бьют столько раз, сколько сейчас часов (0 -> 12 ударов, 13 -> 1 удар и т.д.)
            int hourToRing = currentHour % 12;
            if (hourToRing == 0)
            {
                hourToRing = 12;
            }
            totalBells += hourToRing;
        }
        else if (currentMinute == 30)
        {
            // Половина часа: часы бьют 1 раз
            totalBells += 1;
        }
    }

    return totalBells;
}

int main()
{
    int startHour, startMinute;
    int endHour, endMinute;

    // Ввод начального и конечного времени
    std::cin >> startHour >> startMinute;
    std::cin >> endHour >> endMinute;

    // Подсчет количества ударов
    int result = countBells(startHour, startMinute, endHour, endMinute);

    // Вывод результата
    std::cout << result << std::endl;

    return 0;
}
