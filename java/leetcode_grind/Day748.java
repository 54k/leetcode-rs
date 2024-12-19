package leetcode_grind;

public class Day748 {
    // https://leetcode.com/problems/move-pieces-to-obtain-a-string/description/?envType=daily-question&envId=2024-12-05
    static class Solution1 {
        public boolean canChange(String start, String target) {
            String pat1 = "", pat2 = "";

            int n = start.length();
            for (int i = 0; i < n; i++) {
                var c1 = start.charAt(i);
                if (c1 != '_') {
                    pat1 += c1;
                }

                var c2 = target.charAt(i);
                if (c2 != '_') {
                    pat2 += c2;
                }
            }

            if (!pat1.equals(pat2)) {
                return false;
            }

            int i = 0, j = 0;
            while (i < n && j < n) {
                var c1 = start.charAt(i);
                while (i < n - 1 && c1 == '_') {
                    c1 = start.charAt(++i);
                }
                var c2 = target.charAt(j);
                while (j < n - 1 && c2 == '_') {
                    c2 = target.charAt(++j);
                }

                if (c1 != c2) {
                    return false;
                }
                if (c1 == 'L' && i < j) {
                    return false;
                } else if (c1 == 'R' && i > j) {
                    return false;
                }
                i++;
                j++;
            }
            return true;
        }
    }

}
