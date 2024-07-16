package leetcode_grind;

import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.Map;
import java.util.Queue;
import java.util.Set;

public class Day607 {
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

    // https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another/description/?envType=daily-question&envId=2024-07-16
    static class Solution1 {
        public String getDirections(TreeNode root, int startValue, int destValue) {
            TreeNode lowestCommonAncestor = findLowestCommonAncestor(root, startValue, destValue);
            StringBuilder pathToStart = new StringBuilder();
            StringBuilder pathToDest = new StringBuilder();

            findPath(lowestCommonAncestor, startValue, pathToStart);
            findPath(lowestCommonAncestor, destValue, pathToDest);

            StringBuilder directions = new StringBuilder();

            directions.append("U".repeat(pathToStart.length()));
            directions.append(pathToDest);
            return directions.toString();
        }

        TreeNode findLowestCommonAncestor(TreeNode node, int value1, int value2) {
            if (node == null)
                return null;
            if (node.val == value1 || node.val == value2)
                return node;
            TreeNode leftLCA = findLowestCommonAncestor(node.left, value1, value2);
            TreeNode rightLCA = findLowestCommonAncestor(node.right, value1, value2);
            if (leftLCA == null)
                return rightLCA;
            else if (rightLCA == null)
                return leftLCA;
            else
                return node;
        }

        boolean findPath(TreeNode node, int targetValue, StringBuilder path) {
            if (node == null)
                return false;
            if (node.val == targetValue)
                return true;
            path.append("L");
            if (findPath(node.left, targetValue, path)) {
                return true;
            }
            path.setLength(path.length() - 1);
            path.append("R");
            if (findPath(node.right, targetValue, path)) {
                return true;
            }
            path.setLength(path.length() - 1);
            return false;
        }
    }

    static class Pair<F, S> {
        F key;
        S value;

        Pair(F k, S v) {
            key = k;
            value = v;
        }

        F getKey() {
            return key;
        }

        S getValue() {
            return value;
        }
    }

    static class Solution2 {
        public String getDirections(TreeNode root, int startValue, int destValue) {
            Map<Integer, TreeNode> parentMap = new HashMap<>();
            TreeNode startNode = findStartNode(root, startValue);
            populateParentMap(root, parentMap);

            Queue<TreeNode> queue = new LinkedList<>();
            queue.add(startNode);
            Set<TreeNode> visitedNodes = new HashSet<>();
            Map<TreeNode, Pair<TreeNode, String>> pathTracker = new HashMap<>();
            visitedNodes.add(startNode);

            while (!queue.isEmpty()) {
                TreeNode currentNode = queue.poll();
                if (currentNode.val == destValue) {
                    return backtrackPath(currentNode, pathTracker);
                }

                if (parentMap.containsKey(currentNode.val)) {
                    TreeNode parentNode = parentMap.get(currentNode.val);
                    if (!visitedNodes.contains(parentNode)) {
                        queue.add(parentNode);
                        pathTracker.put(parentNode, new Pair(currentNode, "U"));
                        visitedNodes.add(parentNode);
                    }
                }

                if (currentNode.left != null && !visitedNodes.contains(currentNode.left)) {
                    queue.add(currentNode.left);
                    pathTracker.put(currentNode.left, new Pair(currentNode, "L"));
                    visitedNodes.add(currentNode.left);
                }

                if (currentNode.right != null && !visitedNodes.contains(currentNode.right)) {
                    queue.add(currentNode.right);
                    pathTracker.put(currentNode.right, new Pair(currentNode, "R"));
                    visitedNodes.add(currentNode.right);
                }
            }

            return "";
        }

        String backtrackPath(TreeNode node, Map<TreeNode, Pair<TreeNode, String>> pathTracker) {
            StringBuilder path = new StringBuilder();
            while (pathTracker.containsKey(node)) {
                path.append(pathTracker.get(node).getValue());
                node = pathTracker.get(node).getKey();
            }
            path.reverse();
            return path.toString();
        }

        void populateParentMap(TreeNode node, Map<Integer, TreeNode> parentMap) {
            if (node == null)
                return;
            if (node.left != null) {
                parentMap.put(node.left.val, node);
                populateParentMap(node.left, parentMap);
            }
            if (node.right != null) {
                parentMap.put(node.right.val, node);
                populateParentMap(node.right, parentMap);
            }
        }

        TreeNode findStartNode(TreeNode node, int startValue) {
            if (node == null)
                return null;
            if (node.val == startValue)
                return node;
            TreeNode leftResult = findStartNode(node.left, startValue);
            if (leftResult != null)
                return leftResult;
            return findStartNode(node.right, startValue);
        }
    }
}
