# https://leetcode.com/problems/balance-a-binary-search-tree/description/
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def balanceBST(self, root: TreeNode) -> TreeNode:
        arr = []
        heights = {None: 0}
        def inorder(r: TreeNode):
            if r is None:
                return
            inorder(r.left)
            arr.append(r.val)
            inorder(r.right)
        inorder(root) 

        def rotate_left(node):
            w = node.right
            node.right = w.left
            w.left = node
            heights[node] = max(heights[node.left], heights[node.right]) + 1
            heights[w] = max(heights[w.left], heights[w.right]) + 1
            return w

        def rotate_right(node):
            w = node.left
            node.left = w.right
            w.right = node
            heights[node] = max(heights[node.left], heights[node.right]) + 1
            heights[w] = max(heights[w.left], heights[w.right]) + 1
            return w

        def big_rotate_left(node):
            node.right = rotate_right(node.right)
            return rotate_left(node)

        def big_rotate_right(node):
            node.left = rotate_left(node.left)
            return rotate_right(node)
        
        def rebalance(node):
            diff = heights[node.left] - heights[node.right]
            if abs(diff) <= 1:
                heights[node] = max(heights[node.left], heights[node.right]) + 1
            elif diff == -2:
                if heights[node.right.left] - heights[node.right.right] <= 0:
                    node = rotate_left(node)
                else:
                    node = big_rotate_left(node)
            elif diff == 2:
                if heights[node.left.left] - heights[node.left.right] >= 0:
                    node = rotate_right(node)
                else:
                    node = big_rotate_right(node)
            return node

        new_root = None
        def insert(node, val):
            if node is None:
                tn = TreeNode(val)
                heights[tn] = 1
                return tn
            if node.val > val:
                node.left = insert(node.left, val)
            else:
                node.right = insert(node.right, val)
            return rebalance(node)
        
        for val in arr:
            new_root = insert(new_root, val)

        return new_root