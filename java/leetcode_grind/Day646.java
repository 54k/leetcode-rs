package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day646 {
    // https://leetcode.com/problems/find-the-closest-palindrome/description/?envType=daily-question&envId=2024-08-24
    static class Solution1 {
        public String nearestPalindromic(String n) {
            int len = n.length();
            int i = len % 2 == 0 ? len / 2 - 1 : len / 2;
            long firstHalf = Long.parseLong(n.substring(0, i + 1));

            List<Long> possibilities = new ArrayList<>();
            possibilities.add(halfToPalindrome(firstHalf, len % 2 == 0));
            possibilities.add(halfToPalindrome(firstHalf + 1, len % 2 == 0));
            possibilities.add(halfToPalindrome(firstHalf - 1, len % 2 == 0));
            possibilities.add((long) Math.pow(10, len - 1) - 1);
            possibilities.add((long) Math.pow(10, len) + 1);

            long diff = Long.MAX_VALUE, res = 0, nl = Long.parseLong(n);
            for (long cand : possibilities) {
                if (cand == nl)
                    continue;
                if (Math.abs(cand - nl) < diff) {
                    diff = Math.abs(cand - nl);
                    res = cand;
                } else if (Math.abs(cand - nl) == diff) {
                    res = Math.min(res, cand);
                }
            }
            return String.valueOf(res);
        }

        long halfToPalindrome(long left, boolean even) {
            long res = left;
            if (!even)
                left = left / 10;
            while (left > 0) {
                res = res * 10 + (left % 10);
                left /= 10;
            }
            return res;
        }
    }

    static class Solution2 {
        long convert(long num) {
            String s = Long.toString(num);
            int n = s.length();
            int l = (n - 1) / 2, r = n / 2;
            char[] sArray = s.toCharArray();
            while (l >= 0) {
                sArray[r++] = sArray[l--];
            }
            return Long.parseLong(new String(sArray));
        }

        long nextPalindrome(long num) {
            long left = 0, right = num;
            long ans = Long.MIN_VALUE;
            while (left <= right) {
                long mid = (right - left) / 2 + left;
                long palin = convert(mid);

                if (palin < num) {
                    ans = palin;
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            return ans;
        }

        long previousPalindrome(long num) {
            long left = num, right = (long) 1e18;
            long ans = Long.MIN_VALUE;
            while (left <= right) {
                long mid = (right - left) / 2 + left;
                long palin = convert(mid);
                if (palin > num) {
                    ans = palin;
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            return ans;
        }

        public String nearestPalindromic(String n) {
            long num = Long.parseLong(n);
            long a = nextPalindrome(num);
            long b = previousPalindrome(num);
            if (Math.abs(a - num) <= Math.abs(b - num)) {
                return Long.toString(a);
            }
            return Long.toString(b);
        }
    }
}
