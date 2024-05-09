# https://leetcode.com/problems/shortest-cycle-in-a-graph/description/
class Solution:
    def findShortestCycle(self, n: int, edges: List[List[int]]) -> int:
        adj = [[] for _ in range(n)]    
        for e in edges:
            adj[e[0]].append(e[1])
            adj[e[1]].append(e[0])
        min_cycle = n + 1

        for i in range(n):
            dist = [-1] * n
            dist[i] = 0

            q = [i]
            front = 0

            while front < len(q):
                u = q[front]
                front += 1
                for v in adj[u]:
                    if dist[v] == -1:
                        dist[v] = dist[u] + 1
                        q.append(v)
                    elif dist[v] >= dist[u]:
                        min_cycle = min(min_cycle, dist[v] + dist[u] + 1)
        return min_cycle if min_cycle <= n else -1