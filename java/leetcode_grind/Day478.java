package leetcode_grind;

public class Day478 {
    // https://leetcode.com/problems/minimum-length-of-string-after-deleting-similar-ends/
    static class Solution1 {
        public int minimumLength(String s) {
            var i = 0;
            var j = s.length() - 1;
            var arr = s.toCharArray();
            while (i < j) {
                if (arr[i] != arr[j]) {
                    break;
                }
                while (i <= j && arr[i] == arr[j])
                    i++;
                var t = j;
                while (i <= j && arr[j] == arr[t])
                    j--;
            }
            return j - i + 1;
        }
    }
}
