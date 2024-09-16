#include <iostream>
#include <string>
#include <iomanip> // Для setfill и setw
#include <sstream> // Для ostringstream

std::string convertTo24HourFormat(const std::string &time12)
{
    int hours, minutes;
    char period;

    // Парсинг входной строки
    std::sscanf(time12.c_str(), "%d:%d %c.m.", &hours, &minutes, &period);

    // Преобразование времени в 24-часовой формат
    if (period == 'a')
    {
        if (hours == 12)
            hours = 0; // 12:xx a.m. -> 00:xx
    }
    else
    { // period == 'p'
        if (hours != 12)
            hours += 12; // Не преобразуем 12:xx p.m., остальные часы добавляем 12
    }

    // Формирование строки с учетом нулей
    std::ostringstream oss;
    oss << std::setfill('0') << std::setw(2) << hours << ":"
        << std::setfill('0') << std::setw(2) << minutes;

    return oss.str();
}

int main()
{
    std::string time12;
    std::getline(std::cin, time12);

    std::string time24 = convertTo24HourFormat(time12);
    std::cout << time24 << std::endl;

    return 0;
}
