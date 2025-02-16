package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day820 {
    // https://leetcode.com/problems/construct-the-lexicographically-largest-valid-sequence/description/?envType=daily-question&envId=2025-02-16
    static class Solution1 {
        public int[] constructDistancedSequence(int n) {
            int[] resultSequence = new int[n * 2 - 1];
            boolean[] isNumberUsed = new boolean[n + 1];
            findLexicographicallyLargestSequence(0, resultSequence, isNumberUsed, n);
            return resultSequence;
        }

        boolean findLexicographicallyLargestSequence(
                int currentIndex,
                int[] resultSequence,
                boolean[] isNumberUsed,
                int targetNumber) {
            if (currentIndex == resultSequence.length) {
                return true;
            }

            if (resultSequence[currentIndex] != 0) {
                return findLexicographicallyLargestSequence(currentIndex + 1, resultSequence, isNumberUsed,
                        targetNumber);
            }

            for (int numberToPlace = targetNumber; numberToPlace >= 1; numberToPlace--) {
                if (isNumberUsed[numberToPlace])
                    continue;

                isNumberUsed[numberToPlace] = true;
                resultSequence[currentIndex] = numberToPlace;

                if (numberToPlace == 1) {
                    if (findLexicographicallyLargestSequence(currentIndex + 1, resultSequence, isNumberUsed,
                            targetNumber)) {
                        return true;
                    }
                } else if (currentIndex + numberToPlace < resultSequence.length
                        && resultSequence[currentIndex + numberToPlace] == 0) {
                    resultSequence[currentIndex + numberToPlace] = numberToPlace;

                    if (findLexicographicallyLargestSequence(currentIndex + 1, resultSequence, isNumberUsed,
                            targetNumber)) {
                        return true;
                    }

                    resultSequence[currentIndex + numberToPlace] = 0;
                }
                resultSequence[currentIndex] = 0;
                isNumberUsed[numberToPlace] = false;
            }
            return false;
        }
    }

    // https://leetcode.com/problems/design-most-recently-used-queue/description/?envType=weekly-question&envId=2025-02-15
    static class MRUQueue1 {
        List<Integer> queue = new ArrayList<>();

        public MRUQueue1(int n) {
            for (int number = 1; number <= n; number++) {
                queue.add(number);
            }
        }

        public int fetch(int k) {
            int value = queue.get(k - 1);
            queue.remove(k - 1);
            queue.add(value);
            return value;
        }
    }

    /**
     * Your MRUQueue object will be instantiated and called as such:
     * MRUQueue obj = new MRUQueue(n);
     * int param_1 = obj.fetch(k);
     */

}
