package leetcode_grind;

import java.util.*;

public class Day706 {
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

    // https://leetcode.com/problems/cousins-in-binary-tree-ii/description/?envType=daily-question&envId=2024-10-23
    static class Solution1 {
        public TreeNode replaceValueInTree(TreeNode root) {
            if (root == null) {
                return root;
            }
            Queue<TreeNode> nodeQueue = new LinkedList<>();
            nodeQueue.offer(root);
            int previousLevelSum = root.val;
            while (!nodeQueue.isEmpty()) {
                int levelSize = nodeQueue.size();
                int currentLevelSum = 0;
                for (int i = 0; i < levelSize; i++) {
                    TreeNode currentNode = nodeQueue.poll();
                    currentNode.val = previousLevelSum - currentNode.val;
                    int siblingSum = (currentNode.left != null ? currentNode.left.val : 0) +
                            (currentNode.right != null ? currentNode.right.val : 0);

                    if (currentNode.left != null) {
                        currentLevelSum += currentNode.left.val;
                        currentNode.left.val = siblingSum;
                        nodeQueue.offer(currentNode.left);
                    }
                    if (currentNode.right != null) {
                        currentLevelSum += currentNode.right.val;
                        currentNode.right.val = siblingSum;
                        nodeQueue.offer(currentNode.right);
                    }
                }
                previousLevelSum = currentLevelSum;
            }
            return root;
        }
    }

    static class Solution2 {
        int[] levelSums = new int[100000];

        public TreeNode replaceValueInTree(TreeNode root) {
            calculateLevelSum(root, 0);
            replaceValueInTreeInternal(root, 0, 0);
            return root;
        }

        void calculateLevelSum(TreeNode node, int level) {
            if (node == null) {
                return;
            }
            levelSums[level] += node.val;
            calculateLevelSum(node.left, level + 1);
            calculateLevelSum(node.right, level + 1);
        }

        void replaceValueInTreeInternal(TreeNode node, int siblingSum, int level) {
            if (node == null) {
                return;
            }
            int leftChildVal = (node.left == null) ? 0 : node.left.val;
            int rightChildVal = (node.right == null) ? 0 : node.right.val;
            if (level == 0 || level == 1) {
                node.val = 0;
            } else {
                node.val = levelSums[level] - node.val - siblingSum;
            }
            replaceValueInTreeInternal(node.left, rightChildVal, level + 1);
            replaceValueInTreeInternal(node.right, leftChildVal, level + 1);
        }
    }

    static class Solution3 {
        public TreeNode replaceValueInTree(TreeNode root) {
            if (root == null)
                return root;
            Queue<TreeNode> nodeQueue = new LinkedList<>();
            nodeQueue.offer(root);
            List<Integer> levelSums = new ArrayList<>();

            while (!nodeQueue.isEmpty()) {
                int levelSum = 0;
                int levelSize = nodeQueue.size();
                for (int i = 0; i < levelSize; i++) {
                    TreeNode currentNode = nodeQueue.poll();
                    levelSum += currentNode.val;
                    if (currentNode.left != null)
                        nodeQueue.offer(currentNode.left);
                    if (currentNode.right != null)
                        nodeQueue.offer(currentNode.right);
                }
                levelSums.add(levelSum);
            }

            nodeQueue.offer(root);
            int levelIndex = 1;
            root.val = 0;
            while (!nodeQueue.isEmpty()) {
                int levelSize = nodeQueue.size();
                for (int i = 0; i < levelSize; ++i) {
                    TreeNode currentNode = nodeQueue.poll();
                    int siblingSum = (currentNode.left != null ? currentNode.left.val : 0) +
                            (currentNode.right != null ? currentNode.right.val : 0);

                    if (currentNode.left != null) {
                        currentNode.left.val = levelSums.get(levelIndex) - siblingSum;
                        nodeQueue.offer(currentNode.left);
                    }
                    if (currentNode.right != null) {
                        currentNode.right.val = levelSums.get(levelIndex) - siblingSum;
                        nodeQueue.offer(currentNode.right);
                    }
                }
                ++levelIndex;
            }
            return root;
        }
    }

    // https://leetcode.com/problems/shortest-cycle-in-a-graph/description/
    static class Solution4 {
        public int findShortestCycle(int n, int[][] edges) {
            var adj = new HashMap<Integer, List<Integer>>();
            for (var e : edges) {
                adj.computeIfAbsent(e[0], $ -> new ArrayList<>()).add(e[1]);
                adj.computeIfAbsent(e[1], $ -> new ArrayList<>()).add(e[0]);
            }
            var minCycle = n + 1;

            for (int i = 0; i < n; i++) {
                var dist = new int[n];
                Arrays.fill(dist, -1);
                dist[i] = 0;

                var q = new LinkedList<Integer>();
                q.add(i);
                while (q.size() > 0) {
                    var u = q.remove();
                    for (var v : adj.getOrDefault(u, new ArrayList<>())) {
                        if (dist[v] == -1) {
                            dist[v] = dist[u] + 1;
                            q.add(v);
                        } else if (dist[v] >= dist[u]) {
                            minCycle = Math.min(minCycle, dist[v] + dist[u] + 1);
                        }
                    }
                }
            }

            if (minCycle <= n) {
                return minCycle;
            }
            return -1;
        }
    }
}
