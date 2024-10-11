#include <iostream>
#include <string>
#include <algorithm>
using namespace std;

string decimalToBinary(string r)
{
    string osta = "";
    while (r != "0" && r != "1")
    {
        int x = 0;
        string temp = "";
        for (char c : r)
        {
            x = x * 10 + (c - '0');
            temp += (x / 2) + '0';
            x = x % 2;
        }
        if (temp[0] == '0' && temp.size() > 1)
            temp.erase(0, 1);
        osta += to_string(x);
        r = temp;
    }
    osta += r;
    reverse(osta.begin(), osta.end()); // Переворачиваем результат
    return osta;
}

int main()
{
    string text;
    getline(cin, text);
    string result = "";
    string number = "";

    for (char c : text)
    {
        if (isdigit(c))
        {
            number += c; // Собираем число
        }
        else
        {
            if (!number.empty())
            {
                result += decimalToBinary(number); // Преобразуем число в бинарный вид
                number = "";                       // Очищаем строку для следующего числа
            }
            result += c; // Добавляем обычный символ в результат
        }
    }

    if (!number.empty())
    {
        result += decimalToBinary(number); // Преобразуем последнее число в бинарный вид
    }

    cout << result << endl;
    return 0;
}
