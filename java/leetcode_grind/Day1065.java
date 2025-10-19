package leetcode_grind;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

public class Day1065 {
    // https://leetcode.com/problems/lexicographically-smallest-string-after-applying-operations/description/?envType=daily-question&envId=2025-10-19
    static class Solution1 {
        public String findLexSmallestString(String s, int a, int b) {
            Set<String> vis = new HashSet<>();
            String smallest = s;
            Deque<String> q = new ArrayDeque<>();
            q.offer(s);
            vis.add(s);

            while (!q.isEmpty()) {
                String cur = q.poll();
                if (cur.compareTo(smallest) < 0)
                    smallest = cur;

                StringBuilder sb = new StringBuilder(cur);
                for (int i = 1; i < sb.length(); i += 2) {
                    sb.setCharAt(i, (char) ((sb.charAt(i) - '0' + a) % 10 + '0'));
                }

                String added = sb.toString();
                if (vis.add(added))
                    q.offer(added);

                String rotated = cur.substring(cur.length() - b) + cur.substring(0, cur.length() - b);
                if (vis.add(rotated))
                    q.offer(rotated);
            }
            return smallest;
        }
    }

    // https://leetcode.com/problems/strobogrammatic-number/description/
    static class Solution2 {
        public boolean isStrobogrammatic(String num) {
            Map<Character, Character> rotatedDigits = new HashMap<>(
                    Map.of('0', '0', '1', '1', '6', '9', '8', '8', '9', '6'));

            for (int left = 0, right = num.length() - 1; left <= right; left++, right--) {
                char leftChar = num.charAt(left);
                char rightChar = num.charAt(right);
                if (!rotatedDigits.containsKey(leftChar) || rotatedDigits.get(leftChar) != rightChar) {
                    return false;
                }
            }
            return true;
        }
    }
}