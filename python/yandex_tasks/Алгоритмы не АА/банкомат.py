# Требуется выдать запрошенную сумму купюрами в рублях начиная от более крупных к более мелким.
# Купюры существуют 50 руб, 100 руб, 500 руб, 1000 руб, 5000 руб.
# При этом число купюр ограничено, на вход даётся объект с количеством купюр.
# Этот объект нужно менять, чтобы количество купюр всегда было в актуальном состоянии.
# Если выдать заданную сумму нельзя — вывести сообщение об ошибке.
# Не давать решать через покупюрный набор (можно приводить пример когда запрашивают миллион,
# а в банкомате есть только полтинники).


from pprint import pprint


def atm(s, limits):
    banknotes_values = sorted(limits.keys(), reverse=True)
    banknotes = {}
    result = []

    if s % banknotes_values[-1]:  # цель меньше самой мелкой купюры
        return 'Error: Incorrect value'

    for b in banknotes_values:
        desired = s // b

        if desired == 0:
            continue

        if limits[b] < desired:
            desired = limits[b]

        banknotes[b] = desired

        s -= desired * b

    if s != 0:
        return 'Error: Not enough money'

    for b in banknotes_values:
        if b not in banknotes:
            continue
        result.append(
            f'{banknotes[b]}x{b}'
        )
        limits[b] -= banknotes[b]

    return result


l = {
    1000: 6,
    500: 5,
    100: 5,
    50: 4
}

pprint(atm(1250, l))  # '1x1000 2x100 1x50'
pprint(atm(1000000, l))  # 'Error: Not enough money'
pprint(atm(2400, l))  # '2x1000 3x100 2x50'
pprint(atm(6512, l))  # 'Error: Incorrect value'
pprint(atm(50, l))  # '1x50'
pprint(atm(50, l))  # 'Error: Not enough money'
pprint(atm(5500, l))  # '3x1000 5x500'
