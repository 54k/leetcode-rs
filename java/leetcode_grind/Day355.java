package leetcode_grind;

import java.util.PriorityQueue;

public class Day355 {
    static public class TreeNode {
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

    // https://leetcode.com/problems/count-nodes-equal-to-average-of-subtree/description
    static class Solution1 {
        public int averageOfSubtree(TreeNode root) {
            var ans = new int[] { 0 };
            var preorder = new Object() {
                int[] apply(TreeNode node) {
                    if (node == null) {
                        return new int[] { 0, 0 };
                    }

                    int[] left = apply(node.left);
                    int[] right = apply(node.right);

                    var sum = left[0] + right[0] + node.val;
                    var count = left[1] + right[1] + 1;
                    if (node.val == sum / count) {
                        ans[0]++;
                    }
                    return new int[] { sum, count };
                }
            };
            preorder.apply(root);
            return ans[0];
        }
    }

    // https://leetcode.com/problems/minimum-falling-path-sum/description/
    static class Solution2 {
        public int minFallingPathSum(int[][] matrix) {
            var n = matrix[0].length;
            for (int i = matrix.length - 2; i >= 0; i--) {
                for (int j = 0; j < n; j++) {
                    matrix[i][j] += Math.min(
                            matrix[i + 1][j],
                            Math.min(matrix[i + 1][Math.max(j - 1, 0)], matrix[i + 1][Math.min(j + 1, n - 1)]));
                }
            }

            var ans = Integer.MAX_VALUE;
            for (int i = 0; i < n; i++) {
                ans = Math.min(ans, matrix[0][i]);
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/trapping-rain-water-ii/description/
    static class Solution3 {
        public int trapRainWater(int[][] heightMap) {
            if (heightMap.length == 0) {
                return 0;
            }

            var m = heightMap.length;
            var n = heightMap[0].length;
            if (m < 3 || n < 3) {
                return 0;
            }

            var heap = new PriorityQueue<int[]>((a, b) -> {
                return a[0] - b[0];
            });

            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    if (i == 0 || i == m - 1 || j == 0 || j == n - 1) {
                        heap.add(new int[] { heightMap[i][j], i, j });
                        heightMap[i][j] = -1;
                    }
                }
            }

            int level = 0, res = 0;

            while (!heap.isEmpty()) {
                var poll = heap.poll();
                var x = poll[1];
                var y = poll[2];
                var height = poll[0];
                level = Math.max(height, level);

                for (var dir : new int[][] { { x - 1, y }, { x + 1, y }, { x, y - 1 }, { x, y + 1 } }) {
                    var i = dir[0];
                    var j = dir[1];
                    if (0 <= i && i < m && 0 <= j && j < n && heightMap[i][j] != -1) {
                        heap.add(new int[] { heightMap[i][j], i, j });

                        if (heightMap[i][j] < level) {
                            res += level - heightMap[i][j];
                        }
                        heightMap[i][j] = -1;
                    }
                }
            }

            return res;
        }
    }
}
