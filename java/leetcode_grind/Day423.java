package leetcode_grind;

import java.util.ArrayList;
import java.util.List;
import java.util.PriorityQueue;

public class Day423 {
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

    // https://leetcode.com/problems/leaf-similar-trees/description
    static class Solution1 {
        public boolean leafSimilar(TreeNode root1, TreeNode root2) {
            var rec = new Object() {
                int apply(TreeNode root, List<Integer> leaves) {
                    if (root == null) {
                        return -1;
                    }
                    var left = apply(root.left, leaves);
                    var right = apply(root.right, leaves);
                    var h = 1 + Math.max(left, right);
                    if (h == 0) {
                        leaves.add(root.val);
                    }
                    return h;
                }
            };

            var left = new ArrayList<Integer>();
            var right = new ArrayList<Integer>();

            rec.apply(root1, left);
            rec.apply(root2, right);

            return left.equals(right);
        }
    }

    // https://leetcode.com/problems/closest-binary-search-tree-value-ii/description/
    static class Solution2 {
        public List<Integer> closestKValues(TreeNode root, double target, int k) {
            var heap = new PriorityQueue<Integer>(
                    (a, b) -> Math.abs((double) a - target) > Math.abs((double) b - target) ? -1 : 1);

            var dfs = new Object() {
                void apply(TreeNode node) {
                    if (node == null) {
                        return;
                    }

                    heap.add(node.val);
                    if (heap.size() > k) {
                        heap.remove();
                    }

                    apply(node.left);
                    apply(node.right);
                }
            };

            dfs.apply(root);
            return new ArrayList<>(heap);
        }
    }

}
