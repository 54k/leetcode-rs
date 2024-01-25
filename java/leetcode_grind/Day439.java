package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;

public class Day439 {
    // https://leetcode.com/problems/longest-common-subsequence/description/
    static class Solution1 {
        public int longestCommonSubsequence(String text1, String text2) {
            int m = text1.length();
            int n = text2.length();

            int[][] dp = new int[m + 1][n + 1];

            for (int i = 1; i <= m; ++i) {
                for (int j = 1; j <= n; ++j) {
                    if (text1.charAt(i - 1) == text2.charAt(j - 1)) {
                        dp[i][j] = dp[i - 1][j - 1] + 1;
                    } else {
                        dp[i][j] = Math.max(dp[i - 1][j], dp[i][j - 1]);
                    }
                }
            }

            return dp[m][n];
        }
    }

    // https://leetcode.com/problems/find-original-array-from-doubled-array/description/
    static class Solution2 {
        public int[] findOriginalArray(int[] changed) {
            if (changed.length == 0 || changed.length % 2 == 1)
                return new int[] {};

            var arr = new Integer[changed.length];
            for (int i = 0; i < arr.length; i++) {
                arr[i] = changed[i];
            }

            Arrays.sort(arr, Comparator.comparingInt(Math::abs));
            Map<Integer, Integer> cnt = new HashMap<>();
            for (var e : arr) {
                cnt.put(e, cnt.getOrDefault(e, 0) + 1);
            }
            var ans = new int[changed.length / 2];
            int i = 0;
            for (var e : arr) {
                if (cnt.get(e) == 0) {
                    continue;
                }
                if (cnt.getOrDefault(e * 2, 0) <= 0) {
                    return new int[] {};
                }

                cnt.put(e, cnt.get(e) - 1);
                cnt.put(e * 2, cnt.get(e * 2) - 1);
                ans[i++] = e;
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/subdomain-visit-count/description/
    static class Solution3 {
        public List<String> subdomainVisits(String[] cpdomains) {
            var counter = new HashMap<String, Integer>();

            for (var log : cpdomains) {
                var split = log.split("\\s");

                var cnt = Integer.valueOf(split[0]);
                var d = split[1];

                counter.put(d, counter.getOrDefault(d, 0) + cnt);
                while (d.indexOf('.') != -1) {
                    d = d.substring(d.indexOf('.') + 1);
                    counter.put(d, counter.getOrDefault(d, 0) + cnt);
                }
            }

            var res = new ArrayList<String>();
            for (var e : counter.entrySet()) {
                res.add(String.format("%s %s", e.getValue(), e.getKey()));
            }

            return res;
        }
    }

    // https://leetcode.com/problems/minimum-cost-to-connect-sticks/
    static class Solution4 {
        public int connectSticks(int[] sticks) {
            PriorityQueue<Integer> pq = new PriorityQueue<>();
            for (var s : sticks) {
                pq.add(s);
            }

            int ans = 0;
            while (pq.size() > 1) {
                int a = pq.remove();
                int b = pq.remove();

                ans += a + b;
                pq.add(a + b);
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/find-original-array-from-doubled-array/
    static class Solution5 {
        public int[] findOriginalArray(int[] changed) {
            if (changed.length == 0 || changed.length % 2 == 1)
                return new int[] {};

            var arr = new Integer[changed.length];
            for (int i = 0; i < arr.length; i++) {
                arr[i] = changed[i];
            }

            Arrays.sort(arr, Comparator.comparingInt(Math::abs));
            Map<Integer, Integer> cnt = new HashMap<>();
            for (var e : arr) {
                cnt.put(e, cnt.getOrDefault(e, 0) + 1);
            }
            var ans = new int[changed.length / 2];
            int i = 0;
            for (var e : arr) {
                if (cnt.get(e) == 0) {
                    continue;
                }
                if (cnt.getOrDefault(e * 2, 0) <= 0) {
                    return new int[] {};
                }

                cnt.put(e, cnt.get(e) - 1);
                cnt.put(e * 2, cnt.get(e * 2) - 1);
                ans[i++] = e;
            }
            return ans;
        }
    }
}
