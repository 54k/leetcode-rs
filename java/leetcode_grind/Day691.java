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

    
}
