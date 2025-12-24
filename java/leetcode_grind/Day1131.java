package leetcode_grind;

import java.util.Arrays;
import java.util.Collections;

public class Day1131 {
    // https://leetcode.com/problems/apple-redistribution-into-boxes/description/?envType=daily-question&envId=2025-12-24
    static class Solution1 {
        public int minimumBoxes(int[] apple, int[] capacity) {
            int sum = 0;
            for (int a : apple) {
                sum += a;
            }

            Integer[] capArray = new Integer[capacity.length];
            for (int i = 0; i < capacity.length; i++) {
                capArray[i] = capacity[i];
            }

            Arrays.sort(capArray, Collections.reverseOrder());

            int need = 0;
            while (sum > 0) {
                sum -= capArray[need];
                need += 1;
            }
            return need;
        }
    }

}
