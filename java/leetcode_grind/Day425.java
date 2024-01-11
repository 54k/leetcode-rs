package leetcode_grind;

import java.util.ArrayList;
import java.util.List;
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

    // https://leetcode.com/problems/spiral-matrix/description/
    static class Solution2 {
        public List<Integer> spiralOrder(int[][] matrix) {
            int up = 0, down = matrix.length - 1;
            int left = 0, right = matrix[0].length - 1;

            List<Integer> result = new ArrayList<>();

            while (result.size() < matrix.length * matrix[0].length) {
                for (int col = left; col <= right; col++) {
                    result.add(matrix[up][col]);
                }

                for (int row = up + 1; row <= down; row++) {
                    result.add(matrix[row][right]);
                }

                if (up != down) {
                    for (int col = right - 1; col >= left; col--) {
                        result.add(matrix[down][col]);
                    }
                }

                if (left != right) {
                    for (int row = down - 1; row > up; row--) {
                        result.add(matrix[row][left]);
                    }
                }

                left++;
                right--;
                up++;
                down--;
            }

            return result;
        }
    }
}
