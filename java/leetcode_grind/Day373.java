package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day373 {
    // https://leetcode.com/problems/minimum-amount-of-time-to-collect-garbage/description
    static class Solution1 {
        public int garbageCollection1(String[] garbage, int[] travel) {
            int[] prefixSum = new int[travel.length + 1];
            prefixSum[1] = travel[0];
            for (int i = 1; i < travel.length; i++) {
                prefixSum[i + 1] = prefixSum[i] + travel[i];
            }

            Map<Character, Integer> garbageLastPos = new HashMap<>();
            Map<Character, Integer> garbageCount = new HashMap<>();

            for (int i = 0; i < garbage.length; i++) {
                for (char c : garbage[i].toCharArray()) {
                    garbageLastPos.put(c, i);
                    garbageCount.put(c, garbageCount.getOrDefault(c, 0) + 1);
                }
            }

            String garbageTypes = "MPG";
            int ans = 0;
            for (char c : garbageTypes.toCharArray()) {
                if (garbageCount.containsKey(c)) {
                    ans += prefixSum[garbageLastPos.get(c)] + garbageCount.get(c);
                }
            }
            return ans;
        }

        public int garbageCollection2(String[] garbage, int[] travel) {
            for (int i = 1; i < travel.length; i++) {
                travel[i] += travel[i - 1];
            }

            var ans = 0;
            var garbageLastPos = new HashMap<Character, Integer>();
            for (int i = 0; i < garbage.length; i++) {
                for (char c : garbage[i].toCharArray()) {
                    garbageLastPos.put(c, i);
                }
                ans += garbage[i].length();
            }

            for (char c : "MPG".toCharArray()) {
                ans += garbageLastPos.getOrDefault(c, 0) == 0 ? 0 : travel[garbageLastPos.get(c) - 1];
            }
            return ans;
        }

        public int garbageCollection3(String[] garbage, int[] travel) {
            var n = garbage.length;
            var ans = 0;

            for (var ch : new char[] { 'M', 'P', 'G' }) {
                var p = new int[n];
                var lastH = 0;
                for (int i = n - 1; i >= 0; i--) {
                    var h = garbage[i].replaceAll("[^" + ch + "]", "").length();
                    if (h > 0) {
                        lastH = Math.max(i, lastH);
                    }
                    p[i] = h + (i < n - 1 ? p[i + 1] : 0) + (i > 0 && lastH >= i ? travel[i - 1] : 0);
                }
                ans += p[0];
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/count-palindromic-subsequences/description/
    // https://leetcode.com/problems/count-palindromic-subsequences/solutions/2850466/c-java-python3-counting-prefixes-and-suffixes/
    class Solution2 {
        public int countPalindromes(String s) {
            throw new Error("not implemented");
        }
    }
}
