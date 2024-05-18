# https://leetcode.com/problems/sum-of-distances-in-tree/description/
class Solution1:
    def sumOfDistancesInTree(self, n: int, edges: List[List[int]]) -> List[int]:
        graph = collections.defaultdict(set)
        for u, v in edges:
            graph[u].add(v)
            graph[v].add(u)
        
        count = [1] * n
        ans = [0] * n

        def dfs(node = 0, parent = None):
            for child in graph[node]:
                if child != parent:
                    dfs(child, node)
                    count[node] += count[child]
                    ans[node] += ans[child] + count[child]
        
        def dfs2(node = 0, parent = None):
            for child in graph[node]:
                if child != parent:
                    ans[child] = ans[node] - count[child] + n - count[child]
                    dfs2(child, node)
        
        dfs()
        dfs2()
        return ans

#  https://leetcode.com/problems/binary-tree-cameras/description/
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution2:
    def minCameraCover(self, root: Optional[TreeNode]) -> int:
        ans = 0
        covered = {None}

        def dfs(node, par=None):
            nonlocal ans
            if node:
                dfs(node.left, node)
                dfs(node.right, node)

                if par is None and node not in covered or \
                        node.left not in covered or node.right not in covered:
                    ans += 1
                    covered.update({node, par, node.left, node.right})

        dfs(root)
        return ans
