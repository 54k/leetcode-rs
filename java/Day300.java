// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/description/
public class Day300 {
    public int minAddToMakeValid(String s) {
        var ans = 0;
        var bal = 0;
        for (var ch : s.toCharArray()) {
            if (ch == '(') {
                bal++;
            } else {
                bal--;
            }
            if (bal == -1) {
                bal++;
                ans++;
            }
        }
        return ans + bal;
    }
}