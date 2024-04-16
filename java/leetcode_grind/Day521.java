package leetcode_grind;

import java.util.HashSet;
import java.util.Set;

public class Day521 {
    // https://leetcode.com/problems/binary-tree-cameras/description/
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

    static class Solution1 {
        int ans;
        Set<TreeNode> covered;

        public int minCameraCover(TreeNode root) {
            ans = 0;
            covered = new HashSet<>();
            covered.add(null);
            dfs(root, null);
            return ans;
        }

        public void dfs(TreeNode node, TreeNode par) {
            if (node == null) {
                return;
            }

            dfs(node.left, node);
            dfs(node.right, node);

            if (par == null && !covered.contains(node) || !covered.contains(node.left)
                    || !covered.contains(node.right)) {
                ans++;
                covered.add(node);
                covered.add(par);
                covered.add(node.left);
                covered.add(node.right);
            }
        }
    }

}
