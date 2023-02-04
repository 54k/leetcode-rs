"""
У нас есть набор билетов вида:

[
    { 'from': 'London', 'to': 'Moscow' },
    { 'from': 'NY', 'to': 'London' },
    { 'from': 'Moscow', 'to': 'SPb' },
    ...
]

Из этих билетов можно построить единственный, неразрывный маршрут.
Петель и повторов в маршруте нет.

Нужно написать программу, которая возвращает билеты
в порядке следования по маршруту.
"""


#  map
def get_route(tickets):
    #  делаю мап ключ -- откуда, значение -- куда
    points = {}
    for ticket in tickets:
        points[ticket['from']] = ticket['to']

    #  единственный фром без ту -- старт
    start = list(set(points.keys()) - set(points.values()))[0]
    route = []

    while start in points:
        route.append(
            {
                'from': start,
                'to': points[start]
            }
        )
        start = points[start]

    return route


#  left look right look
def get_route(tickets):
    from_dict, to_dict = {}, {}
    for ticket in tickets:
        from_dict[ticket['from']] = ticket
        to_dict[ticket['to']] = ticket

    left_route, right_route = [], []

    ticket = tickets[0]

    while ticket:
        right_route.append(ticket)
        ticket = from_dict.get(ticket['to'])

    ticket = to_dict[tickets[0]['from']]

    while ticket:
        left_route.append(ticket)
        ticket = to_dict.get(ticket['from'])

    left_route.reverse()

    return left_route + right_route
