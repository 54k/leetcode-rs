package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day1066 {
    // https://leetcode.com/problems/final-value-of-variable-after-performing-operations/description/?envType=daily-question&envId=2025-10-20
    static class Solution1 {
        public int finalValueAfterOperations(String[] operations) {
            int ret = 0;
            for (String op : operations) {
                if (op.contains("-")) {
                    ret--;
                } else {
                    ret++;
                }
            }
            return ret;
        }
    }

    // https://leetcode.com/problems/bulls-and-cows/description/
    static class Solution2 {
        public String getHint(String secret, String guess) {
            Map<Character, Integer> h = new HashMap<>();
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
            return Integer.toString(bulls) + "A" + Integer.toString(cows) + "B";
        }
    }
}
