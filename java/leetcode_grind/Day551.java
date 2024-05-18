package leetcode_grind;

public class Day551 {
    public static class TreeNode {
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

    // https://leetcode.com/problems/distribute-coins-in-binary-tree/description
    static class Solution1 {
        public int distributeCoins(TreeNode root) {

            var dfs = new Object() {
                int ans = 0;

                int apply(TreeNode node) {
                    if (node == null) {
                        return 0;
                    }
                    var l = apply(node.left);
                    var r = apply(node.right);
                    ans += Math.abs(l) + Math.abs(r);
                    return node.val + l + r - 1;
                }

            };

            dfs.apply(root);
            return dfs.ans;
        }
    }
}