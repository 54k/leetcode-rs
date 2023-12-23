package leetcode_grind;

import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

public class Day406 {
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

    // https://leetcode.com/problems/path-crossing/description
    static class Solution1 {
        public boolean isPathCrossing(String path) {
            Map<Character, Pair<Integer, Integer>> moves = new HashMap<>();
            moves.put('N', new Pair(0, 1));
            moves.put('S', new Pair(0, -1));
            moves.put('W', new Pair(-1, 0));
            moves.put('E', new Pair(1, 0));

            Set<Pair<Integer, Integer>> visited = new HashSet<>();
            visited.add(new Pair(0, 0));

            int x = 0;
            int y = 0;

            for (Character c : path.toCharArray()) {
                var curr = moves.get(c);
                int dx = curr.getKey();
                int dy = curr.getValue();
                x += dx;
                y += dy;

                var pair = new Pair(x, y);
                if (visited.contains(pair)) {
                    return true;
                }
                visited.add(pair);
            }
            return false;
        }
    }

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

    // https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/description/
    static class Solution2 {
        Map<TreeNode, Integer> depth;
        int maxDepth;

        public TreeNode subtreeWithAllDeepest(TreeNode root) {
            depth = new HashMap<>();
            depth.put(null, -1);
            dfs(root, null);
            maxDepth = -1;
            for (Integer d : depth.values()) {
                maxDepth = Math.max(maxDepth, d);
            }
            return answer(root);
        }

        void dfs(TreeNode node, TreeNode parent) {
            if (node != null) {
                depth.put(node, depth.get(parent) + 1);
                dfs(node.left, node);
                dfs(node.right, node);
            }
        }

        TreeNode answer(TreeNode node) {
            if (node == null || depth.get(node) == maxDepth) {
                return node;
            }
            TreeNode L = answer(node.left),
                    R = answer(node.right);
            if (L != null && R != null)
                return node;
            if (L != null)
                return L;
            return R;
        }
    }

    static class Solution3 {
        static class Result {
            TreeNode node;
            int dist;

            Result(TreeNode n, int d) {
                node = n;
                dist = d;
            }
        }

        public TreeNode subtreeWithAllDeepest(TreeNode root) {
            return dfs(root).node;
        }

        Result dfs(TreeNode node) {
            if (node == null)
                return new Result(null, 0);
            Result L = dfs(node.left);
            Result R = dfs(node.right);
            if (L.dist > R.dist)
                return new Result(L.node, L.dist + 1);
            if (L.dist < R.dist)
                return new Result(R.node, R.dist + 1);
            return new Result(node, L.dist + 1);
        }
    }

    // https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/description/
    static class Solution4 {
        static class Result {
            TreeNode node;
            int dist;

            Result(TreeNode n, int d) {
                node = n;
                dist = d;
            }
        }

        public TreeNode lcaDeepestLeaves(TreeNode root) {
            return dfs(root).node;
        }

        Result dfs(TreeNode node) {
            if (node == null) {
                return new Result(null, 0);
            }
            Result left = dfs(node.left);
            Result right = dfs(node.right);

            if (left.dist < right.dist)
                return new Result(right.node, right.dist + 1);
            if (left.dist > right.dist)
                return new Result(left.node, left.dist + 1);
            return new Result(node, left.dist + 1);
        }
    }

    // https://leetcode.com/problems/find-distance-in-a-binary-tree/description/
    static class Solution5 {
        public int findDistance(TreeNode root, int p, int q) {
            var depth = new HashMap<Integer, Integer>();
            var dep = new Object() {
                void apply(TreeNode root, int d) {
                    if (root != null) {
                        apply(root.left, d + 1);
                        apply(root.right, d + 1);
                        depth.put(root.val, d + 1);
                    }
                }
            };
            dep.apply(root, -1);

            var lca = new Object() {
                TreeNode apply(TreeNode root, int p, int q) {
                    if (root == null || root.val == p || root.val == q) {
                        return root;
                    }
                    TreeNode left = apply(root.left, p, q);
                    TreeNode right = apply(root.right, p, q);
                    if (left != null && right != null) {
                        return root;
                    } else if (left != null) {
                        return left;
                    }
                    return right;
                }
            };

            var v = lca.apply(root, p, q);
            return depth.get(p) + depth.get(q) - 2 * depth.get(v.val);
        }
    }

    // https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree-ii/description/
    static class Solution7 {
        public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
            TreeNode ans = LCA(root, p, q);
            if (ans == p) {
                return dfs(p, q) ? p : null;
            } else if (ans == q) {
                return dfs(q, p) ? q : null;
            }
            return ans;
        }

        TreeNode LCA(TreeNode node, TreeNode p, TreeNode q) {
            if (node == null || node == p || node == q) {
                return node;
            }
            TreeNode left = LCA(node.left, p, q);
            TreeNode right = LCA(node.right, p, q);
            if (left != null && right != null) {
                return node;
            } else if (left != null) {
                return left;
            }
            return right;
        }

        boolean dfs(TreeNode node, TreeNode target) {
            if (node == target) {
                return true;
            }
            if (node == null) {
                return false;
            }
            return dfs(node.left, target) || dfs(node.right, target);
        }
    }
}
