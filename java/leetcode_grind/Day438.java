package leetcode_grind;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.HashMap;
import java.util.Map;

class Day438 {
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

    // https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/description/?envType=daily-question&envId=2024-01-23
    static class Solution1 {
        public int pseudoPalindromicPaths1(TreeNode root) {
            Map<Integer, Integer> paths = new HashMap<>();
            var dfs = new Object() {
                int ans = 0;

                void apply(TreeNode root) {
                    if (root == null)
                        return;

                    try {
                        paths.put(root.val, paths.getOrDefault(root.val, 0) + 1);
                        if (root.left == null && root.right == null) {
                            boolean odd = false;
                            for (var e : paths.entrySet()) {
                                if (e.getValue() % 2 == 1) {
                                    if (odd) {
                                        return;
                                    }
                                    odd = true;
                                }
                            }
                            ans++;
                            return;
                        }

                        apply(root.left);
                        apply(root.right);
                    } finally {
                        paths.put(root.val, paths.getOrDefault(root.val, 0) - 1);
                        if (paths.get(root.val) == 0) {
                            paths.remove(root.val);
                        }
                    }
                }
            };

            dfs.apply(root);
            return dfs.ans;
        }

        public int pseudoPalindromicPaths2(TreeNode root) {
            class Pair<F, S> {
                F key;
                S value;

                Pair(F f, S s) {
                    key = f;
                    value = s;
                }
            }

            int count = 0, path = 0;
            Deque<Pair<TreeNode, Integer>> stack = new ArrayDeque<>();
            stack.push(new Pair<>(root, 0));

            while (!stack.isEmpty()) {
                Pair<TreeNode, Integer> p = stack.pop();
                TreeNode node = p.key;
                path = p.value;

                if (node != null) {
                    path = path ^ (1 << node.val);
                    if (node.left == null && node.right == null) {
                        if ((path & (path - 1)) == 0) {
                            ++count;
                        }
                    } else {
                        stack.push(new Pair<>(node.right, path));
                        stack.push(new Pair<>(node.left, path));
                    }
                }
            }

            return count;
        }

        public int pseudoPalindromicPaths3(TreeNode root) {
            var dfs = new Object() {
                int count;

                void apply(TreeNode node, int path) {
                    if (node == null) {
                        return;
                    }

                    path = (1 << node.val) ^ path;
                    if (node.left == null && node.right == null) {
                        if ((path & (path - 1)) == 0) {
                            ++count;
                        }
                    }

                    apply(node.left, path);
                    apply(node.right, path);
                }
            };

            dfs.apply(root, 0);
            return dfs.count;
        }
    }
}