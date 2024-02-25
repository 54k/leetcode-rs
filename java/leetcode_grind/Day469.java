package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Stack;

public class Day469 {

    // https://leetcode.com/problems/greatest-common-divisor-traversal/description
    static class Solution1 {
        public boolean canTraverseAllPairs(int[] nums) {
            int MAX = 100000;
            int N = nums.length;
            boolean[] has = new boolean[MAX + 1];
            for (int x : nums) {
                has[x] = true;
            }

            if (N == 1)
                return true;
            if (has[1])
                return false;

            int[] sieve = new int[MAX + 1];
            for (int d = 2; d <= MAX; d++) {
                if (sieve[d] == 0) {
                    for (int v = d; v <= MAX; v += d) {
                        sieve[v] = d;
                    }
                }
            }

            var adj = new HashMap<Integer, List<Integer>>();
            for (int x : nums) {
                int val = x;
                while (val > 1) {
                    int prime = sieve[val];
                    int root = MAX + prime;

                    adj.computeIfAbsent(x, $ -> new ArrayList<>()).add(root);
                    adj.computeIfAbsent(root, $ -> new ArrayList<>()).add(x);

                    while (val % prime == 0) {
                        val /= prime;
                    }
                }
            }

            HashSet<Integer> vis = new HashSet<>();
            Stack<Integer> stack = new Stak<>();
            stack.push(nums[0]);
            vis.add(nums[0]);
            while (!stack.isEmpty()) {
                int top = stack.pop();
                for (int nei : adj.getOrDefault(top, new ArrayList<>())) {
                    if (vis.add(nei)) {
                        stack.push(nei);
                    }
                }
            }

            if (vis.size() == adj.size()) {
                return true;
            }

            return false;
        }
    }
}
