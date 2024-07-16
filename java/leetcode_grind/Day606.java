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

    static class Solution3 {
        public TreeNode createBinaryTree(int[][] descriptions) {
            Map<Integer, List<int[]>> parentToChildren = new HashMap<>();
            Set<Integer> allNodes = new HashSet<>();
            Set<Integer> children = new HashSet<>();

            for (int[] desc : descriptions) {
                int parent = desc[0];
                int child = desc[1];
                int isLeft = desc[2];

                if (!parentToChildren.containsKey(parent)) {
                    parentToChildren.put(parent, new ArrayList<>());
                }
                parentToChildren.get(parent).add(new int[] { child, isLeft });
                allNodes.add(parent);
                allNodes.add(child);
                children.add(child);
            }

            int rootVal = 0;
            for (int node : allNodes) {
                if (!children.contains(node)) {
                    rootVal = node;
                    break;
                }
            }

            return dfs(parentToChildren, rootVal);
        }

        TreeNode dfs(Map<Integer, List<int[]>> parentToChildren, int val) {
            TreeNode node = new TreeNode(val);
            if (parentToChildren.containsKey(val)) {
                for (int[] childInfo : parentToChildren.get(val)) {
                    int child = childInfo[0];
                    int isLeft = childInfo[1];

                    if (isLeft == 1) {
                        node.left = dfs(parentToChildren, child);
                    } else {
                        node.right = dfs(parentToChildren, child);
                    }
                }
            }
            return node;
        }
    }
}
