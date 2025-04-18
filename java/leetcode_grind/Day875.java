package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Day875 {
    // https://leetcode.com/problems/find-the-count-of-good-integers/description/?envType=daily-question&envId=2025-04-12
    static class Solution1 {
        public long countGoodIntegers(int n, int k) {
            Set<String> dict = new HashSet<>();
            int base = (int) Math.pow(10, (n - 1) / 2);
            int skip = n & 1;

            for (int i = base; i < base * 10; i++) {
                String s = Integer.toString(i);
                s += new StringBuilder(s).reverse().substring(skip);
                long palindromicInteger = Long.parseLong(s);

                if (palindromicInteger % k == 0) {
                    char[] chars = s.toCharArray();
                    Arrays.sort(chars);
                    dict.add(new String(chars));
                }
            }

            long[] factorial = new long[n + 1];
            factorial[0] = 1;
            for (int i = 1; i <= n; i++) {
                factorial[i] = factorial[i - 1] * i;
            }
            long ans = 0;

            for (String s : dict) {
                int[] cnt = new int[10];
                for (char c : s.toCharArray()) {
                    cnt[c - '0']++;
                }

                long tot = (n - cnt[0]) * factorial[n - 1];
                for (int x : cnt) {
                    tot /= factorial[x];
                }
                ans += tot;
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/powerful-integers/description/
    static class Solution2 {
        public List<Integer> powerfulIntegers(int x, int y, int bound) {
            int a = x == 1 ? bound : (int) (Math.log(bound) / Math.log(x));
            int b = y == 1 ? bound : (int) (Math.log(bound) / Math.log(y));

            Set<Integer> powerfulIntegers = new HashSet<Integer>();

            for (int i = 0; i <= a; i++) {
                for (int j = 0; j <= b; j++) {
                    int value = (int) Math.pow(x, i) + (int) Math.pow(y, j);

                    if (value <= bound) {
                        powerfulIntegers.add(value);
                    }
                    if (y == 1) {
                        break;
                    }
                }

                if (x == 1) {
                    break;
                }
            }

            return new ArrayList<Integer>(powerfulIntegers);
        }
    }
}
