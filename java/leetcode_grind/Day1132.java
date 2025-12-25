package leetcode_grind;

import java.util.Arrays;
import java.util.Collections;

public class Day1132 {
    // https://leetcode.com/problems/maximize-happiness-of-selected-children/description/?envType=daily-question&envId=2025-12-25
    static class Solution1 {
        public long maximumHappinessSum(int[] happiness, int k) {
            int happinessSize = happiness.length;
            Integer[] happinessArray = new Integer[happinessSize];
            for (int i = 0; i < happinessSize; i++) {
                happinessArray[i] = happiness[i];
            }

            Arrays.sort(happinessArray, Collections.reverseOrder());

            long totalHappinessSum = 0;
            int turns = 0;

            for (int i = 0; i < k; i++) {
                totalHappinessSum += Math.max(happinessArray[i] - turns, 0);
                turns++;
            }
            return totalHappinessSum;
        }
    }

}
