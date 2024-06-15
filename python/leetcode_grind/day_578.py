# https://leetcode.com/problems/ipo/description/
class Solution1:
    def findMaximizedCapital(self, k: int, w: int, profits: List[int], capital: List[int]) -> int:
        n = len(profits)
        projects = list(zip(capital, profits))
        projects.sort()
        q = []
        ptr = 0
        for i in range(k):
            while ptr < n and projects[ptr][0] <= w:
                heappush(q, -projects[ptr][1])
                ptr += 1
            if not q:
                break
            w += -heappop(q)
        return w

# https://leetcode.com/problems/put-boxes-into-the-warehouse-i/description/
class Solution2:
    def maxBoxesInWarehouse(self, boxes: List[int], warehouse: List[int]) -> int:
        i = 0
        count = 0
        boxes.sort(reverse = True)
        for room in warehouse:
            while i < len(boxes) and boxes[i] > room:
                i += 1
            if i == len(boxes):
                return count
            count += 1
            i += 1
        return count