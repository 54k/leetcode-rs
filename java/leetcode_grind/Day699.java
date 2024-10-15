package leetcode_grind;

public class Day699 {
    // https://leetcode.com/problems/separate-black-and-white-balls/description/?envType=daily-question&envId=2024-10-15
    static class Solution1 {
        public long minimumSteps(String s) {
            var arr = s.toCharArray();
            var w = 0;
            var b = 0;
            var ans = 0l;
            while (b < arr.length) {
                if (arr[b] == '0') {
                    var t = arr[w];
                    arr[w] = arr[b];
                    arr[b] = t;
                    ans += b - w;
                    w++;
                }
                b++;
            }
            return ans;
        }
    }
}
