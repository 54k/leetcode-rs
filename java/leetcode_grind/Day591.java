package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;
import java.util.Set;
import java.util.Stack;

public class Day591 {
    // https://leetcode.com/problems/all-ancestors-of-a-node-in-a-directed-acyclic-graph/description/?envType=daily-question&envId=2024-06-29
    static class Solution1 {
        public List<List<Integer>> getAncestors(int n, int[][] edges) {
            List<Integer>[] adjacencyList = new ArrayList[n];
            for (int i = 0; i < n; i++) {
                adjacencyList[i] = new ArrayList<>();
            }
            int[] indegree = new int[n];
            for (int[] edge : edges) {
                int from = edge[0];
                int to = edge[1];
                adjacencyList[from].add(to);
                indegree[to]++;
            }
            Queue<Integer> nodesWithZeroIndegree = new LinkedList<>();
            for (int i = 0; i < indegree.length; i++) {
                if (indegree[i] == 0) {
                    nodesWithZeroIndegree.add(i);
                }
            }
            List<Integer> topologicaOrder = new ArrayList<>();
            while (!nodesWithZeroIndegree.isEmpty()) {
                int currentNode = nodesWithZeroIndegree.poll();
                topologicaOrder.add(currentNode);

                for (int neighbor : adjacencyList[currentNode]) {
                    indegree[neighbor]--;
                    if (indegree[neighbor] == 0) {
                        nodesWithZeroIndegree.add(neighbor);
                    }
                }
            }
            List<List<Integer>> ancestorsList = new ArrayList<>();
            List<Set<Integer>> ancestorsSetList = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                ancestorsList.add(new ArrayList<>());
                ancestorsSetList.add(new HashSet<>());
            }

            for (int node : topologicaOrder) {
                for (int neighbor : adjacencyList[node]) {
                    ancestorsSetList.get(neighbor).add(node);
                    ancestorsSetList.get(neighbor).addAll(ancestorsSetList.get(node));
                }
            }

            for (int i = 0; i < ancestorsList.size(); i++) {
                ancestorsList.get(i).addAll(ancestorsSetList.get(i));
                Collections.sort(ancestorsList.get(i));
            }

            return ancestorsList;
        }
    }

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

    // https://leetcode.com/problems/split-bst/description/?envType=weekly-question&envId=2024-06-29
    static class Solution2 {
        public TreeNode[] splitBST(TreeNode root, int target) {
            if (root == null) {
                return new TreeNode[] { null, null };
            }

            if (root.val <= target) {
                var r = root.right;
                root.right = null;
                var s = splitBST(r, target);
                root.right = s[0];
                return new TreeNode[] { root, s[1] };
            } else {
                var l = root.left;
                root.left = null;
                var s = splitBST(l, target);
                root.left = s[1];
                return new TreeNode[] { s[0], root };
            }
        }
    }

    static class Solution3 {
        public TreeNode[] splitBST(TreeNode root, int target) {
            TreeNode[] ans = new TreeNode[2];
            if (root == null) {
                return ans;
            }
            Stack<TreeNode> stack = new Stack<>();
            while (root != null) {
                stack.push(root);
                if (root.val > target) {
                    root = root.left;
                } else {
                    root = root.right;
                }
            }
            while (!stack.isEmpty()) {
                TreeNode curr = stack.pop();
                if (curr.val > target) {
                    curr.left = ans[1];
                    ans[1] = curr;
                } else {
                    curr.right = ans[0];
                    ans[0] = curr;
                }
            }
            return ans;
        }
    }

    static class Solution4 {
        public TreeNode[] splitBST(TreeNode root, int target) {
            TreeNode dummySm = new TreeNode(0);
            TreeNode curSm = dummySm;
            TreeNode dummyLg = new TreeNode(0);
            TreeNode curLg = dummyLg;

            TreeNode current = root;
            TreeNode nextNode;

            while (current != null) {
                if (current.val <= target) {
                    curSm.right = current;
                    curSm = current;
                    nextNode = current.right;
                    curSm.right = null;
                    current = nextNode;
                } else {
                    curLg.left = current;
                    curLg = current;
                    nextNode = current.left;
                    curLg.left = null;
                    current = nextNode;
                }
            }

            return new TreeNode[] { dummySm.right, dummyLg.left };
        }
    }
}
