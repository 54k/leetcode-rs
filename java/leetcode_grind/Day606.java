package leetcode_grind;

import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

public class Day606 {
    // https://leetcode.com/problems/create-binary-tree-from-descriptions/description/?envType=daily-question&envId=2024-07-15
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
        public TreeNode createBinaryTree(int[][] descriptions) {
            Map<Integer, TreeNode> nodeMap = new HashMap<>();
            Set<Integer> children = new HashSet<>();

            for (int[] description : descriptions) {
                int parentValue = description[0];
                int childValue = description[1];
                boolean isLeft = description[2] == 1;

                if (!nodeMap.containsKey(parentValue)) {
                    nodeMap.put(parentValue, new TreeNode(parentValue));
                }
                if (!nodeMap.containsKey(childValue)) {
                    nodeMap.put(childValue, new TreeNode(childValue));
                }

                if (isLeft) {
                    nodeMap.get(parentValue).left = nodeMap.get(childValue);
                } else {
                    nodeMap.get(parentValue).right = nodeMap.get(childValue);
                }

                children.add(childValue);
            }

            for (TreeNode node : nodeMap.values()) {
                if (!children.contains(node.val)) {
                    return node;
                }
            }
            return null;
        }
    }

}
