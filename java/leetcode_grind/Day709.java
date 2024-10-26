package leetcode_grind;

import java.util.*;

public class Day709 {
    // https://leetcode.com/problems/height-of-binary-tree-after-subtree-removal-queries/description/?envType=daily-question&envId=2024-10-26
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
        static final int[] maxHeightAfterRemoval = new int[100001];
        int currentMaxHeight = 0;

        public int[] treeQueries(TreeNode root, int[] queries) {
            traverseLeftToRight(root, 0);
            currentMaxHeight = 0;
            traverseRightToLeft(root, 0);

            int queryCount = queries.length;
            int[] queryResults = new int[queryCount];
            for (int i = 0; i < queryCount; i++) {
                queryResults[i] = maxHeightAfterRemoval[queries[i]];
            }
            return queryResults;
        }

        void traverseLeftToRight(TreeNode node, int currentHeight) {
            if (node == null)
                return;
            maxHeightAfterRemoval[node.val] = currentMaxHeight;
            currentMaxHeight = Math.max(currentMaxHeight, currentHeight);
            traverseLeftToRight(node.left, currentHeight + 1);
            traverseLeftToRight(node.right, currentHeight + 1);
        }

        void traverseRightToLeft(TreeNode node, int currentHeight) {
            if (node == null)
                return;
            maxHeightAfterRemoval[node.val] = Math.max(maxHeightAfterRemoval[node.val], currentMaxHeight);
            currentMaxHeight = Math.max(currentMaxHeight, currentHeight);
            traverseRightToLeft(node.right, currentHeight + 1);
            traverseRightToLeft(node.left, currentHeight + 1);
        }
    }

    static class Solution2 {
        public int[] treeQueries(TreeNode root, int[] queries) {
            Map<Integer, Integer> resultMap = new HashMap<>();
            Map<TreeNode, Integer> heightCache = new HashMap<>();

            dfs(root, 0, 0, resultMap, heightCache);
            int[] result = new int[queries.length];
            for (int i = 0; i < queries.length; i++) {
                result[i] = resultMap.get(queries[i]);
            }
            return result;
        }

        int height(TreeNode node, Map<TreeNode, Integer> heightCache) {
            if (node == null) {
                return -1;
            }

            if (heightCache.containsKey(node)) {
                return heightCache.get(node);
            }

            int h = 1 + Math.max(height(node.left, heightCache), height(node.right, heightCache));
            heightCache.put(node, h);
            return h;
        }

        void dfs(TreeNode node, int depth, int maxVal, Map<Integer, Integer> resultMap,
                Map<TreeNode, Integer> heightCache) {
            if (node == null) {
                return;
            }
            resultMap.put(node.val, maxVal);

            dfs(node.left, depth + 1, Math.max(maxVal, depth + 1 + height(node.right, heightCache)), resultMap,
                    heightCache);
            dfs(node.right, depth + 1, Math.max(maxVal, depth + 1 + height(node.left, heightCache)), resultMap,
                    heightCache);
        }
    }

    static class Solution3 {
        public int[] treeQueries(TreeNode root, int[] queries) {
            Map<Integer, Integer> nodeIndexMap = new HashMap<>();
            Map<Integer, Integer> subtreeSize = new HashMap<>();

            List<Integer> nodeDepths = new ArrayList<>();
            List<Integer> maxDepthFromLeft = new ArrayList<>();
            List<Integer> maxDepthFromRight = new ArrayList<>();

            dfs(root, 0, nodeIndexMap, nodeDepths);
            int totalNodes = nodeDepths.size();

            calculateSubstreeSize(root, subtreeSize);

            maxDepthFromLeft.add(nodeDepths.get(0));
            maxDepthFromRight.add(nodeDepths.get(totalNodes - 1));

            for (int i = 1; i < totalNodes; i++) {
                maxDepthFromLeft.add(Math.max(maxDepthFromLeft.get(i - 1), nodeDepths.get(i)));
                maxDepthFromRight.add(Math.max(maxDepthFromRight.get(i - 1), nodeDepths.get(totalNodes - i - 1)));
            }
            Collections.reverse(maxDepthFromRight);

            int[] results = new int[queries.length];
            for (int i = 0; i < queries.length; i++) {
                int queryNode = queries[i];
                int startIndex = nodeIndexMap.get(queryNode) - 1;
                int endIndex = startIndex + 1 + subtreeSize.get(queryNode);

                int maxDepth = maxDepthFromLeft.get(startIndex);
                if (endIndex < totalNodes) {
                    maxDepth = Math.max(maxDepth, maxDepthFromRight.get(endIndex));
                }
                results[i] = maxDepth;
            }
            return results;
        }

        void dfs(TreeNode root, int depth, Map<Integer, Integer> nodeIndexMap, List<Integer> nodeDepths) {
            if (root == null)
                return;
            nodeIndexMap.put(root.val, nodeDepths.size());
            nodeDepths.add(depth);
            dfs(root.left, depth + 1, nodeIndexMap, nodeDepths);
            dfs(root.right, depth + 1, nodeIndexMap, nodeDepths);
        }

        int calculateSubstreeSize(TreeNode root, Map<Integer, Integer> subtreeSize) {
            if (root == null)
                return 0;
            int leftSize = calculateSubstreeSize(root.left, subtreeSize);
            int rightSize = calculateSubstreeSize(root.right, subtreeSize);
            int totalSize = leftSize + rightSize + 1;
            subtreeSize.put(root.val, totalSize);
            return totalSize;
        }
    }

    static class Solution4 {
        public int[] treeQueries(TreeNode root, int[] queries) {
            List<Integer> eulerTour = new ArrayList<>();
            Map<Integer, Integer> nodeHeights = new HashMap<>();
            Map<Integer, Integer> firstOccurrence = new HashMap<>();
            Map<Integer, Integer> lastOccurrence = new HashMap<>();

            dfs(root, 0, eulerTour, nodeHeights, firstOccurrence, lastOccurrence);

            int tourSize = eulerTour.size();
            int[] maxDepthLeft = new int[tourSize];
            int[] maxDepthRight = new int[tourSize];

            maxDepthLeft[0] = maxDepthRight[tourSize - 1] = nodeHeights.get(root.val);

            for (int i = 1; i < tourSize; i++) {
                maxDepthLeft[i] = Math.max(maxDepthLeft[i - 1], nodeHeights.get(eulerTour.get(i)));
            }
            for (int i = tourSize - 2; i >= 0; i--) {
                maxDepthRight[i] = Math.max(maxDepthRight[i + 1], nodeHeights.get(eulerTour.get(i)));
            }

            int[] results = new int[queries.length];
            for (int i = 0; i < queries.length; i++) {
                int queryNode = queries[i];
                int leftMax = (firstOccurrence.get(queryNode) > 0) ? maxDepthLeft[firstOccurrence.get(queryNode) - 1]
                        : 0;
                int rightMax = (lastOccurrence.get(queryNode) < tourSize - 1)
                        ? maxDepthRight[lastOccurrence.get(queryNode) + 1]
                        : 0;
                results[i] = Math.max(leftMax, rightMax);
            }
            return results;
        }

        void dfs(TreeNode root, int height, List<Integer> eulerTour, Map<Integer, Integer> nodeHeights,
                Map<Integer, Integer> firstOccurence, Map<Integer, Integer> lastOccurence) {
            if (root == null) {
                return;
            }

            nodeHeights.put(root.val, height);
            firstOccurence.put(root.val, eulerTour.size());
            eulerTour.add(root.val);

            dfs(root.left, height + 1, eulerTour, nodeHeights, firstOccurence, lastOccurence);
            dfs(root.right, height + 1, eulerTour, nodeHeights, firstOccurence, lastOccurence);

            lastOccurence.put(root.val, eulerTour.size());
            eulerTour.add(root.val);
        }
    }

    static class Solution5 {
        public int[] treeQueries(TreeNode root, int[] queries) {
            Map<Integer, Integer> nodeDepths = new HashMap<>();
            Map<Integer, Integer> subtreeHeights = new HashMap<>();

            Map<Integer, Integer> firstLargestHeight = new HashMap<>();
            Map<Integer, Integer> secondLargestHeight = new HashMap<>();

            dfs(root, 0, nodeDepths, subtreeHeights, firstLargestHeight, secondLargestHeight);

            int[] results = new int[queries.length];
            for (int i = 0; i < queries.length; i++) {
                int queryNode = queries[i];
                int nodeLevel = nodeDepths.get(queryNode);

                if (subtreeHeights.get(queryNode).equals(firstLargestHeight.get(nodeLevel))) {
                    results[i] = nodeLevel + secondLargestHeight.getOrDefault(nodeLevel, 0) - 1;
                } else {
                    results[i] = nodeLevel + firstLargestHeight.getOrDefault(nodeLevel, 0) - 1;
                }
            }
            return results;
        }

        int dfs(
                TreeNode node,
                int level,
                Map<Integer, Integer> nodeDepths,
                Map<Integer, Integer> subtreeHeights,
                Map<Integer, Integer> firstLargestHeight,
                Map<Integer, Integer> secondLargestHeight) {
            if (node == null)
                return 0;
            nodeDepths.put(node.val, level);
            int leftHeight = dfs(node.left, level + 1, nodeDepths, subtreeHeights, firstLargestHeight,
                    secondLargestHeight);
            int rightHeight = dfs(node.right, level + 1, nodeDepths, subtreeHeights, firstLargestHeight,
                    secondLargestHeight);
            int currentHeigth = 1 + Math.max(leftHeight, rightHeight);
            subtreeHeights.put(node.val, currentHeigth);
            int currentFirstLargest = firstLargestHeight.getOrDefault(level, 0);
            if (currentHeigth > currentFirstLargest) {
                secondLargestHeight.put(level, currentFirstLargest);
                firstLargestHeight.put(level, currentHeigth);
            } else if (currentHeigth > secondLargestHeight.getOrDefault(level, 0)) {
                secondLargestHeight.put(level, currentHeigth);
            }
            return currentHeigth;
        }
    }
}
