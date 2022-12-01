#include <stdio.h>

int is_hate_num(long n) {
    int set[10];
    for (int i = 0; i < 10; ++i) {
        set[i] = 0;
    }

    while (n != 0) {
        int mod = n % 10;
        if (set[mod] == 1 || mod == 2 || mod == 0) {
            return 1;
        }
        set[mod] = 1;
        n /= 10;
    }

    return 0;
}

int main() {
    long n;

    scanf("%ld", &n);

    while (n < 100000001 && is_hate_num(++n)) {}
    if (n >= 100000001) n = -1;

    printf("%ld", n);
    return 0;
}
