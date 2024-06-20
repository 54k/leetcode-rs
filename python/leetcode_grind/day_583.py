# https://leetcode.com/problems/design-memory-allocator/description/
class Allocator:

    def __init__(self, n: int):
        self.arr = [0] * n
        

    def allocate(self, size: int, mID: int) -> int:
        cnt = 0
        for i in range(len(self.arr)):
            if self.arr[i] == 0:
                cnt+=1
                if cnt == size:
                    for j in range(i, i - cnt, -1):
                        self.arr[j] = mID
                    return i-cnt+1
            else:
                cnt = 0
        return -1

    def free(self, mID: int) -> int:
        cnt = 0
        for i in range(len(self.arr)):
            if self.arr[i] == mID:
                self.arr[i] = 0
                cnt += 1
        return cnt
        


# Your Allocator object will be instantiated and called as such:
# obj = Allocator(n)
# param_1 = obj.allocate(size,mID)
# param_2 = obj.free(mID)

# https://leetcode.com/problems/magnetic-force-between-two-balls/description/
class Solution1:
    def maxDistance(self, position: List[int], m: int) -> int:
        def can_place_balls(x, position, m):
            prev_ball_pos = position[0]
            balls_placed = 1
            for i in range(1, len(position)):
                curr_pos = position[i]
                if curr_pos - prev_ball_pos >= x:
                    balls_placed += 1
                    prev_ball_pos = curr_pos
                if balls_placed == m:
                    return True
            return False
        
        answer = 0
        position.sort()

        low = 1
        high = int(position[-1] / (m - 1.0)) + 1
        while low <= high:
            mid = low + (high - low) // 2
            if can_place_balls(mid, position, m):
                answer = mid
                low = mid + 1
            else:
                high = mid - 1
        return answer