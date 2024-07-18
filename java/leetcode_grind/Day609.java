package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;
import java.util.Set;

public class Day609 {
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

    static class Solution1 {
        public int countPairs(TreeNode root, int distance) {
            Map<TreeNode, List<TreeNode>> graph = new HashMap<>();
            Set<TreeNode> leafNodes = new HashSet<>();
            traverseTree(root, null, graph, leafNodes);
            int ans = 0;

            for (TreeNode leaf : leafNodes) {
                Queue<TreeNode> bfsQueue = new LinkedList<>();
                Set<TreeNode> seen = new HashSet<>();
                bfsQueue.add(leaf);
                seen.add(leaf);

                for (int i = 0; i <= distance; i++) {
                    int size = bfsQueue.size();
                    for (int j = 0; j < size; j++) {
                        TreeNode currNode = bfsQueue.remove();
                        if (leafNodes.contains(currNode) && currNode != leaf) {
                            ans++;
                        }
                        if (graph.containsKey(currNode)) {
                            for (TreeNode neighbor : graph.get(currNode)) {
                                if (!seen.contains(neighbor)) {
                                    bfsQueue.add(neighbor);
                                    seen.add(neighbor);
                                }
                            }
                        }
                    }
                }
            }

            return ans / 2;
        }

        void traverseTree(TreeNode currNode, TreeNode prevNode, Map<TreeNode, List<TreeNode>> graph,
                Set<TreeNode> leafNodes) {
            if (currNode == null)
                return;
            if (currNode.left == null && currNode.right == null) {
                leafNodes.add(currNode);
            }

            if (prevNode != null) {
                graph.computeIfAbsent(prevNode, k -> new ArrayList<TreeNode>());
                graph.get(prevNode).add(currNode);

                graph.computeIfAbsent(currNode, k -> new ArrayList<TreeNode>());
                graph.get(currNode).add(prevNode);
            }

            traverseTree(currNode.left, currNode, graph, leafNodes);
            traverseTree(currNode.right, currNode, graph, leafNodes);
        }
    }
}
