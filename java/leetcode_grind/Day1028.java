package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day1028 {
    // https://leetcode.com/problems/vowels-game-in-a-string/description/?envType=daily-question&envId=2025-09-12
    static class Solution1 {
        public boolean doesAliceWin(String s) {
            return s.chars().anyMatch(c -> {
                return "aeiou".indexOf(c) != -1;
            });
        }
    }

    // https://leetcode.com/problems/long-pressed-name/description/
    static class Solution2 {
        static class Group {
            String key;
            List<Integer> count;

            Group(String k, List<Integer> c) {
                key = k;
                count = c;
            }
        }

        public boolean isLongPressedName(String name, String typed) {
            Group g1 = groupify(name);
            Group g2 = groupify(typed);
            if (!g1.key.equals(g2.key)) {
                return false;
            }
            for (int i = 0; i < g1.count.size(); i++) {
                if (g1.count.get(i) > g2.count.get(i)) {
                    return false;
                }
            }
            return true;
        }

        Group groupify(String S) {
            StringBuilder key = new StringBuilder();
            List<Integer> count = new ArrayList<>();
            int anchor = 0;
            int N = S.length();
            for (int i = 0; i < N; i++) {
                if (i == N - 1 || S.charAt(i) != S.charAt(i + 1)) {
                    key.append(S.charAt(i));
                    count.add(i - anchor + 1);
                    anchor = i + 1;
                }
            }
            return new Group(key.toString(), count);
        }
    }

    static class Solution3 {
        public boolean isLongPressedName(String name, String typed) {
            int np = 0, tp = 0;
            char[] name_chars = name.toCharArray();
            char[] typed_chars = typed.toCharArray();

            while (np < name_chars.length && tp < typed_chars.length) {
                if (name_chars[np] == typed_chars[tp]) {
                    np += 1;
                    tp += 1;
                } else if (tp >= 1 && typed_chars[tp] == typed_chars[tp - 1]) {
                    tp += 1;
                } else {
                    return false;
                }
            }

            if (np != name_chars.length) {
                return false;
            } else {
                while (tp < typed_chars.length) {
                    if (typed_chars[tp] != typed_chars[tp - 1]) {
                        return false;
                    }
                    tp += 1;
                }
            }

            return true;
        }
    }
}
