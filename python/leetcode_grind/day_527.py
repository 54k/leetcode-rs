# https://leetcode.com/problems/number-of-islands-ii/description
class Solution:
    def numIslands2(self, m: int, n: int, positions: List[List[int]]) -> List[int]:
        class UF:
            def __init__(self, n):
                self.repr = [-1] * n
                self.sz = [1] * n
                self.cnt = 0

            def find(self, x):
                if self.repr[x] == -1:
                    return -1
                if self.repr[x] is not x:
                    self.repr[x] = self.find(self.repr[x])
                return self.repr[x]

            def union(self, x, y):
                px, py = self.find(x), self.find(y)
                if px == py or px == -1 or py == -1:
                    return
                if self.sz[px] < self.sz[py]:
                    self.repr[px] = py
                    self.sz[py] += self.sz[px]
                else:
                    self.repr[py] = px
                    self.sz[px] += self.sz[py]
                self.cnt -= 1
            
            def add(self, x):
                if self.find(x) == -1:
                    self.repr[x] = x
                    self.cnt += 1
        ans = [] 
        uf = UF(m*n)
        for pos in positions:
            v = pos[0]*n+pos[1]
            uf.add(v)
            for d in [[-1,0],[1,0],[0,-1],[0,1]]:
                ux, uy = pos[0]+d[0], pos[1]+d[1]
                if 0 <= ux < m and 0 <= uy < n:
                    u = ux*n+uy
                    uf.union(v, u)
            ans.append(uf.cnt)
        return ans