package leetcode_grind;

public class Day1117 {
    // https://leetcode.com/problems/count-the-number-of-computer-unlocking-permutations/description/?envType=daily-question&envId=2025-12-10
    static class Solution1 {
        public int countPermutations(int[] complexity) {
            int n = complexity.length;
            for (int i = 1; i < n; i++) {
                if (complexity[i] <= complexity[0]) {
                    return 0;
                }
            }

            int ans = 1;
            int mod = 1000000007;
            for (int i = 2; i < n; i++) {
                ans = (int) (((long) ans * i) % mod);
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/minimize-product-sum-of-two-arrays/description/
    static class Solution2 {
        public int minProductSum(int[] nums1, int[] nums2) {
            int[] counter1 = new int[101], counter2 = new int[101];
            for (int num : nums1) {
                counter1[num]++;
            }
            for (int num : nums2) {
                counter2[num]++;
            }

            int p1 = 0, p2 = 100, ans = 0;
            int occ;

            while (p1 < 101 && p2 > 0) {
                while (p1 < 101 && counter1[p1] == 0) {
                    p1++;
                }
                while (p2 > 0 && counter2[p2] == 0) {
                    p2--;
                }
                if (p1 == 101 || p2 == 0) {
                    break;
                }
                occ = Math.min(counter1[p1], counter2[p2]);
                ans += occ * p1 * p2;
                counter1[p1] -= occ;
                counter2[p2] -= occ;
            }

            return ans;
        }
    }
}
