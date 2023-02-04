"""
Дана строка, состоящая из букв A-Z:
"AAAABBBCCXYZDDDDEEEFFFAAAAAABBBBBBBBBBBBBBBBBBBBBBBBBBBB"
Нужно написать функцию RLE, которая на выходе даст строку вида:
"A4B3C2XYZD4E3F3A6B28"
И сгенерирует любую ошибку, если на вход пришла невалидная строка.

Пояснения:
1. Если символ встречается 1 раз, он остается без изменений
2. Если символ повторяется более 1 раза, к нему добавляется количество повторений
"""


def rle(st):
    last_symbol = None
    last_count = 0
    result = []
    for s in st:
        if not s.isalpha():
            0 / 0
        else:
            if last_symbol == s:
                last_count += 1
            else:
                if last_symbol:
                    result.append(f'{last_symbol}{last_count}')
                last_count = 1
                last_symbol = s

    result.append(f'{last_symbol}{last_count}')

    return ''.join(result)


print(
    rle("AAAABBBCCXYZDDDDEEEFFFAAAAAABBBBBBBBBBBBBBBBBBBBBBBBBBBB")
)
