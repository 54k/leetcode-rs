package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.PriorityQueue;

public class Day423 {
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

    // https://leetcode.com/problems/leaf-similar-trees/description
    static class Solution1 {
        public boolean leafSimilar(TreeNode root1, TreeNode root2) {
            var rec = new Object() {
                int apply(TreeNode root, List<Integer> leaves) {
                    if (root == null) {
                        return -1;
                    }
                    var left = apply(root.left, leaves);
                    var right = apply(root.right, leaves);
                    var h = 1 + Math.max(left, right);
                    if (h == 0) {
                        leaves.add(root.val);
                    }
                    return h;
                }
            };

            var left = new ArrayList<Integer>();
            var right = new ArrayList<Integer>();

            rec.apply(root1, left);
            rec.apply(root2, right);

            return left.equals(right);
        }
    }

    // https://leetcode.com/problems/closest-binary-search-tree-value-ii/description/
    static class Solution2 {
        public List<Integer> closestKValues(TreeNode root, double target, int k) {
            var heap = new PriorityQueue<Integer>(
                    (a, b) -> Math.abs((double) a - target) > Math.abs((double) b - target) ? -1 : 1);

            var dfs = new Object() {
                void apply(TreeNode node) {
                    if (node == null) {
                        return;
                    }

                    heap.add(node.val);
                    if (heap.size() > k) {
                        heap.remove();
                    }

                    apply(node.left);
                    apply(node.right);
                }
            };

            dfs.apply(root);
            return new ArrayList<>(heap);
        }
    }

    // https://leetcode.com/problems/binary-tree-longest-consecutive-sequence/description/
    static class Solution3 {
        public int longestConsecutiveTopDown(TreeNode root) {
            var dfs = new Object() {
                int apply(TreeNode n, TreeNode p, int length) {
                    if (n == null)
                        return length;

                    length = (p != null && n.val == p.val + 1) ? length + 1 : 1;

                    return Math.max(
                            length,
                            Math.max(
                                    apply(n.left, n, length),
                                    apply(n.right, n, length)));
                }
            };

            return dfs.apply(root, null, 0);
        }

        int ans = 0;

        public int longestConsecutiveBottomUp(TreeNode root) {
            var dfs = new Object() {
                int apply(TreeNode r) {
                    if (r == null) {
                        return 0;
                    }

                    int left = apply(r.left) + 1;
                    int right = apply(r.right) + 1;

                    if (r.left != null && r.left.val != r.val + 1) {
                        left = 1;
                    }
                    if (r.right != null && r.right.val != r.val + 1) {
                        right = 1;
                    }
                    int length = Math.max(left, right);
                    ans = Math.max(length, ans);
                    return length;
                }
            };

            dfs.apply(root);
            return ans;
        }
    }

    // https://leetcode.com/problems/binary-tree-longest-consecutive-sequence-ii/description/
    static class Solution4 {
        int maxval = 0;

        public int longestConsecutive(TreeNode root) {
            longestPath(root);
            return maxval;
        }

        int[] longestPath(TreeNode root) {
            if (root == null) {
                return new int[] { 0, 0 };
            }

            int inr = 1, dcr = 1;
            if (root.left != null) {
                int[] left = longestPath(root.left);
                if (root.val == root.left.val + 1) {
                    dcr = left[1] + 1;
                } else if (root.val + 1 == root.left.val) {
                    inr = left[0] + 1;
                }
            }

            if (root.right != null) {
                int[] right = longestPath(root.right);
                if (root.val == root.right.val + 1) {
                    dcr = Math.max(dcr, right[1] + 1);
                } else if (root.val + 1 == root.right.val) {
                    inr = Math.max(inr, right[0] + 1);
                }
            }

            maxval = Math.max(maxval, dcr + inr - 1);
            return new int[] { inr, dcr };
        }
    }

    // https://leetcode.com/problems/count-nodes-equal-to-sum-of-descendants/description/
    static class Solution5 {
        public int equalToDescendants(TreeNode root) {
            int[] ans = new int[1];
            var dfs = new Object() {
                int apply(TreeNode node) {
                    if (node == null) {
                        return 0;
                    }

                    int sum = apply(node.left) + apply(node.right);
                    if (node.val == sum) {
                        ans[0]++;
                    }

                    return sum + node.val;
                }
            };
            dfs.apply(root);
            return ans[0];
        }
    }

    // https://leetcode.com/problems/most-frequent-subtree-sum/description/
    static class Solution6 {
        int maxFreq = 0;

        public int[] findFrequentTreeSum(TreeNode root) {
            var freq = new HashMap<Integer, Integer>();
            var dfs = new Object() {
                int apply(TreeNode node) {
                    if (node == null) {
                        return 0;
                    }

                    int sum = apply(node.left) + apply(node.right) + node.val;
                    freq.put(sum, freq.getOrDefault(sum, 0) + 1);
                    maxFreq = Math.max(maxFreq, freq.get(sum));
                    return sum;
                }
            };

            dfs.apply(root);
            var ans = new ArrayList<Integer>();
            for (var e : freq.entrySet()) {
                if (e.getValue() == maxFreq) {
                    ans.add(e.getKey());
                }
            }

            int[] arr = new int[ans.size()];
            for (int i = 0; i < ans.size(); i++) {
                arr[i] = ans.get(i);
            }

            return arr;
        }
    }
}
