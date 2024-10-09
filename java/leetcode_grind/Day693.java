package leetcode_grind;

import java.util.*;

public class Day693 {
    // https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/description/?envType=daily-question&envId=2024-10-09
    static class Solution1 {
        public int minAddToMakeValid(String s) {
            int ads = 0;
            int brac = 0;
            for (char c : s.toCharArray()) {
                if (c == '(') {
                    brac++;
                } else {
                    if (brac > 0) {
                        brac--;
                    } else {
                        ads++;
                    }
                }
            }
            return ads + brac;
        }
    }

    // https://leetcode.com/problems/add-bold-tag-in-string/description/
    static class Solution2 {
        public String addBoldTag(String s, String[] words) {
            var bold = new boolean[s.length()];
            for (var w : words) {
                var pos = 0;
                while (s.indexOf(w, pos) != -1) {
                    var start = s.indexOf(w, pos);
                    var end = start + w.length();
                    for (int i = start; i < end; i++) {
                        bold[i] = true;
                    }
                    pos = start + 1;
                }
            }
            var sb = new StringBuilder();
            for (int i = 0; i < s.length(); i++) {
                if (bold[i] && (i == 0 || bold[i - 1] != bold[i])) {
                    sb.append("<b>");
                }
                sb.append(s.charAt(i));
                if (bold[i] && (i == s.length() - 1 || bold[i + 1] != bold[i])) {
                    sb.append("</b>");
                }
            }
            return sb.toString();
        }
    }

    // https://leetcode.com/problems/tag-validator/description/
    static class Solution3 {
        Stack<String> stack = new Stack<>();
        boolean contains_tag = false;

        boolean isValidTagName(String s, boolean ending) {
            if (s.length() < 1 || s.length() > 9) {
                return false;
            }
            for (int i = 0; i < s.length(); i++) {
                if (!Character.isUpperCase(s.charAt(i))) {
                    return false;
                }
            }
            if (ending) {
                if (!stack.isEmpty() && stack.peek().equals(s)) {
                    stack.pop();
                } else {
                    return false;
                }
            } else {
                contains_tag = true;
                stack.push(s);
            }
            return true;
        }

        boolean isValidCData(String s) {
            return s.indexOf("[CDATA[") == 0;
        }

        public boolean isValid(String code) {
            if (code.charAt(0) != '<' || code.charAt(code.length() - 1) != '>') {
                return false;
            }

            for (int i = 0; i < code.length(); i++) {
                boolean ending = false;
                int closeIndex;
                if (stack.isEmpty() && contains_tag) {
                    return false;
                }
                if (code.charAt(i) == '<') {
                    if (!stack.isEmpty() && code.charAt(i + 1) == '!') {
                        closeIndex = code.indexOf("]]>", i + 1);
                        if (closeIndex < 0 || !isValidCData(code.substring(i + 2, closeIndex))) {
                            return false;
                        }
                    } else {
                        if (code.charAt(i + 1) == '/') {
                            i++;
                            ending = true;
                        }
                        closeIndex = code.indexOf('>', i + 1);
                        if (closeIndex < 0 || !isValidTagName(code.substring(i + 1, closeIndex), ending)) {
                            return false;
                        }
                    }
                    i = closeIndex;
                }
            }
            return stack.isEmpty() && contains_tag;
        }
    }
}
