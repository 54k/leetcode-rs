package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day787 {
    // https://leetcode.com/problems/find-the-prefix-common-array-of-two-arrays/description/
    static class Solution1 {
        public int[] findThePrefixCommonArray(int[] A, int[] B) {
            int n = A.length;
            int[] prefixCommonArray = new int[n];
            int[] frequency = new int[n + 1];
            int commonCount = 0;

            for (int currentIndex = 0; currentIndex < n; ++currentIndex) {
                frequency[A[currentIndex]] += 1;
                if (frequency[A[currentIndex]] == 2)
                    ++commonCount;

                frequency[B[currentIndex]] += 1;
                if (frequency[B[currentIndex]] == 2)
                    ++commonCount;

                prefixCommonArray[currentIndex] = commonCount;
            }

            return prefixCommonArray;
        }
    }

    // https://leetcode.com/problems/number-of-subarrays-with-lcm-equal-to-k/description/
    static class Solution2 {
        public int subarrayLCM(int[] nums, int k) {
            int a = 0, n = nums.length;
            for (int i = 0; i < n; i++) {
                int c = 1;
                for (int j = i; j < n && k % nums[j] == 0; j++) {
                    c = task(c, nums[j]);
                    a += (c == k) ? 1 : 0;
                }
            }
            return a;
        }

        int gcd(int a, int b) {
            if (b == 0)
                return a;
            return gcd(b, a % b);
        }

        int task(int a, int b) {
            return (a * b) / gcd(a, b);
        }
    }

    static class Solution3 {
        public int subarrayLCM(int[] nums, int k) {
            int res = 0;
            Map<Integer, Integer> m = new HashMap<>();
            for (int n : nums) {
                Map<Integer, Integer> m1 = new HashMap<>();
                if (k % n == 0) {
                    m.put(n, m.getOrDefault(n, 0) + 1);
                    for (Map.Entry<Integer, Integer> e : m.entrySet()) {
                        int lcm = task(e.getKey(), n);
                        m1.put(lcm, m1.getOrDefault(lcm, 0) + e.getValue());
                    }
                    res += m1.getOrDefault(k, 0);
                }
                m = m1;
            }
            return res;
        }

        int gcd(int a, int b) {
            if (b == 0)
                return a;
            return gcd(b, a % b);
        }

        int task(int a, int b) {
            return (a * b) / gcd(a, b);
        }
    }

}
