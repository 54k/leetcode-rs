package leetcode_grind;

import java.util.List;
import java.util.ArrayList;
import java.util.Deque;
import java.util.ArrayDeque;
import java.util.Collections;

public class Day647 {
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

    // https://leetcode.com/problems/binary-tree-postorder-traversal/description/?envType=daily-question&envId=2024-08-25
    static class Solution1 {
        public List<Integer> postorderTraversal(TreeNode root) {
            List<Integer> result = new ArrayList<>();
            Deque<TreeNode> traversalStack = new ArrayDeque<>();
            TreeNode currentNode = root;
            while (currentNode != null || !traversalStack.isEmpty()) {
                if (currentNode != null) {
                    result.add(currentNode.val);
                    traversalStack.push(currentNode);
                    currentNode = currentNode.right;
                } else {
                    currentNode = traversalStack.pop();
                    currentNode = currentNode.left;
                }
            }
            Collections.reverse(result);
            return result;
        }
    }

}
