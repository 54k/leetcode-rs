# https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/description/
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None

class Solution:
    def distanceK(self, root: TreeNode, target: TreeNode, k: int) -> List[int]:
        def add_parent(cur, parent):
            if cur:
                cur.parent = parent
                add_parent(cur.left, cur)
                add_parent(cur.right, cur)

        add_parent(root, None)

        answer = []
        visited = set()

        def dfs(cur, distance):
            if not cur or cur in visited:
                return

            visited.add(cur)

            if distance == 0:
                answer.append(cur.val)
                return

            dfs(cur.parent, distance - 1)
            dfs(cur.left, distance - 1)
            dfs(cur.right, distance - 1)

        dfs(target, k)
        return answer

#User function Template for python3

class Solution:

    def removeKdigits(self, S, K):
        # code here
        st = []
        for ch in S:
            while len(st) > 0 and K > 0 and st[-1] > ch:
                st.pop()
                K -= 1

            if not len(st) and ch == '0':
                continue
            st.append(ch)
        
        while len(st) and K > 0:
            st.pop()
            K -= 1
        
        return "0" if len(st) == 0 else "".join(st)
        
