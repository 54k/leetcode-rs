package leetcode_grind;

import java.util.ArrayList;

public class Day334 {
    static class Solution1 {
        // https://leetcode.com/problems/peak-index-in-a-mountain-array/description/
        public int peakIndexInMountainArray(int[] arr) {
            var lo = 0;
            var hi = arr.length - 1;
            while (lo < hi) {
                var mid = (lo + hi) / 2;
                if (arr[mid] <= arr[mid + 1]) {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            return lo;
        }
    }

    // https://leetcode.com/problems/longest-increasing-subsequence/description/
    static class Solution2 {
        public int lengthOfLIS(int[] nums) {
            var seq = new ArrayList<Integer>();
            for (var num : nums) {
                if (seq.isEmpty() || seq.get(seq.size() - 1) < num) {
                    seq.add(num);
                } else {
                    for (var i = 0; i < seq.size(); i++) {
                        if (seq.get(i) >= num) {
                            seq.set(i, num);
                            break;
                        }
                    }
                }
            }
            return seq.size();
        }
    }

    // https://leetcode.com/problems/longest-palindromic-substring/description/
    static class Solution3 {
        public String longestPalindrome(String s) {
            var ans = "";
            var arr = s.toCharArray();
            var dp = new boolean[s.length()][s.length()];

            for (int i = 0; i < dp.length; i++) {
                dp[i][i] = true;
            }

            for (int i = 0; i < dp.length - 1; i++) {
                if (arr[i] == arr[i + 1]) {
                    dp[i][i + 1] = true;
                    ans = s.substring(i, i + 2);
                }
            }

            for (int diff = 2; diff < dp.length; diff++) {
                for (int i = 0; i < arr.length - diff; i++) {
                    var j = i + diff;
                    if (arr[i] == arr[j] && dp[i + 1][j - 1]) {
                        dp[i][j] = true;
                        ans = s.substring(i, j + 1);
                    }
                }
            }
            return ans;
        }
    }
}
