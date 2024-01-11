package leetcode_grind;

import java.util.Queue;

public class Day425 {
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

    // https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/description/
    static class Solution1 {
        int result = 0;

        void helper(TreeNode node, int curMax, int curMin) {
            if (node == null) {
                return;
            }

            int possibleResult = Math.max(
                    Math.abs(curMax - node.val), Math.abs(curMin - node.val));

            result = Math.max(result, possibleResult);

            curMax = Math.max(curMax, node.val);
            curMin = Math.min(curMin, node.val);

            helper(node.left, curMax, curMin);
            helper(node.right, curMax, curMin);
        }

        public int maxAncestorDiff(TreeNode root) {
            if (root == null) {
                return 0;
            }
            result = 0;
            helper(root, root.val, root.val);
            return result;
        }
    }

    class GfG {
        // Function to reverse first k elements of a queue.
        public Queue<Integer> modifyQueue(Queue<Integer> q, int k) {
            // add code here.
            int[] aux = new int[k];
            for (int i = 0; i < aux.length; i++) {
                aux[i] = q.poll();
            }
            int z = q.size();

            while (--k >= 0) {
                q.add(aux[k]);
            }

            while (z-- > 0) {
                int e = q.remove();
                q.add(e);
            }
            return q;
        }
    }
}
