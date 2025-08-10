package leetcode_grind;

import java.util.Arrays;
import java.util.List;

public class Day995 {
    // https://leetcode.com/problems/reordered-power-of-2/description/?envType=daily-question&envId=2025-08-10
    static class Solution1 {
        public boolean reorderedPowerOf2(int N) {
            String S = Integer.toString(N);
            int[] A = new int[S.length()];
            for (int i = 0; i < S.length(); i++) {
                A[i] = S.charAt(i) - '0';
            }
            return permutations(A, 0);
        }

        boolean permutations(int[] A, int start) {
            if (start == A.length) {
                return isPowerOfTwo(A);
            }
            for (int i = start; i < A.length; i++) {
                swap(A, start, i);
                if (permutations(A, start + 1)) {
                    return true;
                }
                swap(A, start, i);
            }
            return false;
        }

        boolean isPowerOfTwo(int[] A) {
            if (A[0] == 0)
                return false;
            int N = 0;
            for (int x : A) {
                N = 10 * N + x;
            }
            while (N > 0 && ((N & 1) == 0)) {
                N >>= 1;
            }
            return N == 1;
        }

        void swap(int[] A, int i, int j) {
            int t = A[i];
            A[i] = A[j];
            A[j] = t;
        }
    }

    static class Solution2 {
        public boolean reorderedPowerOf2(int N) {
            int[] A = count(N);
            for (int i = 0; i < 31; i++) {
                if (Arrays.equals(A, count(1 << i))) {
                    return true;
                }
            }
            return false;
        }

        int[] count(int N) {
            int[] ans = new int[10];
            while (N > 0) {
                ans[N % 10]++;
                N /= 10;
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/sequence-reconstruction/description/
    static class Solution3 {
        public boolean sequenceReconstruction(int[] nums, List<List<Integer>> sequences) {
            if (sequences == null || sequences.isEmpty()) {
                return false;
            }

            int numsLen = nums.length;
            int[] idx = new int[numsLen + 1];
            boolean[] pairs = new boolean[numsLen];

            for (int i = 0; i < numsLen; i++) {
                idx[nums[i]] = i;
            }

            for (List<Integer> seq : sequences) {
                for (int i = 0; i < seq.size(); i++) {
                    if (seq.get(i) > numsLen || seq.get(i) < 0) {
                        return false;
                    }

                    if (i > 0 && idx[seq.get(i - 1)] >= idx[seq.get(i)]) {
                        return false;
                    }

                    if (i > 0 && idx[seq.get(i - 1)] + 1 == idx[seq.get(i)]) {
                        pairs[idx[seq.get(i - 1)]] = true;
                    }
                }
            }

            for (int i = 0; i < numsLen - 1; i++) {
                if (!pairs[i]) {
                    return false;
                }
            }
            return true;
        }
    }
}
