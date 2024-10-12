#include <vector>

using namespace std;

const int N = 10;
int minDiv[N];
vector<int> primes;

int phi[N]; // функция эйлера

int main()
{
    // iota(minDiv, minDiv + n, 0)
    for (int i = 2; i < N; ++i)
    {
        minDiv[i] = i;
        phi[i] = i - 1;
    }

    for (int i = 2; i < N; i++)
    {
        if (minDiv[i] == i)
        {
            primes.push_back(i);
        }
        for (int j = 0; j < (int)primes.size() && primes[j] < minDiv[i] && i * primes[j] < N; ++j)
        {
            minDiv[i * primes[j]] = primes[j];
            if (i % primes[j] != 0)
            {
                phi[i * primes[j]] = phi[i] * phi[primes[j]];
            }
            else
            {
                phi[i * primes[j]] = phi[i] * primes[j];
            }
        }
    }
}