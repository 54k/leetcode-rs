package leetcode_grind;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Day329 {
    // https://leetcode.com/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons
    static class Solution {
        public int numOfArraysBottomUp(int n, int m, int k) {
            var dp = new int[n + 1][m + 1][k + 1];
            var MOD = 1000000007;

            for (var num = 0; num < dp[0].length; num++) {
                dp[n][num][0] = 1;
            }

            for (var i = n - 1; i >= 0; i--) {
                for (var maxSoFar = m; maxSoFar >= 0; maxSoFar--) {
                    for (var remain = 0; remain <= k; remain++) {
                        var ans = 0;
                        for (int num = 1; num <= maxSoFar; num++) {
                            ans = (ans + dp[i + 1][maxSoFar][remain]) % MOD;
                        }

                        if (remain > 0) {

                            for (int num = maxSoFar + 1; num <= m; num++) {
                                ans = (ans + dp[i + 1][num][remain - 1]) % MOD;
                            }
                        }

                        dp[i][maxSoFar][remain] = ans;
                    }
                }
            }

            return dp[0][0][k];
        }

        public int numOfArraysPerfixSum(int n, int m, int k) {
            var dp = new long[n + 1][m + 1][k + 1];
            var prefix = new long[n + 1][m + 1][k + 1];
            var MOD = 1000000007;

            for (var num = 1; num <= m; num++) {
                dp[1][num][1] = 1;
                prefix[1][num][1] = prefix[1][num - 1][1] + 1;
            }

            for (var i = 1; i <= n; i++) {
                for (var maxNum = 1; maxNum <= m; maxNum++) {
                    for (var cost = 1; cost <= k; cost++) {
                        long ans = (maxNum * dp[i - 1][maxNum][cost]) % MOD;
                        ans = (ans + prefix[i - 1][maxNum - 1][cost - 1]) % MOD;

                        dp[i][maxNum][cost] += ans;
                        dp[i][maxNum][cost] %= MOD;

                        prefix[i][maxNum][cost] = prefix[i][maxNum - 1][cost] + dp[i][maxNum][cost];

                        prefix[i][maxNum][cost] %= MOD;
                    }
                }
            }

            return (int) prefix[n][m][k];
        }
    }

    // https://leetcode.com/problems/encode-n-ary-tree-to-binary-tree/description/
    static class NaryTreeEncoder {
        static class Node {
            public int val;
            public List<Node> children;

            public Node() {
            }

            public Node(int _val) {
                val = _val;
            }

            public Node(int _val, List<Node> _children) {
                val = _val;
                children = _children;
            }
        }

        static class TreeNode {
            int val;
            TreeNode left;
            TreeNode right;

            TreeNode(int x) {
                val = x;
            }
        }

        static class Codec {
            static class Pair<F, S> {
                F first;
                S second;

                Pair(F f, S s) {
                    first = f;
                    second = s;
                }
            }

            // Encodes an n-ary tree to a binary tree.
            public TreeNode encode(Node root) {
                if (root == null) {
                    return null;
                }
                var h = new TreeNode(root.val);
                var queue = new LinkedList<Pair<TreeNode, Node>>();
                queue.add(new Pair<>(h, root));

                while (queue.size() > 0) {
                    var pop = queue.pop();

                    TreeNode head = null;
                    TreeNode next = null;

                    for (var child : pop.second.children) {
                        var ch = new TreeNode(child.val);
                        if (next == null) {
                            next = ch;
                            head = next;
                        } else {
                            next.right = ch;
                            next = ch;
                        }
                        queue.push(new Pair<>(ch, child));
                    }

                    pop.first.left = head;
                }

                return h;
            }

            // Decodes your binary tree to an n-ary tree.
            public Node decode(TreeNode root) {
                if (root == null) {
                    return null;
                }
                var h = new Node(root.val, new ArrayList<>());
                var queue = new LinkedList<Pair<Node, TreeNode>>();
                queue.push(new Pair<>(h, root));

                while (queue.size() > 0) {
                    var pop = queue.pop();
                    var child = pop.second.left;

                    while (child != null) {
                        var node = new Node(child.val, new ArrayList<>());
                        queue.add(new Pair<>(node, child));
                        pop.first.children.add(node);
                        child = child.right;
                    }
                }

                return h;
            }
        }
    }
}
