package leetcode_grind;

import java.util.HashMap;
import java.util.HashSet;
import java.util.List;

public class Day398 {
    // https://leetcode.com/problems/destination-city/description
    static class Solution1 {
        public String destCity(List<List<String>> paths) {
            var dest = new HashMap<String, String>();
            var cities = new HashSet<String>();
            for (var p : paths) {
                dest.put(p.get(0), p.get(1));
                cities.add(p.get(0));
                cities.add(p.get(1));
            }

            for (var c : cities) {
                if (!dest.containsKey(c)) {
                    return c;
                }
            }
            return "";
        }
    }

    // https://leetcode.com/problems/total-appeal-of-a-string/description/
    static class Solution2 {
        public long appealSum(String s) {
            var total = 0;
            var prev = new int[26];
            long ans = 0;
            for (var i = 0; i < s.length(); i++) {
                total += i + 1 - prev[s.charAt(i) - 'a'];
                prev[s.charAt(i) - 'a'] = i + 1;
                ans += total;
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/bulls-and-cows/
    class Solution3 {
        public String getHint1(String secret, String guess) {
            HashMap<Character, Integer> h = new HashMap<>();
            for (char s : secret.toCharArray()) {
                h.put(s, h.getOrDefault(s, 0) + 1);
            }

            int bulls = 0, cows = 0;
            int n = guess.length();

            for (int idx = 0; idx < n; ++idx) {
                char ch = guess.charAt(idx);
                if (h.containsKey(ch)) {
                    if (ch == secret.charAt(idx)) {
                        bulls++;

                        if (h.get(ch) <= 0) {
                            cows--;
                        }
                    } else {
                        if (h.get(ch) > 0) {
                            cows++;
                        }
                    }
                    h.put(ch, h.get(ch) - 1);
                }
            }

            return String.format("%sA%sB", bulls, cows);
        }

        public String getHint2(String secret, String guess) {
            int bulls = 0, cows = 0;
            HashMap<Character, Integer> h = new HashMap<>();
            int n = guess.length();

            for (int idx = 0; idx < n; idx++) {
                char s = secret.charAt(idx);
                char g = guess.charAt(idx);

                if (s == g) {
                    bulls++;
                } else {
                    if (h.getOrDefault(s, 0) < 0) {
                        cows++;
                    }
                    if (h.getOrDefault(g, 0) > 0) {
                        cows++;
                    }

                    h.put(s, h.getOrDefault(s, 0) + 1);
                    h.put(g, h.getOrDefault(g, 0) - 1);
                }
            }

            return String.format("%sA%sB", bulls, cows);
        }
    }
}
