package leetcode_grind;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Random;
import java.util.TreeMap;

public class Day370 {
    // https://leetcode.com/problems/range-sum-query-mutable/
    class NumArray {
        static class Fenwick {
            int n;
            int[] t;

            Fenwick(int[] nums) {
                n = nums.length;
                t = new int[n];
                for (int i = 0; i < n; i++) {
                    update(i, nums[i]);
                }
            }

            void update(int i, int delta) {
                for (int j = i; j < n; j |= j + 1) {
                    t[j] += delta;
                }
            }

            private int sum(int r) {
                int s = 0;
                for (; r >= 0; r = (r & (r + 1)) - 1) {
                    s += t[r];
                }
                return s;
            }

            public int sum(int l, int r) {
                return sum(r) - sum(l - 1);
            }
        }

        int[] arr;
        Fenwick fenwick;

        public NumArray(int[] nums) {
            arr = nums;
            fenwick = new Fenwick(nums);
        }

        public void update(int index, int val) {
            int delta = val - arr[index];
            arr[index] += delta;
            fenwick.update(index, delta);
        }

        public int sumRange(int left, int right) {
            return fenwick.sum(left, right);
        }
    }

    // https://leetcode.com/problems/guess-the-word/
    // static class Solution {
    // public void findSecretWord(String[] words, Master master) {
    // var excluded = new HashSet<String>();
    // var rand = new Random();
    // var tries = 0;
    // while (true) {
    // var guessed = words[Math.abs(rand.nextInt() % words.length)];
    // if (!excluded.add(guessed)) {
    // continue;
    // }
    // if (tries++ > 30)
    // return;

    // var sim = master.guess(guessed);
    // if (sim == guessed.length()) {
    // return; // guessed
    // }

    // for (var w : words) {
    // if (excluded.contains(w))
    // continue;

    // var numsim = 0;
    // for (int i = 0; i < Math.min(w.length(), guessed.length()); i++) {
    // if (w.charAt(i) == guessed.charAt(i)) {
    // numsim++;
    // }
    // }

    // if (numsim != sim) {
    // excluded.add(w);
    // }
    // }
    // }
    // }
    // }

    // https://leetcode.com/problems/shortest-path-to-get-food/
    static class Solution1 {
        public int getFood(char[][] grid) {
            var m = grid.length;
            var n = grid[0].length;
            var start = new int[] { 0, 0 };
            outer: for (int i = 0; i < m; i++)
                for (int j = 0; j < n; j++)
                    if (grid[i][j] == '*') {
                        start[0] = i;
                        start[1] = j;
                        break outer;
                    }

            var level = new ArrayList<int[]>();
            level.add(start);
            var time = -1;

            while (level.size() > 0) {
                time++;
                var nextLevel = new ArrayList<int[]>();

                for (var pos : level) {
                    for (var dir : new int[][] { { 1, 0 }, { -1, 0 }, { 0, 1 }, { 0, -1 } }) {
                        var next = new int[] { pos[0] + dir[0], pos[1] + dir[1] };
                        if (0 <= next[0] && m > next[0] && 0 <= next[1] && n > next[1]) {
                            var x = next[0];
                            var y = next[1];
                            if (grid[x][y] == 'O') {
                                nextLevel.add(next);
                                grid[x][y] = 'X';
                            } else if (grid[x][y] == '#') {
                                return time + 1;
                            }
                        }
                    }
                }

                level = nextLevel;
            }
            return -1;
        }
    }

    // https://leetcode.com/problems/maximum-depth-of-binary-tree/
    static class Solution2 {
        public int maxDepth(TreeNode root) {
            if (root == null) {
                return 0;
            }
            var left = maxDepth(root.left) + 1;
            var right = maxDepth(root.right) + 1;
            return Math.max(left, right);
        }
    }

    // https://leetcode.com/problems/amount-of-time-for-binary-tree-to-be-infected/
    static class Solution3 {
        public int amountOfTime(TreeNode root, int start) {
            var tree = new HashMap<Integer, List<Integer>>();
            var builder = new Object() {
                void build(TreeNode root) {
                    if (root == null) {
                        return;
                    }

                    tree.putIfAbsent(root.val, new ArrayList<>());
                    if (root.left != null) {
                        tree.putIfAbsent(root.left.val, new ArrayList<>());
                        tree.get(root.val).add(root.left.val);
                        tree.get(root.left.val).add(root.val);
                    }
                    if (root.right != null) {
                        tree.putIfAbsent(root.right.val, new ArrayList<>());
                        tree.get(root.val).add(root.right.val);
                        tree.get(root.right.val).add(root.val);
                    }
                    build(root.left);
                    build(root.right);
                }
            };

            builder.build(root);

            var seen = new HashSet<Integer>();
            seen.add(start);

            var queue = new ArrayDeque<Integer>();
            queue.addLast(start);

            var time = -1;
            while (!queue.isEmpty()) {
                time++;
                var n = queue.size();
                for (int i = 0; i < n; ++i) {
                    var node = queue.pollFirst();
                    for (var next : tree.get(node)) {
                        if (seen.add(next)) {
                            queue.addLast(next);
                        }
                    }
                }
            }

            return time;
        }
    }

    // https://leetcode.com/problems/continuous-subarrays/description/
    static class Solution4 {
        public long continuousSubarrays(int[] nums) {
            var map = new TreeMap<Integer, Integer>();
            int i = 0;
            int j = 0;
            long ans = 0;

            while (i < nums.length) {
                map.put(nums[i], map.getOrDefault(nums[i], 0) + 1);
                i++;

                while (Math.abs(map.firstEntry().getKey() - map.lastEntry().getKey()) > 2) {
                    map.put(nums[j], map.getOrDefault(nums[j], 0) - 1);
                    if (map.get(nums[j]) == 0) {
                        map.remove(nums[j]);
                    }
                    j++;
                }

                ans += i - j;
            }

            return ans;
        }
    }
}
