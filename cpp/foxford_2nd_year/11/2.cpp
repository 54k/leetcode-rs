#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

// Функция сравнения строк по длине, а при равенстве длины — лексикографически
bool compare(const string &a, const string &b)
{
    if (a.length() == b.length())
    {
        return a < b; // Лексикографическая сортировка
    }
    return a.length() < b.length(); // Сортировка по длине
}

int main()
{
    int n;
    cin >> n;
    vector<string> strings(n);

    // Чтение строк
    for (int i = 0; i < n; ++i)
    {
        cin >> strings[i];
    }

    // Сортировка строк по кастомному правилу
    sort(strings.begin(), strings.end(), compare);

    // Вывод отсортированных строк
    for (const auto &str : strings)
    {
        cout << str << endl;
    }

    return 0;
}
