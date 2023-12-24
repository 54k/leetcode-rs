package leetcode_grind;

public class Day407 {

    // https://leetcode.com/problems/string-compression/description/
    static class Solution1 {
        public int compress(char[] chars) {
            int chptr = 0;
            for (int i = 1, group = 1; i <= chars.length; i++) {
                if (i == chars.length || chars[i] != chars[i - 1]) {
                    chars[chptr++] = chars[i - 1];
                    if (group > 1) {
                        for (char c : Integer.toString(group).toCharArray()) {
                            chars[chptr++] = c;
                        }
                    }
                    group = 1;
                } else {
                    group++;
                }
            }
            // System.out.println(Arrays.toString(chars));
            return chptr;
        }
    }
}
