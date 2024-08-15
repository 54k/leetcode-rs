package leetcode_grind;

public class Day637 {
    // https://leetcode.com/problems/lemonade-change/description/
    static class Solution1 {
        public boolean lemonadeChange(int[] bills) {
            var fiveBills = 0;
            var tenBills = 0;
            for (var b : bills) {
                if (b == 5) {
                    fiveBills++;
                } else if (b == 10) {
                    if (fiveBills == 0) {
                        return false;
                    }
                    fiveBills--;
                    tenBills++;
                } else {
                    if (fiveBills > 0 && tenBills > 0) {
                        fiveBills--;
                        tenBills--;
                    } else if (fiveBills >= 3) {
                        fiveBills -= 3;
                    } else {
                        return false;
                    }
                }
            }
            return true;
        }
    }
}
