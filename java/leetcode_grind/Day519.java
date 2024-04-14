package leetcode_grind;

public class Day519 {
    public class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode() {
        }

        TreeNode(int val) {
            this.val = val;
        }

        TreeNode(int val, TreeNode left, TreeNode right) {
            this.val = val;
            this.left = left;
            this.right = right;
        }
    }

    // https://leetcode.com/problems/minimum-distance-between-bst-nodes/description/
    static class Solution1 {
        public int minDiffInBST(TreeNode root) {
            var ans = Integer.MAX_VALUE;
            TreeNode prev = null;
            var curr = root;

            while (curr != null) {
                if (curr.left == null) {
                    if (prev != null) {
                        ans = Math.min(ans, curr.val - prev.val);
                    }
                    prev = curr;
                    curr = curr.right;
                } else {
                    var pre = curr.left;
                    while (pre.right != null) {
                        pre = pre.right;
                    }
                    pre.right = curr;
                    var t = curr;
                    curr = curr.left;
                    t.left = null;
                }
            }

            return ans;
        }
    }
}
