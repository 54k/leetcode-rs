#include <iostream>
using namespace std;

int main()
{
    int N, K, M;
    cin >> N >> K >> M;

    // Если заготовка меньше детали, сразу невозможно сделать ни одной детали
    if (K < M)
    {
        cout << 0 << endl;
        return 0;
    }

    int totalDetails = 0;

    while (N >= K)
    {
        // Шаг 1: Изготавливаем заготовки
        int numBlanks = N / K;
        int remainingMaterial = N % K;

        // Шаг 2: Из каждой заготовки делаем детали
        int numDetails = (K / M) * numBlanks;
        totalDetails += numDetails;

        // Шаг 3: Собираем остатки от заготовок и деталей
        remainingMaterial += (K % M) * numBlanks;

        // Шаг 4: Возвращаем остатки в общий объем сплава
        N = remainingMaterial;
    }

    cout << totalDetails << endl;
    return 0;
}
