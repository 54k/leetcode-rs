package leetcode_grind;

public class Day955 {
    // https://leetcode.com/problems/find-the-original-typed-string-i/description/?envType=daily-question&envId=2025-07-01
    static class Solution1 {
        public int possibleStringCount(String word) {
            int n = word.length(), ans = 1;
            for (int i = 1; i < n; i++) {
                if (word.charAt(i - 1) == word.charAt(i)) {
                    ans++;
                }
            }
            return ans;
        }
    }

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

    // https://leetcode.com/problems/largest-bst-subtree/description/
    static class Solution2 {
        static class NodeValue {
            int maxNode, minNode, maxSize;

            NodeValue(int minNode, int maxNode, int maxSize) {
                this.maxNode = maxNode;
                this.minNode = minNode;
                this.maxSize = maxSize;
            }
        }

        public NodeValue largestBSTSubtreeHelper(TreeNode root) {
            if (root == null) {
                return new NodeValue(Integer.MAX_VALUE, Integer.MIN_VALUE, 0);
            }

            NodeValue left = largestBSTSubtreeHelper(root.left);
            NodeValue right = largestBSTSubtreeHelper(root.right);

            if (left.maxNode < root.val && root.val < right.minNode) {
                return new NodeValue(Math.min(root.val, left.minNode), Math.max(root.val, right.maxNode),
                        left.maxSize + right.maxSize + 1);
            }

            return new NodeValue(Integer.MIN_VALUE, Integer.MAX_VALUE, Math.max(left.maxSize, right.maxSize));
        }

        public int largestBSTSubtree(TreeNode root) {
            return largestBSTSubtreeHelper(root).maxSize;
        }
    }
}
