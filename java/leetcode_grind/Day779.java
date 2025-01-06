package leetcode_grind;

public class Day779 {
    // https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/description/?envType=daily-question&envId=2025-01-06
    static class Solution1 {
        public int[] minOperations(String boxes) {
            int[] answer = new int[boxes.length()];
            for (int currentBox = 0; currentBox < boxes.length(); currentBox++) {
                if (boxes.charAt(currentBox) == '1') {
                    for (int newPosition = 0; newPosition < boxes.length(); newPosition++) {
                        answer[newPosition] += Math.abs(newPosition - currentBox);
                    }
                }
            }
            return answer;
        }
    }
}
