# https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/description
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution1:
    def maxAncestorDiff(self, root: Optional[TreeNode]) -> int:
        def dfs(node, mn, mx):
            if node == None:
                return mx - mn

            mn = min(mn, node.val)
            mx = max(mx, node.val)

            l = dfs(node.left, mn, mx)
            r = dfs(node.right, mn, mx)
            return max(l, r)

        if root == None:
            return 0
        return dfs(root, root.val, root.val)

        
# https://www.geeksforgeeks.org/problems/remove-k-digits/1
class Solution2:
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