package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;

public class day649 {

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

    // https://leetcode.com/problems/path-with-maximum-probability/description/?envType=daily-question&envId=2024-08-27
    static class Solution1 {
        public double maxProbability(int n, int[][] edges, double[] succProb, int start_node, int end_node) {
            double[] maxProb = new double[n];
            maxProb[start_node] = 1.0;

            for (int i = 0; i < n - 1; i++) {
                boolean hasUpdate = false;
                for (int j = 0; j < edges.length; j++) {
                    int u = edges[j][0];
                    int v = edges[j][1];

                    double pathProb = succProb[j];
                    if (maxProb[u] * pathProb > maxProb[v]) {
                        maxProb[v] = maxProb[u] * pathProb;
                        hasUpdate = true;
                    }
                    if (maxProb[v] * pathProb > maxProb[u]) {
                        maxProb[u] = maxProb[v] * pathProb;
                        hasUpdate = true;
                    }
                }
                if (!hasUpdate)
                    break;
            }

            return maxProb[end_node];
        }
    }
    // static class Solution12 {
    // public double maxProbability(int n, int[][] edges, double[] succProb, int
    // start_node, int end_node) {
    // Map<Integer, List<Pair<Integer, Double>>> graph = new HashMap<>();
    // for (int i = 0; i < edges.length; i++) {
    // int u = edges[i][0], v = edges[i][1];
    // double pathProb = succProb[i];
    // graph.computeIfAbsent(u, k -> new ArrayList<>()).add(new Pair<>(v,
    // pathProb));
    // graph.computeIfAbsent(v, k -> new ArrayList<>()).add(new Pair<>(u,
    // pathProb));
    // }
    // double[] maxProb = new double[n];
    // maxProb[start_node] = 1d;
    // Queue<Integer> queue = new LinkedList<>();
    // queue.offer(start_node);
    // while (!queue.isEmpty()) {
    // int curNode = queue.poll();
    // for (var neighbor : graph.getOrDefault(curNode, new ArrayList<>())) {
    // int nxtNode = neighbor.getKey();
    // double pathProb = neighbor.getValue();

    // if (maxProb[curNode] * pathProb > maxProb[nxtNode]) {
    // maxProb[nxtNode] = maxProb[curNode] * pathProb;
    // queue.offer(nxtNode);
    // }
    // }
    // }
    // return maxProb[end_node];
    // }
    // }

    // https://leetcode.com/problems/cousins-in-binary-tree/description/
    static class Solution2 {
        int recordedDepth = -1;
        boolean isCousin = false;

        boolean dfs(TreeNode node, int depth, int x, int y) {
            if (node == null) {
                return false;
            }
            if (this.recordedDepth != -1 && depth > this.recordedDepth) {
                return false;
            }
            if (node.val == x || node.val == y) {
                if (this.recordedDepth == -1) {
                    this.recordedDepth = depth;
                }
                return this.recordedDepth == depth;
            }

            boolean left = dfs(node.left, depth + 1, x, y);
            boolean right = dfs(node.right, depth + 1, x, y);

            if (left && right && this.recordedDepth != depth + 1) {
                this.isCousin = true;
            }
            return left || right;
        }

        public boolean isCousins(TreeNode root, int x, int y) {
            dfs(root, 0, x, y);
            return this.isCousin;
        }
    }

    static class Solution3 {
        public boolean isCousins(TreeNode root, int x, int y) {
            Queue<TreeNode> queue = new LinkedList<>();
            queue.add(root);

            while (!queue.isEmpty()) {
                boolean siblings = false;
                boolean cousins = false;
                int nodesAtDepth = queue.size();

                for (int i = 0; i < nodesAtDepth; i++) {
                    TreeNode node = queue.remove();

                    if (node == null) {
                        siblings = false;
                    } else {
                        if (node.val == x || node.val == y) {
                            if (!cousins) {
                                siblings = cousins = true;
                            } else {
                                return !siblings;
                            }
                        }
                        if (node.left != null)
                            queue.add(node.left);
                        if (node.right != null)
                            queue.add(node.right);
                        queue.add(null);
                    }
                }
                if (cousins)
                    return false;
            }
            return false;
        }
    }

    // https://leetcode.com/problems/cousins-in-binary-tree-ii/description/
    static class Solution4 {
        public TreeNode replaceValueInTree(TreeNode root) {
            Queue<TreeNode> q = new LinkedList<>();
            q.add(root);
            while (!q.isEmpty()) {
                int n = q.size();
                int sum = 0;
                TreeNode[] nodes = new TreeNode[n];
                for (int i = 0; i < n; i++) {
                    TreeNode node = q.remove();
                    if (node != null) {
                        nodes[i] = node;
                        sum += node.val;
                    }
                }
                if (n == 1) {
                    nodes[0].val = 0;
                    q.add(nodes[0].left);
                    q.add(nodes[0].right);
                }
                for (int i = 0; i < n - 1; i += 2) {
                    var node1 = nodes[i];
                    var node2 = nodes[i + 1];
                    var n1 = node1 == null ? 0 : node1.val;
                    var n2 = node2 == null ? 0 : node2.val;
                    int nodeVal = sum - n1 - n2;
                    if (node1 != null) {
                        node1.val = nodeVal;
                        q.add(node1.left);
                        q.add(node1.right);
                    }
                    if (node2 != null) {
                        node2.val = nodeVal;
                        q.add(node2.left);
                        q.add(node2.right);
                    }
                }
            }
            return root;
        }
    }

}
