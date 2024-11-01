package leetcode_grind;

public class Day715 {
    // https://leetcode.com/problems/delete-characters-to-make-fancy-string/description/?envType=daily-question&envId=2024-11-01
    static class Solution1 {
        public String makeFancyString(String s) {
            var ans = new StringBuilder();
            ans.append(s.charAt(0));
            for (int i = 1, cnt = 1; i < s.length(); i++) {
                if (s.charAt(i) == ans.charAt(ans.length() - 1)) {
                    cnt++;
                } else {
                    cnt = 1;
                }
                if (cnt < 3) {
                    ans.append(s.charAt(i));
                }
            }
            return ans.toString();
        }
    }
}
