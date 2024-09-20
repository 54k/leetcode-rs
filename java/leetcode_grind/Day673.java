package leetcode_grind;

import java.util.*;

public class Day673 {
    // https://leetcode.com/problems/shortest-palindrome/description/?envType=daily-question&envId=2024-09-20
    static class Solution1 {
        public String shortestPalindrome(String s) {
            if (s.isEmpty())
                return s;
            var isPalindrome = new Object() {
                boolean apply(int i) {
                    int j = 0;
                    int end = i / 2;
                    while (i >= end) {
                        if (s.charAt(i) != s.charAt(j)) {
                            return false;
                        }
                        i--;
                        j++;
                    }
                    return true;
                }
            };
            int n = s.length();
            int max = 0;
            for (int i = 0; i < n; i++) {
                if (isPalindrome.apply(i)) {
                    max = i;
                }
            }
            String rest = s.substring(max + 1);
            var r = new StringBuilder(rest).reverse().toString();
            return r + s;
        }
    }

    static class Solution2 {
        public String shortestPalindrome(String s) {
            int length = s.length();
            String reversedString = new StringBuilder(s).reverse().toString();
            for (int i = 0; i < length; i++) {
                if (s.substring(0, length - i).equals(reversedString.substring(i))) {
                    return new StringBuilder(reversedString.substring(0, i))
                            .append(s)
                            .toString();
                }
            }
            return "";
        }
    }

    static class Solution3 {
        public String shortestPalindrome(String s) {
            int length = s.length();
            if (length == 0) {
                return s;
            }
            int left = 0;
            for (int right = length - 1; right >= 0; right--) {
                if (s.charAt(right) == s.charAt(left)) {
                    left++;
                }
            }
            if (left == length) {
                return s;
            }
            String nonPalindromeSuffix = s.substring(left);
            StringBuilder reverseSuffix = new StringBuilder(nonPalindromeSuffix).reverse();
            return reverseSuffix
                    .append(shortestPalindrome(s.substring(0, left)))
                    .append(nonPalindromeSuffix)
                    .toString();
        }
    }

    static class Solution4 {
        public String shortestPalindrome(String s) {
            long hashBase = 29;
            long modValue = (long) 1e9 + 7;
            long forwardHash = 0, reverseHash = 0, powerValue = 1;
            int palindromeEndIndex = -1;

            for (int i = 0; i < s.length(); i++) {
                char currentChar = s.charAt(i);
                forwardHash = (forwardHash * hashBase + (currentChar - 'a' + 1)) % modValue;
                reverseHash = (reverseHash + (currentChar - 'a' + 1) * powerValue) % modValue;
                powerValue = (powerValue * hashBase) % modValue;

                if (forwardHash == reverseHash) {
                    palindromeEndIndex = i;
                }
            }

            String suffix = s.substring(palindromeEndIndex + 1);
            StringBuilder reversedSuffix = new StringBuilder(suffix).reverse();
            return reversedSuffix.append(s).toString();
        }
    }

    // https://leetcode.com/problems/maximum-deletions-on-a-string/description/
    // https://leetcode.com/problems/maximum-deletions-on-a-string/solutions/2649459/kmp-and-dp-o-n-2/
    static class Solution6 {
        List<Integer>[] al;
        int[] dp;

        public int deleteString(String s) {
            this.al = new ArrayList[s.length()];
            for (int i = 0; i < s.length(); i++) {
                String sub = s.substring(i);
                al[i] = KMP(sub);
            }
            this.dp = new int[s.length() + 1];
            Arrays.fill(dp, -1);
            return solve(0, s.length());
        }

        int solve(int i, int n) {
            if (i >= n)
                return 0;
            if (al[i].size() == 0)
                return 1;
            if (dp[i] != -1)
                return dp[i];
            int ans = 0;
            for (int next : al[i]) {
                ans = Math.max(ans, 1 + solve(i + next, n));
            }
            return dp[i] = ans;
        }

        List<Integer> KMP(String s) {
            int[] LPS = new int[s.length()];
            for (int i = 1; i < s.length(); i++) {
                int j = LPS[i - 1];
                while (j > 0 && s.charAt(j) != s.charAt(i))
                    j = LPS[j - 1];
                if (s.charAt(i) == s.charAt(j))
                    j++;
                LPS[i] = j;
            }
            List<Integer> al = new ArrayList<>();
            for (int i = 0; i < s.length(); i++) {
                if (i - LPS[i] + 1 == LPS[i])
                    al.add(LPS[i]);
            }
            return al;
        }
    }

    // https://leetcode.com/problems/maximum-length-of-repeated-subarray/description/
    static class Solution8 {
        public int findLength(int[] nums1, int[] nums2) {
            var m = nums1.length;
            var n = nums2.length;
            var max = 0;
            var dp = new int[m + 1][n + 1];
            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    if (nums1[i] == nums2[j]) {
                        dp[i + 1][j + 1] = 1 + dp[i][j];
                        max = Math.max(max, dp[i + 1][j + 1]);
                    }
                }
            }
            return max;
        }
    }

    // https://leetcode.com/problems/shortest-distance-to-a-character/description/
    static class Solution9 {
        public int[] shortestToChar(String s, char c) {
            int N = s.length();
            int[] ans = new int[N];
            int prev = Integer.MIN_VALUE / 2;

            for (int i = 0; i < N; i++) {
                if (s.charAt(i) == c)
                    prev = i;
                ans[i] = i - prev;
            }

            prev = Integer.MAX_VALUE / 2;
            for (int i = N - 1; i >= 0; i--) {
                if (s.charAt(i) == c)
                    prev = i;
                ans[i] = Math.min(ans[i], prev - i);
            }

            return ans;
        }
    }
}
