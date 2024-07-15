package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;
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

    static class Solution2 {
        public TreeNode createBinaryTree(int[][] descriptions) {
            Set<Integer> children = new HashSet<>();
            Set<Integer> parents = new HashSet<>();
            Map<Integer, List<int[]>> parentToChildren = new HashMap<>();

            for (int[] d : descriptions) {
                int parent = d[0], child = d[1], isLeft = d[2];
                parents.add(parent);
                parents.add(child);
                children.add(child);
                parentToChildren.computeIfAbsent(parent, l -> new ArrayList<>()).add(new int[] { child, isLeft });
            }

            parents.removeAll(children);
            TreeNode root = new TreeNode(parents.iterator().next());
            Queue<TreeNode> queue = new LinkedList<>();
            queue.offer(root);

            while (!queue.isEmpty()) {
                TreeNode parent = queue.poll();
                for (int[] childInfo : parentToChildren.getOrDefault(parent.val, Collections.emptyList())) {
                    int childValue = childInfo[0], isLeft = childInfo[1];
                    TreeNode child = new TreeNode(childValue);
                    queue.offer(child);
                    if (isLeft == 1) {
                        parent.left = child;
                    } else {
                        parent.right = child;
                    }
                }
            }

            return root;
        }
    }
}
