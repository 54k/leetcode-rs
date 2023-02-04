"""
Даны даты заезда и отъезда каждого гостя.
Необходимо написать функцию, которая считает максимальное число посетителей, которые одновременно проживали в гостинице.

Ограничение по сложности — строго меньше O(N^2)
"""
from pprint import pprint


def max_guests(segments):
    events = []
    for s in segments:
        events.append(
            (s[0], 1)  # 1 == въезжают
        )
        events.append(
            (s[1], 0)  # 0 == выезжают
        )
    events.sort(
        key=lambda e: (e[0], e[1]))  # сначала сортируем по времени, потом по типу ивента, выезжают раньше чем въезжают
    # events.sort()

    current_count = 0
    max_count = 0
    for e in events:
        if e[1] == 1:
            current_count += 1
        else:
            current_count -= 1
        max_count = max(current_count, max_count)
    return max_count


pprint(
    max_guests(
        []
    )
)

pprint(
    max_guests(
        [(1, 2)]
    )
)

pprint(
    max_guests(
        [(1, 2), (2, 3)]
    )
)

pprint(
    max_guests(
        [(1, 5), (0, 1), (4, 5)]
    )
)
