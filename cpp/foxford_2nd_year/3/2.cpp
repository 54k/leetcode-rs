#include <iostream>
using namespace std;

int main()
{
    int maxWeight; // Максимальный допустимый вес рюкзака
    int n;         // Количество предметов

    cin >> maxWeight;
    cin >> n;

    int backpackWeight = 0; // Текущий вес рюкзака
    int suitcaseWeight = 0; // Текущий вес чемодана

    for (int i = 0; i < n; ++i)
    {
        int itemWeight;
        cin >> itemWeight;

        // Если вес предмета поместится в рюкзак
        if (backpackWeight + itemWeight <= maxWeight)
        {
            backpackWeight += itemWeight;
        }
        else
        {
            suitcaseWeight += itemWeight;
        }
    }

    // Вывод результата: вес рюкзака и чемодана
    cout << backpackWeight << " " << suitcaseWeight << endl;

    return 0;
}
