package leetcode_grind;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Day346 {
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

    // https://leetcode.com/problems/find-largest-value-in-each-tree-row/description
    static class Solution1 {
        public List<Integer> largestValues(TreeNode root) {
            class Pair {
                TreeNode node;
                Integer lvl;

                Pair(TreeNode n, Integer l) {
                    node = n;
                    lvl = l;
                }
            }

            var res = new ArrayList<Integer>();
            if (root == null)
                return res;

            var stack = new LinkedList<Pair>();
            stack.push(new Pair(root, 0));

            while (!stack.isEmpty()) {
                var pair = stack.pop();

                if (res.size() == pair.lvl) {
                    res.add(Integer.MIN_VALUE);
                }

                res.set(pair.lvl, Math.max(res.get(pair.lvl), pair.node.val));

                if (pair.node.right != null) {
                    stack.push(new Pair(pair.node.right, pair.lvl + 1));
                }

                if (pair.node.left != null) {
                    stack.push(new Pair(pair.node.left, pair.lvl + 1));
                }
            }

            return res;
        }
    }
}
