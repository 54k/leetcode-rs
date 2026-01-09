package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Random;
import java.util.Set;

public class Day1147 {
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

    // https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/description/?envType=daily-question&envId=2026-01-09
    static class Solution1 {
        Map<TreeNode, Integer> depth;
        int max_depth;

        public TreeNode subtreeWithAllDeepest(TreeNode root) {
            depth = new HashMap<>();
            depth.put(null, -1);
            dfs(root, null);
            max_depth = -1;
            for (Integer d : depth.values()) {
                max_depth = Math.max(max_depth, d);
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
            if (node == null || depth.get(node) == max_depth) {
                return node;
            }
            TreeNode L = answer(node.left);
            TreeNode R = answer(node.right);

            if (L != null && R != null)
                return node;
            if (L != null)
                return L;
            if (R != null)
                return R;
            return null;
        }
    }

    static class Solution2 {
        static class Result {
            TreeNode node;
            int dist;

            Result(TreeNode n, int d) {
                node = n;
                dist = d;
            }
        }

        Result dfs(TreeNode node) {
            if (node == null)
                return new Result(null, 0);
            Result L = dfs(node.left), R = dfs(node.right);
            if (L.dist > R.dist)
                return new Result(L.node, L.dist + 1);
            if (L.dist < R.dist)
                return new Result(R.node, R.dist + 1);
            return new Result(node, L.dist + 1);
        }

        public TreeNode subtreeWithAllDeepest(TreeNode root) {
            return dfs(root).node;
        }
    }

    // https://leetcode.com/problems/random-flip-matrix/description/
    static class Solution3 {
        Map<Integer, Integer> V = new HashMap<>();
        int nr, nc, rem;
        Random rand = new Random();

        public Solution3(int m, int n) {
            nr = m;
            nc = n;
            rem = nr * nc;
        }

        public int[] flip() {
            int r = rand.nextInt(rem--);
            int x = V.getOrDefault(r, r);
            V.put(r, V.getOrDefault(rem, rem));
            return new int[] { x / nc, x % nc };
        }

        public void reset() {
            V.clear();
            rem = nr * nc;
        }
    }

    static class Solution4 {
        int nr, nc, rem, b_size;
        List<Set<Integer>> buckets = new ArrayList<>();
        Random rand = new Random();

        public Solution4(int m, int n) {
            nr = m;
            nc = n;
            rem = nr * nc;
            b_size = (int) Math.sqrt(nr * nc);
            for (int i = 0; i < nr * nc; i += b_size) {
                buckets.add(new HashSet<>());
            }
        }

        public int[] flip() {
            int c = 0;
            int c0 = 0;
            int k = rand.nextInt(rem);
            for (Set<Integer> b1 : buckets) {
                if (c0 + b_size - b1.size() > k) {
                    while (true) {
                        if (!b1.contains(c)) {
                            if (c0 == k) {
                                b1.add(c);
                                rem--;
                                return new int[] { c / nc, c % nc };
                            }
                            c0++;
                        }
                        c++;
                    }
                }
                c += b_size;
                c0 += b_size - b1.size();
            }
            return null;
        }

        public void reset() {
            for (Set<Integer> b1 : buckets) {
                b1.clear();
            }
            rem = nr * nc;
        }
    }

}
