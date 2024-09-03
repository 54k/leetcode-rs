package leetcode_grind;

public class Day656 {
    // https://leetcode.com/problems/sum-of-digits-of-string-after-convert/description/?envType=daily-question&envId=2024-09-03
    static class Solution1 {
        public int getLucky(String s, int k) {
            int currentNumber = 0;
            for (char ch : s.toCharArray()) {
                int position = ch - 'a' + 1;
                while (position > 0) {
                    currentNumber += position % 10;
                    position /= 10;
                }
            }
            for (int i = 1; i < k; i++) {
                int digitSum = 0;
                while (currentNumber > 0) {
                    digitSum += currentNumber % 10;
                    currentNumber /= 10;
                }
                currentNumber = digitSum;
            }
            return currentNumber;
        }
    }

    static class Solution2 {
        public int getLucky(String s, int k) {
            String numericString = "";
            for (char ch : s.toCharArray()) {
                numericString += Integer.toString(ch - 'a' + 1);
            }
            while (k-- > 0) {
                int digitSum = 0;
                for (char digit : numericString.toCharArray()) {
                    digitSum += digit - '0';
                }
                numericString = Integer.toString(digitSum);
            }
            return Integer.parseInt(numericString);
        }
    }

    // https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/description/
    static class Solution3 {
        int hashValue(String string, int RADIX, int MOD, int m) {
            long ans = 0;
            long factor = 1;
            for (int i = m - 1; i >= 0; i--) {
                ans = (ans + (string.charAt(i) - 'a') * factor) % MOD;
                factor = (factor * RADIX) % MOD;
            }
            return (int) ans;
        }

        public int strStr(String haystack, String needle) {
            int m = needle.length();
            int n = haystack.length();
            if (n < m)
                return -1;

            int RADIX = 26;
            int MOD = 100000033;
            long MAX_WEIGHT = 1;

            for (int i = 0; i < m; i++)
                MAX_WEIGHT = (MAX_WEIGHT * RADIX) % MOD;

            long hashNeedle = hashValue(needle, RADIX, MOD, m), hashHay = 0;

            for (int windowStart = 0; windowStart <= n - m; windowStart++) {
                if (windowStart == 0) {
                    hashHay = hashValue(haystack, RADIX, MOD, m);
                } else {
                    hashHay = (((hashHay * RADIX) % MOD) -
                            (((int) (haystack.charAt(windowStart - 1) - 'a') * MAX_WEIGHT) % MOD) +
                            (int) (haystack.charAt(windowStart + m - 1) - 'a') + MOD) % MOD;
                }

                if (hashNeedle == hashHay) {
                    for (int i = 0; i < m; i++) {
                        if (needle.charAt(i) != haystack.charAt(i + windowStart)) {
                            break;
                        }
                        if (i == m - 1) {
                            return windowStart;
                        }
                    }
                }
            }

            return -1;
        }
    }
}
