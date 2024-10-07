package leetcode_grind;

import java.util.*;

public class Day691 {
    // https://leetcode.com/problems/minimum-string-length-after-removing-substrings/description/?envType=daily-question&envId=2024-10-07
    static class Solution1 {
        public int minLength(String s) {
            while (s.contains("AB") || s.contains("CD")) {
                if (s.contains("AB")) {
                    s = s.replace("AB", "");
                } else if (s.contains("CD")) {
                    s = s.replace("CD", "");
                }
            }
            return s.length();
        }
    }

    static class Solution2 {
        public int minLength(String s) {
            Stack<Character> stack = new Stack<>();
            for (int i = 0; i < s.length(); i++) {
                char currentChar = s.charAt(i);
                if (stack.isEmpty()) {
                    stack.push(currentChar);
                    continue;
                }
                if (currentChar == 'B' && stack.peek() == 'A') {
                    stack.pop();
                } else if (currentChar == 'D' && stack.peek() == 'C') {
                    stack.pop();
                } else {
                    stack.push(currentChar);
                }
            }
            return stack.size();
        }
    }

    static class Solution3 {
        public int minLength(String s) {
            int writePtr = 0;
            char[] charArray = s.toCharArray();
            for (int readPtr = 0; readPtr < s.length(); readPtr++) {
                charArray[writePtr] = charArray[readPtr];

                if (writePtr > 0 && (charArray[writePtr - 1] == 'A' || charArray[writePtr - 1] == 'C')
                        && charArray[writePtr] == charArray[writePtr - 1] + 1) {
                    writePtr--;
                } else {
                    writePtr++;
                }
            }
            return writePtr;
        }
    }

    // https://leetcode.com/problems/find-duplicate-file-in-system/description/
    static class Solution4 {
        public List<List<String>> findDuplicate(String[] paths) {
            var hash = new HashMap<String, List<String>>();
            for (var path : paths) {
                var split = path.split(" ");
                var dir = split[0];
                for (int i = 1; i < split.length; i++) {
                    var file = split[i];
                    var name = file.substring(0, file.indexOf("("));
                    var content = file.substring(file.indexOf("(") + 1, file.indexOf(")"));
                    hash.computeIfAbsent(content, ($) -> new ArrayList<>()).add(dir + "/" + name);
                }
            }
            var res = new ArrayList<List<String>>();
            for (var k : hash.keySet()) {
                if (hash.get(k).size() > 1) {
                    res.add(hash.get(k));
                }
            }
            return res;
        }
    }
}
