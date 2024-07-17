package leetcode_grind;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;
import java.util.Set;

public class Day608 {
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

    // https://leetcode.com/problems/delete-nodes-and-return-forest/description/?envType=daily-question&envId=2024-07-17
    static class Solution1 {
        public List<TreeNode> delNodes(TreeNode root, int[] to_delete) {
            Set<Integer> toDeleteSet = new HashSet<>();
            for (int val : to_delete) {
                toDeleteSet.add(val);
            }
            List<TreeNode> forest = new ArrayList<>();
            root = processNode(root, toDeleteSet, forest);
            if (root != null) {
                forest.add(root);
            }
            return forest;
        }

        TreeNode processNode(TreeNode node, Set<Integer> toDeleteSet, List<TreeNode> forest) {
            if (node == null)
                return null;
            node.left = processNode(node.left, toDeleteSet, forest);
            node.right = processNode(node.right, toDeleteSet, forest);

            if (toDeleteSet.contains(node.val)) {
                if (node.left != null) {
                    forest.add(node.left);
                }
                if (node.right != null) {
                    forest.add(node.right);
                }
                return null;
            }
            return node;
        }
    }

    static class Solution2 {
        public List<TreeNode> delNodes(TreeNode root, int[] to_delete) {
            if (root == null) {
                return new ArrayList<>();
            }
            Set<Integer> toDeleteSet = new HashSet<>();
            for (int val : to_delete) {
                toDeleteSet.add(val);
            }
            List<TreeNode> forest = new ArrayList<>();
            Queue<TreeNode> nodesQueue = new LinkedList<>();
            nodesQueue.add(root);

            while (!nodesQueue.isEmpty()) {
                TreeNode currentNode = nodesQueue.poll();

                if (currentNode.left != null) {
                    nodesQueue.add(currentNode.left);
                    if (toDeleteSet.contains(currentNode.left.val)) {
                        currentNode.left = null;
                    }
                }
                if (currentNode.right != null) {
                    nodesQueue.add(currentNode.right);
                    if (toDeleteSet.contains(currentNode.right.val)) {
                        currentNode.right = null;
                    }
                }

                if (toDeleteSet.contains(currentNode.val)) {
                    if (currentNode.left != null) {
                        forest.add(currentNode.left);
                    }
                    if (currentNode.right != null) {
                        forest.add(currentNode.right);
                    }
                }
            }

            if (!toDeleteSet.contains(root.val)) {
                forest.add(root);
            }

            return forest;
        }
    }
}
