package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day1145 {
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

    // https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/description/?envType=daily-question&envId=2026-01-07
    static class Solution1 {
        List<Integer> allSums = new ArrayList<>();

        public int maxProduct(TreeNode root) {
            long totalSum = treeSum(root);
            long best = 0;
            for (long sum : allSums) {
                best = Math.max(best, sum * (totalSum - sum));
            }
            return (int) (best % 1000_000_007);
        }

        int treeSum(TreeNode subroot) {
            if (subroot == null)
                return 0;
            int leftSum = treeSum(subroot.left);
            int rightSum = treeSum(subroot.right);
            int totalSum = leftSum + rightSum + subroot.val;
            allSums.add(totalSum);
            return totalSum;
        }
    }

    static class Solution2 {
        long maximumProduct = 0;
        int totalTreeSum = 0;

        int treeSum(TreeNode subroot) {
            if (subroot == null)
                return 0;
            int leftSum = treeSum(subroot.left);
            int rightSum = treeSum(subroot.right);
            int totalSum = leftSum + subroot.val + rightSum;
            return totalSum;
        }

        int findMaximumProduct(TreeNode subroot) {
            if (subroot == null) {
                return 0;
            }
            int leftSum = findMaximumProduct(subroot.left);
            int rightSum = findMaximumProduct(subroot.right);
            int totalSum = leftSum + subroot.val + rightSum;
            long totalProduct = (long) totalSum * (totalTreeSum - totalSum);
            this.maximumProduct = Math.max(this.maximumProduct, totalProduct);
            return totalSum;
        }

        public int maxProduct(TreeNode root) {
            this.totalTreeSum = treeSum(root);
            findMaximumProduct(root);
            return (int) (maximumProduct % 1_000_000_007);
        }
    }
}
