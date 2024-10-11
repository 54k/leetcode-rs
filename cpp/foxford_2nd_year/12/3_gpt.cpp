#include <iostream>
#include <algorithm>
#include <string>
using namespace std;

// Функция для деления строки, представляющей большое число, на 2
std::string divideByTwo(std::string number)
{
    std::string result;
    int carry = 0;

    for (char digit : number)
    {
        int current = carry * 10 + (digit - '0'); // текущий разряд числа
        result += (current / 2) + '0';            // добавляем результат деления на 2
        carry = current % 2;                      // сохраняем остаток, чтобы учесть его на следующем разряде
    }

    // Убираем ведущие нули
    while (result[0] == '0' && result.length() > 1)
    {
        result.erase(result.begin());
    }

    return result;
}

// Функция для перевода строки-числа в двоичное представление
std::string decimalToBinary(std::string number)
{
    std::string binary_result;
    while (number != "0")
    {
        int last_digit = (number[number.size() - 1] - '0') % 2; // определяем последний бит
        binary_result += std::to_string(last_digit);            // добавляем его в результат
        number = divideByTwo(number);                           // делим строку-число на 2
    }

    // Разворачиваем строку, так как двоичные разряды идут в обратном порядке
    std::reverse(binary_result.begin(), binary_result.end());

    return binary_result.empty() ? "0" : binary_result;
}

int main()
{
    string s;
    cin >> s;
    string ans = "";

    string num = "";
    for (int i = 0; i < s.size(); i++)
    {
        if (s[i] >= '0' && s[i] <= '9')
        {
            num += s[i];
        }
        else
        {
            if (num.size() > 0)
            {
                ans += decimalToBinary(num);
                num = "";
            }
            ans += s[i];
        }
    }
    if (num.size() > 0)
    {
        ans += decimalToBinary(num);
        num = "";
    }
    cout << ans << endl;
    return 0;
}