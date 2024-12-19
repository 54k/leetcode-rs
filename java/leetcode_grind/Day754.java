package leetcode_grind;

import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.PriorityQueue;
import java.util.stream.Collectors;

public class Day754 {
    // https://leetcode.com/problems/take-gifts-from-the-richest-pile/description/?envType=daily-question&envId=2024-12-12//
    static class Solution1 {
        public long pickGifts(int[] gifts, int k) {
            List<Integer> giftsList = Arrays.stream(gifts).map(x -> -x).boxed().toList();
            PriorityQueue<Integer> giftsHeap = new PriorityQueue<>(giftsList);
            for (int i = 0; i < k; i++) {
                int maxElement = -giftsHeap.poll();
                giftsHeap.offer(-(int) Math.sqrt(maxElement));
            }
            long numberOfRemainingGifts = 0;
            while (!giftsHeap.isEmpty()) {
                numberOfRemainingGifts -= giftsHeap.poll();
            }
            return numberOfRemainingGifts;
        }
    }

    static class Solution2 {
        public long pickGifts(int[] gifts, int k) {
            int n = gifts.length;
            List<Integer> sortedGifts = Arrays.stream(gifts).sorted().boxed().collect(Collectors.toList());

            for (int i = 0; i < k; i++) {
                int maxElement = sortedGifts.get(n - 1);
                sortedGifts.remove(n - 1);

                int sqrtElement = (int) Math.floor(Math.sqrt(maxElement));
                int spotOfSqrt = Collections.binarySearch(sortedGifts, sqrtElement);
                if (spotOfSqrt < 0) {
                    spotOfSqrt = -(spotOfSqrt + 1);
                }
                sortedGifts.add(spotOfSqrt, sqrtElement);
            }

            long numberOfRemainingGifts = 0;
            for (int gift : sortedGifts) {
                numberOfRemainingGifts += gift;
            }
            return numberOfRemainingGifts;
        }
    }
}
