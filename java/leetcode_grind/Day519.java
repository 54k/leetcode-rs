package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;

public class Day519 {
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

    // https://leetcode.com/problems/minimum-distance-between-bst-nodes/description/
    static class Solution1 {
        public int minDiffInBST(TreeNode root) {
            var ans = Integer.MAX_VALUE;
            TreeNode prev = null;
            var curr = root;

            while (curr != null) {
                if (curr.left == null) {
                    if (prev != null) {
                        ans = Math.min(ans, curr.val - prev.val);
                    }
                    prev = curr;
                    curr = curr.right;
                } else {
                    var pre = curr.left;
                    while (pre.right != null) {
                        pre = pre.right;
                    }
                    pre.right = curr;
                    var t = curr;
                    curr = curr.left;
                    t.left = null;
                }
            }

            return ans;
        }
    }

    static class LeafResult {
        TreeNode node;
        int dist;

        LeafResult(TreeNode n, int d) {
            node = n;
            dist = d;
        }
    }

    // https://leetcode.com/problems/closest-leaf-in-a-binary-tree/description/
    static class Solution2 {
        List<TreeNode> path;
        Map<TreeNode, LeafResult> annotation;

        public int findClosestLeaf(TreeNode root, int k) {
            path = new ArrayList<>();
            annotation = new HashMap<>();

            dfs(root, k);

            int distanceFromTarget = path.size() - 1;
            int dist = Integer.MAX_VALUE;
            TreeNode leaf = null;

            for (TreeNode node : path) {
                LeafResult lr = closestLeaf(node);
                if (lr.dist + distanceFromTarget < dist) {
                    dist = lr.dist + distanceFromTarget;
                    leaf = lr.node;
                }
                distanceFromTarget--;
            }
            return leaf.val;
        }

        boolean dfs(TreeNode node, int k) {
            if (node == null) {
                return false;
            } else if (node.val == k) {
                path.add(node);
                return true;
            } else {
                path.add(node);
                boolean ans = dfs(node.left, k);
                if (ans) {
                    return true;
                }
                ans = dfs(node.right, k);
                if (ans) {
                    return true;
                }
                path.remove(path.size() - 1);
                return false;
            }
        }

        LeafResult closestLeaf(TreeNode root) {
            if (root == null) {
                return new LeafResult(null, Integer.MAX_VALUE);
            } else if (root.left == null && root.right == null) {
                return new LeafResult(root, 0);
            } else if (annotation.containsKey(root)) {
                return annotation.get(root);
            } else {
                LeafResult r1 = closestLeaf(root.left);
                LeafResult r2 = closestLeaf(root.right);
                LeafResult ans = new LeafResult(r1.dist < r2.dist ? r1.node : r2.node, Math.min(r1.dist, r2.dist) + 1);
                annotation.put(root, ans);
                return ans;
            }
        }
    }

    // https://leetcode.com/problems/find-the-kth-smallest-sum-of-a-matrix-with-sorted-rows/description/
    static class Solution3 {
        public int[] kthSmallestPair(int[] nums1, int[] nums2, int k) {
            var res = new ArrayList<Integer>();
            var heap = new PriorityQueue<int[]>((a, b) -> a[0] - b[0]);

            int i = 0;
            while (i < nums1.length && i < k) {
                heap.offer(new int[] { nums1[i] + nums2[0], nums1[i], nums2[0], 0 });
                i += 1;
            }

            while (k > 0 && heap.size() > 0) {
                var curr = heap.remove();
                res.add(curr[0]);
                if (curr[3] == nums2.length - 1) {
                    continue;
                }

                heap.offer(new int[] {
                        curr[1] + nums2[curr[3] + 1], curr[1], nums2[curr[3] + 1], curr[3] + 1
                });
                k -= 1;
            }

            int[] ans = new int[res.size()];
            i = 0;
            for (; i < res.size(); i++) {
                ans[i] = res.get(i);
            }
            return ans;
        }

        public int kthSmallest(int[][] mat, int k) {
            int m = mat.length;
            int[] res = mat[0];
            for (int i = 1; i < m; i++) {
                res = kthSmallestPair(res, mat[i], k);
            }
            return res[k - 1];
        }
    }
}
