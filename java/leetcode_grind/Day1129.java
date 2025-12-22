package leetcode_grind;

import java.util.Arrays;

public class Day1129 {
    // https://leetcode.com/problems/delete-columns-to-make-sorted-iii/description/?envType=daily-question&envId=2025-12-22
    static class Solution1 {
        public int minDeletionSize(String[] A) {
            int W = A[0].length();
            int[] dp = new int[W];
            Arrays.fill(dp, 1);
            for (int i = W - 2; i >= 0; i--) {
                search: for (int j = i + 1; j < W; j++) {
                    for (String row : A) {
                        if (row.charAt(i) > row.charAt(j)) {
                            continue search;
                        }
                    }

                    dp[i] = Math.max(dp[i], 1 + dp[j]);
                }
            }

            int kept = 0;
            for (int x : dp) {
                kept = Math.max(kept, x);
            }
            return W - kept;
        }
    }

    // https://leetcode.com/problems/binary-tree-longest-consecutive-sequence-ii/description/?envType=weekly-question&envId=2025-12-22
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

    static class Solution2 {
        int maxval = 0;

        public int longestConsecutive(TreeNode root) {
            longestPath(root);
            return maxval;
        }

        int[] longestPath(TreeNode root) {
            if (root == null) {
                return new int[] { 0, 0 };
            }

            int inr = 1, dcr = 1;
            if (root.left != null) {
                int[] left = longestPath(root.left);
                if (root.val == root.left.val + 1) {
                    dcr = left[1] + 1;
                } else if (root.val == root.left.val - 1) {
                    inr = left[0] + 1;
                }
            }

            if (root.right != null) {
                int[] right = longestPath(root.right);
                if (root.val == root.right.val + 1) {
                    dcr = Math.max(dcr, right[1] + 1);
                } else if (root.val == root.right.val - 1) {
                    inr = Math.max(inr, right[0] + 1);
                }
            }
            maxval = Math.max(maxval, dcr + inr - 1);
            return new int[] { inr, dcr };
        }
    }
}
