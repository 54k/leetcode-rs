package leetcode_grind;

import java.util.LinkedList;
import java.util.Queue;

public class Day522 {
    static class Pair<F, S> {
        F key;
        S value;

        public Pair(F f1, S s1) {
            key = f1;
            value = s1;
        }

        F getKey() {
            return key;
        }

        S getValue() {
            return value;
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

    // https://leetcode.com/problems/smallest-string-starting-from-leaf/description
    static class Solution1 {
        public String smallestFromLeaf(TreeNode root) {
            String smallestString = "";
            Queue<Pair<TreeNode, String>> nodeQueue = new LinkedList<>();

            nodeQueue.add(new Pair<>(root, String.valueOf((char) (root.val + 'a'))));

            while (!nodeQueue.isEmpty()) {
                Pair<TreeNode, String> pair = nodeQueue.poll();
                TreeNode node = pair.getKey();
                String currString = pair.getValue();

                if (node.left == null && node.right == null) {
                    if (smallestString.isEmpty()) {
                        smallestString = currString;
                    } else {
                        smallestString = currString.compareTo(smallestString) < 0 ? currString : smallestString;
                    }
                }

                if (node.left != null) {
                    nodeQueue.add(new Pair<>(node.left, (char) (node.left.val + 'a') + currString));
                }
                if (node.right != null) {
                    nodeQueue.add(new Pair<>(node.right, (char) (node.right.val + 'a') + currString));
                }
            }

            return smallestString;
        }
    }

}
