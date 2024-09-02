package leetcode_grind;

public class Day655 {
    // https://leetcode.com/problems/find-the-student-that-will-replace-the-chalk/description/?envType=daily-question&envId=2024-09-02
    static class Solution1 {
        public int chalkReplacer(int[] chalk, int k) {
            var n = chalk.length;
            var pref = new long[n + 1];
            for (int i = 1; i <= n; i++) {
                pref[i] = (long) chalk[i - 1] + pref[i - 1];
            }
            long K = (long) k % pref[n];
            int lo = -1, hi = n + 2;
            while (lo + 1 < hi) {
                int mid = (lo + hi) / 2;
                if (pref[mid] <= K) {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            return lo;
        }
    }

    static class Solution2 {
        public int chalkReplacer(int[] chalk, int k) {
            int n = chalk.length;
            long[] prefixSum = new long[n];
            prefixSum[0] = chalk[0];
            for (int i = 1; i < n; i++) {
                prefixSum[i] = prefixSum[i - 1] + chalk[i];
            }
            long sum = prefixSum[n - 1];
            long remainingChalk = (k % sum);
            return binarySearch(prefixSum, remainingChalk);
        }

        int binarySearch(long[] arr, long tar) {
            int low = 0, high = arr.length - 1;
            while (low < high) {
                int mid = low + (high - low) / 2;
                if (arr[mid] <= tar) {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            return low;
        }
    }

    // https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/description/
    static class Solution3 {
        public int strStr(String haystack, String needle) {
            int m = needle.length();
            int n = haystack.length();
            if (n < m)
                return -1;

            int[] longest_border = new int[m];
            int prev = 0;
            int i = 1;

            while (i < m) {
                if (needle.charAt(i) == needle.charAt(prev)) {
                    prev += 1;
                    longest_border[i] = prev;
                    i += 1;
                } else {
                    if (prev == 0) {
                        longest_border[i] = 0;
                        i += 1;
                    } else {
                        prev = longest_border[prev - 1];
                    }
                }
            }

            int haystackPointer = 0;
            int needlePointer = 0;

            while (haystackPointer < n) {
                if (haystack.charAt(haystackPointer) == needle.charAt(needlePointer)) {
                    needlePointer += 1;
                    haystackPointer += 1;
                    if (needlePointer == m) {
                        return haystackPointer - m;
                    }
                } else {
                    if (needlePointer == 0) {
                        haystackPointer += 1;
                    } else {
                        needlePointer = longest_border[needlePointer - 1];
                    }
                }
            }

            return -1;
        }
    }
}
