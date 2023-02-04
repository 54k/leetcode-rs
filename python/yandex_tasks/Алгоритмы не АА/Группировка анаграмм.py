"""
Дан массив строк, нужно сгруппировать в нем анаграммы.
Слово X является анаграммой слова Y, если оно может быть
получено из другого перестановкой букв.
"""

from collections import defaultdict
from pprint import pprint


def group_anagrams(words):
    anagrams = defaultdict(list)

    # for w in words:
    #     w_sorted = sorted(w)
    #     anagrams[''.join(w_sorted)].append(w)

    for w in words:
        chars_count = defaultdict(int)
        for c in w:
            chars_count[c] += 1
        anagrams[frozenset(chars_count.items())].append(w)  # используем сет таплов как ключ (он хешебл)

    return anagrams.values()


pprint(
    group_anagrams(
        ['сон', 'нос', 'сорт', 'трос', 'торт', 'рост']
    )
)
