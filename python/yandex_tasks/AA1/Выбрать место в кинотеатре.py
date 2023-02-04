"""
Места в кинотеатре расположены в один ряд.
Только что пришедший зритель выбирает место, чтобы сидеть максимально далеко
от остальных зрителей в ряду. То есть расстояние от того места,
куда сядет зритель до ближайшего к нему зрителя должно быть максимально.

Гарантируется, что в ряду всегда есть свободные места и уже сидит хотя бы один зритель.
Напишите функцию, которая по заданному ряду мест (массиву из нулей и единиц)
вернёт расстояние от выбранного пришедшим зрителем места до другого ближайшего зрителя.
"""
from typing import List
from pprint import pprint


class Solution:
    def maxDistToClosest(self, seats: List[int]) -> int:
        max_free_seq = 0
        current_free_seq = 0

        first_seq = -1

        for i in range(len(seats)):
            s = seats[i]
            if s == 0:
                current_free_seq += 1
                max_free_seq = max(current_free_seq, max_free_seq)
            else:
                if first_seq == -1:
                    first_seq = current_free_seq
                current_free_seq = 0

        if max_free_seq % 2 == 1:
            max_free_seq += 1

        return max(max_free_seq // 2, first_seq, current_free_seq)


pprint(Solution.maxDistToClosest(1, [1, 1, 0, 0, 0, 0, 1, 0, 1]))
