package leetcode_grind;

import java.util.Arrays;

public class Day1128 {
    // https://leetcode.com/problems/delete-columns-to-make-sorted-ii/description/?envType=daily-question&envId=2025-12-21
    static class Solution1 {
        public int minDeletionSize(String[] A) {
            int N = A.length;
            int W = A[0].length();
            boolean[] cuts = new boolean[N - 1];

            int ans = 0;
            search: for (int j = 0; j < W; j++) {
                for (int i = 0; i < N - 1; i++) {
                    if (!cuts[i] && A[i].charAt(j) > A[i + 1].charAt(j)) {
                        ans++;
                        continue search;
                    }
                }

                for (int i = 0; i < N - 1; i++) {
                    if (A[i].charAt(j) < A[i + 1].charAt(j)) {
                        cuts[i] = true;
                    }
                }
            }

            return ans;
        }
    }

    static class Solution2 {
        public int minDeletionSize(String[] A) {
            int N = A.length;
            int W = A[0].length();
            int ans = 0;

            String[] cur = new String[N];
            for (int j = 0; j < W; j++) {
                String[] cur2 = Arrays.copyOf(cur, N);
                for (int i = 0; i < N; i++) {
                    cur2[i] += A[i].charAt(j);
                }
                if (isSorted(cur2)) {
                    cur = cur2;
                } else {
                    ans++;
                }
            }
            return ans;
        }

        boolean isSorted(String[] A) {
            for (int i = 0; i < A.length - 1; i++) {
                if (A[i].compareTo(A[i + 1]) > 0) {
                    return false;
                }
            }
            return true;
        }
    }

}
