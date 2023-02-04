# Есть два ступенчатых графика некоторых наблюдаемых величин,
# заданных отсортированными списками координат начала ступенек (<время измерения>, <значение>).
# Подразумевается, что величина сохраняет своё значение между измерениями.
# Все времена измерений в обоих графиках различны.


from pprint import pprint


def charts_sum(chart1, chart2):
    chart1_len = len(chart1)
    chart2_len = len(chart2)

    result = []

    i = 0
    j = 0

    last_1 = 0
    last_2 = 0

    while True:
        if i < chart1_len and j < chart2_len and chart1[i][0] == chart2[j][0]:
            result.append(
                [chart1[i][0], chart1[i][1] + chart2[i][1]]
            )
            last_1 = chart1[i][1]
            last_2 = chart2[i][1]
            i += 1
            j += 1
        elif j >= chart2_len or (i < chart1_len and chart1[i][0] < chart2[j][0]):
            result.append(
                [chart1[i][0], chart1[i][1] + last_2]
            )
            last_1 = chart1[i][1]
            i += 1
        else:
            result.append(
                [chart2[j][0], chart2[j][1] + last_1]
            )
            last_2 = chart2[i][1]
            j += 1

        if i == chart1_len and j == chart2_len:
            break

    return result


pprint(
    charts_sum(
        [[1, 2], [4, 1]],
        [[2, 4], [3, 6], [5, 7]]
    )
)
