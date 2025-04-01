package leetcode_grind;

import java.util.Arrays;

public class Day863 {
    // https://leetcode.com/problems/put-marbles-in-bags/description/?envType=daily-question&envId=2025-03-31
    static class Solution1 {
        public long putMarbles(int[] weights, int k) {
            int n = weights.length;
            int[] pairWeights = new int[n - 1];

            for (int i = 0; i < n - 1; i++) {
                pairWeights[i] = weights[i] + weights[i + 1];
            }
            Arrays.sort(pairWeights, 0, n - 1);

            long answer = 0;
            for (int i = 0; i < k - 1; i++) {
                answer += pairWeights[n - 2 - i] - pairWeights[i];
            }

            return answer;
        }
    }
}
