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