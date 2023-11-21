package leetcode_grind;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.Stack;
import java.util.function.Function;

public class Day374 {
    // https://leetcode.com/problems/count-nice-pairs-in-an-array/
    class Solution1 {
        public int countNicePairs(int[] nums) {
            var mod = 1_000_000_007;
            Function<Integer, Integer> rev = (n) -> {
                int r = 0;
                while (n > 0) {
                    r = r * 10 + n % 10;
                    n /= 10;
                }
                return r;
            };

            var ans = 0;
            var counter = new HashMap<Integer, Integer>();
            for (var num : nums) {
                var sum = num - rev.apply(num);
                ans = (ans + counter.getOrDefault(sum, 0)) % mod;
                counter.put(sum, counter.getOrDefault(sum, 0) + 1);
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/count-number-of-bad-pairs/
    static class Solution2 {
        public long countBadPairs(int[] nums) {
            var counter = new HashMap<Integer, Integer>();
            var n = (long) nums.length;
            long ans = n * (n - 1) / 2;
            for (int i = 0; i < nums.length; i++) {
                var sum = nums[i] - i;
                ans -= counter.getOrDefault(sum, 0);
                counter.put(sum, counter.getOrDefault(sum, 0) + 1);
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/k-diff-pairs-in-an-array/
    static class Solution3 {
        public int findPairs(int[] nums, int k) {
            int result = 0;
            Map<Integer, Integer> freq = new HashMap<>();
            for (int num : nums) {
                freq.put(num, freq.getOrDefault(num, 0) + 1);
            }

            for (var entry : freq.entrySet()) {
                int x = entry.getKey();
                int val = entry.getValue();

                if (k > 0 && freq.containsKey(x + k)) {
                    result++;
                } else if (k == 0 && val > 1) {
                    result++;
                }
            }

            return result;
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

    // https://leetcode.com/problems/minimum-absolute-difference-in-bst/
    static class Solution4 {
        public int getMinimumDifference(TreeNode root) {
            var dfs = new Object() {
                int prev = -1;
                int diff = Integer.MAX_VALUE;

                void inorder(TreeNode node) {
                    if (node == null) {
                        return;
                    }

                    inorder(node.left);

                    if (prev != -1) {
                        diff = Math.min(diff, Math.abs(prev - node.val));
                    }
                    prev = node.val;

                    inorder(node.right);
                }
            };
            if (root == null)
                return 0;
            dfs.inorder(root);
            return dfs.diff;
        }
    }

    // https://leetcode.com/problems/equal-tree-partition/
    static class Solution5 {
        public boolean checkEqualTree(TreeNode root) {
            var sums = new Object() {
                Stack<Integer> sums = new Stack<>();

                int apply(TreeNode node) {
                    if (node == null)
                        return 0;
                    sums.push(apply(node.left) + apply(node.right) + node.val);
                    return sums.peek();
                }
            };

            var total = sums.apply(root);
            sums.sums.pop();

            if (Math.abs(total) % 2 == 1) {
                return false;
            }

            for (var s : sums.sums) {
                if (s == total / 2) {
                    return true;
                }
            }

            return false;
        }
    }

    // https://leetcode.com/problems/tree-diameter/
    static class Solution6 {
        public int treeDiameter(int[][] edges) {
            var adj = new HashMap<Integer, Set<Integer>>();

            for (var e : edges) {
                adj.putIfAbsent(e[0], new HashSet<>());
                adj.get(e[0]).add(e[1]);
                adj.putIfAbsent(e[1], new HashSet<>());
                adj.get(e[1]).add(e[0]);
            }

            if (adj.isEmpty()) {
                return 0;
            }

            var dfs = new Object() {
                int diameter = 0;

                int apply(int v, int p) {
                    var dist1 = 0;
                    var dist2 = 0;

                    for (var u : adj.get(v)) {
                        if (u != p) {
                            var d = 1 + apply(u, v);
                            if (d > dist1) {
                                dist2 = dist1;
                                dist1 = d;
                            } else if (d > dist2) {
                                dist2 = d;
                            }
                        }
                    }

                    diameter = Math.max(diameter, dist1 + dist2);

                    return dist1;
                }
            };

            dfs.apply(0, -1);
            return dfs.diameter;
        }
    }

    // https://leetcode.com/problems/minimum-height-trees/
    static class Solution7 {
        public List<Integer> findMinHeightTrees(int n, int[][] edges) {
            var adj = new HashMap<Integer, Set<Integer>>();

            if (n <= 2) {
                var centroids = new ArrayList<Integer>();
                for (int i = 0; i < n; i++) {
                    centroids.add(i);
                }
                return centroids;
            }

            for (int i = 0; i < n; i++) {
                adj.put(i, new HashSet<>());
            }

            for (var e : edges) {
                adj.get(e[0]).add(e[1]);
                adj.get(e[1]).add(e[0]);
            }

            var leafs = new ArrayList<Integer>();

            for (int i = 0; i < n; i++) {
                if (adj.get(i).size() == 1) {
                    leafs.add(i);
                }
            }

            var remaining = n;
            while (remaining > 2) {
                remaining -= leafs.size();
                var next = new ArrayList<Integer>();

                for (var leaf : leafs) {
                    var neighbor = adj.get(leaf).iterator().next();
                    adj.get(neighbor).remove(leaf);

                    if (adj.get(neighbor).size() == 1) {
                        next.add(neighbor);
                    }

                }

                leafs = next;
            }

            return leafs;
        }
    }

    // https://leetcode.com/problems/course-schedule/
    static class Solution8 {
        public boolean canFinish1(int numCourses, int[][] prerequisites) {
            var adj = new HashMap<Integer, List<Integer>>();

            for (var i = 0; i < numCourses; i++) {
                adj.put(i, new ArrayList<>());
            }

            var inDegrees = new int[numCourses];
            for (var p : prerequisites) {
                var before = p[0];
                var after = p[1];
                adj.get(after).add(before);
                inDegrees[before]++;
            }

            var queue = new ArrayDeque<Integer>();
            for (int i = 0; i < numCourses; i++) {
                if (inDegrees[i] == 0) {
                    queue.addLast(i);
                }
            }

            var result = new ArrayList<Integer>();
            while (!queue.isEmpty()) {
                var v = queue.pollFirst();
                result.add(v);
                for (var u : adj.get(v)) {
                    if (--inDegrees[u] == 0) {
                        queue.addLast(u);
                    }
                }
            }

            return result.size() == numCourses;
        }

        public boolean canFinish2(int numCourses, int[][] prerequisites) {
            var adj = new HashMap<Integer, List<Integer>>();

            for (var i = 0; i < numCourses; i++) {
                adj.put(i, new ArrayList<>());
            }

            for (var p : prerequisites) {
                var before = p[0];
                var after = p[1];
                adj.get(before).add(after);
            }

            var result = new ArrayList<Integer>();
            var visited = new int[numCourses];
            var dfs = new Object() {
                boolean apply(int v) {
                    if (visited[v] == 1) {
                        return false;
                    }
                    visited[v] = 1;

                    for (var u : adj.get(v)) {
                        if (visited[u] != 2) {
                            if (!apply(u)) {
                                return false;
                            }
                        }
                    }

                    visited[v] = 2;
                    result.add(v);
                    return true;
                }
            };

            for (int course = 0; course < numCourses; course++) {
                if (visited[course] == 0) {
                    if (!dfs.apply(course)) {
                        return false;
                    }
                }
            }

            return result.size() == numCourses;
        }
    }

    // https://leetcode.com/problems/course-schedule-ii/
    static class Solution9 {
        public int[] findOrder(int numCourses, int[][] prerequisites) {
            var adj = new HashMap<Integer, List<Integer>>();

            for (var i = 0; i < numCourses; i++) {
                adj.put(i, new ArrayList<>());
            }

            for (var p : prerequisites) {
                var before = p[0];
                var after = p[1];
                adj.get(before).add(after);
            }

            var result = new ArrayList<Integer>();
            var visited = new int[numCourses];
            var dfs = new Object() {
                boolean apply(int v) {
                    if (visited[v] == 1) {
                        return false;
                    }
                    visited[v] = 1;

                    for (var u : adj.get(v)) {
                        if (visited[u] != 2) {
                            if (!apply(u)) {
                                return false;
                            }
                        }
                    }

                    visited[v] = 2;
                    result.add(v);
                    return true;
                }
            };

            for (int course = 0; course < numCourses; course++) {
                if (visited[course] == 0) {
                    if (!dfs.apply(course)) {
                        return new int[0];
                    }
                }
            }

            var r = new int[numCourses];
            for (int i = result.size() - 1; i >= 0; i--) {
                r[i] = result.get(i);
            }
            return r;
        }
    }

    // https://leetcode.com/problems/tree-diameter/
    static class Solution10 {
        public int treeDiameter1(int[][] edges) {
            var adj = new HashMap<Integer, List<Integer>>();

            for (var e : edges) {
                adj.putIfAbsent(e[0], new ArrayList<>());
                adj.get(e[0]).add(e[1]);
                adj.putIfAbsent(e[1], new ArrayList<>());
                adj.get(e[1]).add(e[0]);
            }

            if (adj.isEmpty()) {
                return 0;
            }

            var bfs = new Object() {
                int[] apply(int start) {
                    ArrayDeque<int[]> queue = new ArrayDeque<>();
                    queue.addLast(new int[] { 0, start });
                    Set<Integer> visited = new HashSet<>();
                    visited.add(start);

                    var max = new int[] { 0, start };

                    while (!queue.isEmpty()) {
                        var pop = queue.pollFirst();
                        var d = pop[0];
                        var v = pop[1];

                        if (max[0] < d) {
                            max = new int[] { d, v };
                        }

                        for (var u : adj.get(v)) {
                            if (visited.add(u)) {
                                queue.addLast(new int[] { d + 1, u });
                            }
                        }
                    }

                    return max;
                }
            };

            return bfs.apply(bfs.apply(0)[1])[0];
        }

        public int treeDiameter2(int[][] edges) {
            var adj = new HashMap<Integer, Set<Integer>>();

            for (var e : edges) {
                adj.putIfAbsent(e[0], new HashSet<>());
                adj.get(e[0]).add(e[1]);
                adj.putIfAbsent(e[1], new HashSet<>());
                adj.get(e[1]).add(e[0]);
            }

            if (adj.isEmpty()) {
                return 0;
            }

            var leafs = new ArrayList<Integer>();
            for (var e : adj.entrySet()) {
                if (e.getValue().size() == 1) {
                    leafs.add(e.getKey());
                }
            }

            var dist = 0;
            var countNodes = adj.size();
            while (countNodes > 2) {
                dist++;
                countNodes -= leafs.size();
                var next = new ArrayList<Integer>();
                for (var leaf : leafs) {
                    var neighbor = adj.get(leaf).iterator().next();
                    adj.get(neighbor).remove(leaf);
                    if (adj.get(neighbor).size() == 1) {
                        next.add(neighbor);
                    }
                }
                leafs = next;
            }

            if (countNodes == 2) {
                return dist * 2 + 1;
            }

            return dist * 2;
        }

        public int treeDiameter3(int[][] edges) {
            var adj = new HashMap<Integer, Set<Integer>>();

            for (var e : edges) {
                adj.putIfAbsent(e[0], new HashSet<>());
                adj.get(e[0]).add(e[1]);
                adj.putIfAbsent(e[1], new HashSet<>());
                adj.get(e[1]).add(e[0]);
            }

            if (adj.isEmpty()) {
                return 0;
            }

            var dfs = new Object() {
                int diameter = 0;

                int apply(int v, int p) {
                    var dist1 = 0;
                    var dist2 = 0;

                    for (var u : adj.get(v)) {
                        if (u != p) {
                            var d = 1 + apply(u, v);
                            if (d > dist1) {
                                dist2 = dist1;
                                dist1 = d;
                            } else if (d > dist2) {
                                dist2 = d;
                            }
                        }
                    }

                    diameter = Math.max(diameter, dist1 + dist2);

                    return dist1;
                }
            };

            dfs.apply(0, -1);
            return dfs.diameter;
        }
    }

    // https://leetcode.com/problems/create-components-with-same-value/
    static class Solution11 {
        public int componentValue(int[] nums, int[][] edges) {
            var n = nums.length;
            var sum = 0;
            var degree = new int[n];
            List<Integer>[] adj = new List[n];

            for (int i = 0; i < n; i++) {
                adj[i] = new ArrayList();
                sum += nums[i];
            }

            for (var e : edges) {
                degree[e[0]]++;
                degree[e[1]]++;
                adj[e[0]].add(e[1]);
                adj[e[1]].add(e[0]);
            }

            var bfs = new Object() {
                boolean apply(int[] sums, int target, int[] deg) {
                    var queue = new ArrayDeque<Integer>();

                    for (int i = 0; i < n; i++) {
                        if (deg[i] == 1) {
                            queue.addLast(i);
                        }
                    }

                    while (!queue.isEmpty()) {
                        var v = queue.pollFirst();
                        for (var u : adj[v]) {
                            if (sums[v] > target) {
                                return false;
                            }
                            sums[u] += sums[v] < target ? sums[v] : 0;

                            if (--deg[u] == 1) {
                                queue.addLast(u);
                            }
                        }
                    }

                    return true;
                }
            };

            var dfs = new Object() {
                int apply(int target, int v, int p) {
                    var s = nums[v];

                    for (var u : adj[v]) {
                        if (u == p) {
                            continue;
                        }
                        var s2 = apply(target, u, v);
                        if (s2 == -1) {
                            return -1;
                        }
                        s += s2;
                    }

                    if (s == target) {
                        return 0;
                    }
                    if (s > target) {
                        return -1;
                    }
                    return s;
                }
            };

            for (int i = n; i > 1; i--) {
                if (sum % i != 0) {
                    continue;
                }

                if (dfs.apply(sum / i, 0, 0) == 0) {
                    return i - 1;
                }
                // if (bfs.apply(nums.clone(), sum / i, degree.clone())) {
                // return i - 1;
                // }
            }

            return 0;
        }
    }

    // https://leetcode.com/problems/maximum-number-of-k-divisible-components/
    static class Solution12 {
        public int maxKDivisibleComponents(int n, int[][] edges, int[] values, int k) {
            List<Integer>[] adj = new List[n];
            int[] degree = new int[n];
            long[] sums = new long[n];

            for (var i = 0; i < n; i++) {
                adj[i] = new ArrayList<>();
            }

            for (var e : edges) {
                degree[e[0]]++;
                degree[e[1]]++;

                adj[e[0]].add(e[1]);
                adj[e[1]].add(e[0]);
            }

            var bfs = new Object() {
                int ans = 0;

                void apply() {
                    var queue = new ArrayDeque<Integer>();
                    for (int i = 0; i < n; i++) {
                        if (degree[i] == 1) {
                            queue.addLast(i);
                        }
                    }

                    if (queue.isEmpty()) {
                        ans++;
                        return;
                    }

                    while (!queue.isEmpty()) {
                        var v = queue.pollFirst();
                        sums[v] += (long) values[v];
                        if (sums[v] % k == 0) {
                            ans++;
                        }

                        for (var u : adj[v]) {
                            sums[u] += sums[v] % k == 0 ? 0 : sums[v];
                            if (--degree[u] == 1) {
                                queue.addLast(u);
                            }
                        }
                    }
                }
            };

            var dfs = new Object() {
                int ans = 0;

                long apply(int v, int p) {
                    long s = (long) values[v];

                    for (var u : adj[v]) {
                        if (u == p) {
                            continue;
                        }
                        long s2 = apply(u, v);
                        s += s2;
                    }

                    if (s % k == 0) {
                        ans++;
                        return 0;
                    }

                    return s;
                }
            };

            bfs.apply();
            // dfs.apply(0, -1);
            return bfs.ans;
        }
    }
}
