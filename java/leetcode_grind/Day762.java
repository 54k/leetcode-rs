package leetcode_grind;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;

public class Day762 {
    // https://leetcode.com/problems/reverse-odd-levels-of-binary-tree/description/?envType=daily-question&envId=2024-12-20
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

    static class Solution1 {
        public TreeNode reverseOddLevels(TreeNode root) {
            if (root == null) {
                return null;
            }
            Queue<TreeNode> queue = new LinkedList<>();
            queue.add(root);
            int level = 0;

            while (!queue.isEmpty()) {
                int size = queue.size();
                List<TreeNode> currentLevelNodes = new ArrayList<>();

                for (int i = 0; i < size; i++) {
                    TreeNode node = queue.poll();
                    currentLevelNodes.add(node);
                    if (node.left != null)
                        queue.add(node.left);
                    if (node.right != null)
                        queue.add(node.right);
                }

                if (level % 2 == 1) {
                    int left = 0, right = currentLevelNodes.size() - 1;
                    while (left < right) {
                        int temp = currentLevelNodes.get(left).val;
                        currentLevelNodes.get(left).val = currentLevelNodes.get(right).val;
                        currentLevelNodes.get(right).val = temp;
                        left++;
                        right--;
                    }
                }
                level++;
            }
            return root;
        }
    }

    static class Solution2 {
        public TreeNode reverseOddLevels(TreeNode root) {
            traverseDFS(root.left, root.right, 0);
            return root;
        }

        void traverseDFS(
                TreeNode leftChild,
                TreeNode rightChild,
                int level) {
            if (leftChild == null || rightChild == null) {
                return;
            }
            if (level % 2 == 0) {
                int temp = leftChild.val;
                leftChild.val = rightChild.val;
                rightChild.val = temp;
            }
            traverseDFS(leftChild.left, rightChild.right, level + 1);
            traverseDFS(leftChild.right, rightChild.left, level + 1);
        }

    }
}
