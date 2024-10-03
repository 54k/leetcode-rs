package leetcode_grind;

import java.math.BigInteger;
import java.util.*;

public class Day686 {
    // https://leetcode.com/problems/make-sum-divisible-by-p/description/?envType=daily-question&envId=2024-10-03
    static class Solution1 {
        public int minSubarray(int[] nums, int p) {
            int n = nums.length;
            long totalSum = 0;
            for (int num : nums) {
                totalSum += num;
            }

            if (totalSum % p == 0)
                return 0;
            int minLen = n;

            for (int start = 0; start < n; start++) {
                long subSum = 0;
                for (int end = start; end < n; end++) {
                    subSum += nums[end];
                    long remainingSum = (totalSum - subSum) % p;
                    if (remainingSum == 0) {
                        minLen = Math.min(minLen, end - start + 1);
                    }
                }
            }
            return minLen == n ? -1 : minLen;
        }
    }

    static class Solution2 {
        public int minSubarray(int[] nums, int p) {
            int n = nums.length;
            int totalSum = 0;
            for (int num : nums) {
                totalSum = (totalSum + num) % p;
            }
            int target = totalSum % p;
            if (target == 0) {
                return 0;
            }
            HashMap<Integer, Integer> modMap = new HashMap<>();
            modMap.put(0, -1);
            int currentSum = 0;
            int minLen = n;
            for (int i = 0; i < n; i++) {
                currentSum = (currentSum + nums[i]) % p;
                int needed = (currentSum - target + p) % p;
                if (modMap.containsKey(needed)) {
                    minLen = Math.min(minLen, i - modMap.get(needed));
                }
                modMap.put(currentSum, i);
            }
            return minLen == n ? -1 : minLen;
        }
    }

    // https://leetcode.com/problems/maximum-length-of-repeated-subarray/description/
    static class Solution3 {
        public int findLength(int[] A, int[] B) {
            int ans = 0;
            Map<Integer, ArrayList<Integer>> Bstarts = new HashMap<>();
            for (int j = 0; j < B.length; j++) {
                Bstarts.computeIfAbsent(B[j], x -> new ArrayList<>()).add(j);
            }
            for (int i = 0; i < A.length; i++) {
                if (Bstarts.containsKey(A[i])) {
                    for (int j : Bstarts.get(A[i])) {
                        int k = 0;
                        while (i + k < A.length && j + k < B.length && A[i + k] == B[j + k]) {
                            k++;
                        }
                        ans = Math.max(ans, k);
                    }
                }
            }
            return ans;
        }
    }

    static class Solution4 {
        boolean check(int length, int[] A, int[] B) {
            Set<String> seen = new HashSet<>();
            for (int i = 0; i + length <= A.length; i++) {
                seen.add(Arrays.toString(Arrays.copyOfRange(A, i, i + length)));
            }
            for (int j = 0; j + length <= B.length; ++j) {
                if (seen.contains(Arrays.toString(Arrays.copyOfRange(B, j, j + length)))) {
                    return true;
                }
            }
            return false;
        }

        public int findLength(int[] A, int[] B) {
            int lo = 0, hi = Math.min(A.length, B.length) + 1;
            while (lo < hi) {
                int mi = (lo + hi) / 2;
                if (check(mi, A, B)) {
                    lo = mi + 1;
                } else {
                    hi = mi;
                }
            }
            return lo - 1;
        }
    }

    static class Solution5 {
        public int findLength(int[] A, int[] B) {
            int ans = 0;
            int[][] memo = new int[A.length + 1][B.length + 1];
            for (int i = A.length - 1; i >= 0; --i) {
                for (int j = B.length - 1; j >= 0; --j) {
                    if (A[i] == B[j]) {
                        memo[i][j] = memo[i + 1][j + 1] + 1;
                        if (ans < memo[i][j]) {
                            ans = memo[i][j];
                        }
                    }
                }
            }
            return ans;
        }
    }

    static class Solution6 {
        int P = 113;
        int MOD = 1_000_000_007;
        int Pinv = BigInteger.valueOf(P).modInverse(BigInteger.valueOf(MOD)).intValue();

        int[] rolling(int[] source, int length) {
            int[] ans = new int[source.length - length + 1];
            long h = 0, power = 1;
            if (length == 0) {
                return ans;
            }
            for (int i = 0; i < source.length; ++i) {
                h = (h + source[i] * power) % MOD;
                if (i < length - 1) {
                    power = (power * P) % MOD;
                } else {
                    ans[i - (length - 1)] = (int) h;
                    h = (h - source[i - (length - 1)]) * Pinv % MOD;
                    if (h < 0)
                        h += MOD;
                }
            }
            return ans;
        }

        boolean check(int guess, int[] A, int[] B) {
            Map<Integer, List<Integer>> hashes = new HashMap<>();
            int k = 0;
            for (int x : rolling(A, guess)) {
                hashes.computeIfAbsent(x, z -> new ArrayList<>()).add(k++);
            }
            int j = 0;
            for (int x : rolling(B, guess)) {
                for (int i : hashes.getOrDefault(x, new ArrayList<Integer>())) {
                    if (Arrays.equals(Arrays.copyOfRange(A, i, i + guess), Arrays.copyOfRange(B, j, j + guess))) {
                        return true;
                    }
                }
                j++;
            }
            return false;
        }

        public int findLength(int[] A, int[] B) {
            int lo = 0, hi = Math.min(A.length, B.length) + 1;
            while (lo < hi) {
                int mi = (lo + hi) / 2;
                if (check(mi, A, B)) {
                    lo = mi + 1;
                } else {
                    hi = mi;
                }
            }
            return lo - 1;
        }
    }
}
