package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day1116 {
    // https://leetcode.com/problems/count-special-triplets/description/?envType=daily-question&envId=2025-12-09
    static class Solution1 {
        public int specialTriplets(int[] nums) {
            final int MOD = 1000000007;
            Map<Integer, Integer> numCnt = new HashMap<>();
            Map<Integer, Integer> numPartialCnt = new HashMap<>();
            for (int v : nums) {
                numCnt.put(v, numCnt.getOrDefault(v, 0) + 1);
            }

            long ans = 0;
            for (int v : nums) {
                int target = v * 2;
                int lCnt = numPartialCnt.getOrDefault(target, 0);
                numPartialCnt.put(v, numPartialCnt.getOrDefault(v, 0) + 1);
                int rCnt = numCnt.getOrDefault(target, 0) - numPartialCnt.getOrDefault(target, 0);
                ans = (ans + (long) lCnt * rCnt) % MOD;
            }
            return (int) ans;
        }
    }

}
