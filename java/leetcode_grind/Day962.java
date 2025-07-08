package leetcode_grind;

import java.util.Arrays;

public class Day962 {
    // https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended-ii/description/?envType=daily-question&envId=2025-07-08
    static class Solution1 {
        public int maxValue(int[][] events, int k) {
            Arrays.sort(events, (a, b) -> a[0] - b[0]);
            int n = events.length;

            dp = new int[k + 1][n];
            for (int[] row : dp) {
                Arrays.fill(row, -1);
            }

            return dfs(0, k, events);
        }

        private int[][] dp;

        private int dfs(int curIndex, int count, int[][] events) {
            if (count == 0 || curIndex == events.length) {
                return 0;
            }
            if (dp[count][curIndex] != -1) {
                return dp[count][curIndex];
            }

            int nextIndex = bisectRight(events, events[curIndex][1]);

            dp[count][curIndex] = Math.max(dfs(curIndex + 1, count, events),
                    events[curIndex][2] + dfs(nextIndex, count - 1, events));

            return dp[count][curIndex];
        }

        static int bisectRight(int[][] events, int target) {
            int left = 0, right = events.length;
            while (left < right) {
                int mid = (left + right) / 2;
                if (events[mid][0] <= target) {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            return left;
        }
    }

    static class Solution2 {
        public int maxValue(int[][] events, int k) {
            int n = events.length;
            int[][] dp = new int[k + 1][n + 1];
            Arrays.sort(events, (a, b) -> a[0] - b[0]);

            for (int curIndex = n - 1; curIndex >= 0; --curIndex) {
                for (int count = 1; count <= k; count++) {
                    int nextIndex = bisectRight(events, events[curIndex][1]);
                    dp[count][curIndex] = Math.max(dp[count][curIndex + 1],
                            events[curIndex][2] + dp[count - 1][nextIndex]);
                }
            }
            return dp[k][0];
        }

        static int bisectRight(int[][] events, int target) {
            int left = 0, right = events.length;
            while (left < right) {
                int mid = (left + right) / 2;
                if (events[mid][0] <= target) {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            return left;
        }
    }

    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode(int x) {
            val = x;
        }
    }

    // https://leetcode.com/problems/inorder-successor-in-bst/description/?envType=weekly-question&envId=2025-07-08
    static class Solution3 {

        TreeNode previous;
        TreeNode inorderSuccessorNode;

        public TreeNode inorderSuccessor(TreeNode root, TreeNode p) {
            if (p.right != null) {
                TreeNode leftmost = p.right;
                while (leftmost.left != null) {
                    leftmost = leftmost.left;
                }
                this.inorderSuccessorNode = leftmost;
            } else {
                this.inorderCase2(root, p);
            }
            return this.inorderSuccessorNode;
        }

        void inorderCase2(TreeNode node, TreeNode p) {
            if (node == null) {
                return;
            }
            this.inorderCase2(node.left, p);

            if (this.previous == p && this.inorderSuccessorNode == null) {
                this.inorderSuccessorNode = node;
                return;
            }
            this.previous = node;

            this.inorderCase2(node.right, p);
        }
    }

    static class Solution4 {
        public TreeNode inorderSuccessor(TreeNode root, TreeNode p) {
            TreeNode successor = null;
            while (root != null) {
                if (p.val >= root.val) {
                    root = root.right;
                } else {
                    successor = root;
                    root = root.left;
                }
            }
            return successor;
        }
    }
}
