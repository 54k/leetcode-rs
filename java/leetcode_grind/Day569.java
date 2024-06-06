package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;

public class Day569 {
    // https://leetcode.com/problems/hand-of-straights/description
    static class Solution1 {
        public boolean isNStraightHand(int[] hand, int groupSize) {
            var cnt = new HashMap<Integer, Integer>();
            for (var v : hand) {
                cnt.put(v, cnt.getOrDefault(v, 0) + 1);
            }
            Arrays.sort(hand);
            for (var ptr = 0; ptr < hand.length; ptr++) {
                if (cnt.getOrDefault(hand[ptr], 0) == 0) {
                    continue;
                }
                for (var j = hand[ptr]; j < hand[ptr] + groupSize; j++) {
                    if (cnt.getOrDefault(j, 0) > 0) {
                        cnt.put(j, cnt.get(j) - 1);
                        // System.out.printf("Take j %s \n", j);
                    } else {
                        // System.out.printf("No candidate j %s found, exit \n", j);
                        return false;
                    }
                }
                // System.out.printf("End forming group \n");
            }
            return true;
        }
    }

    // https://leetcode.com/problems/hand-of-straights/description
    static class Solution2 {
        public boolean isNStraightHand(int[] hand, int groupSize) {
            if (hand.length % groupSize != 0) {
                return false;
            }

            HashMap<Integer, Integer> cardCount = new HashMap<>();
            for (int card : hand) {
                int count = cardCount.getOrDefault(card, 0);
                cardCount.put(card, count + 1);
            }

            for (int card : hand) {
                int startCard = card;
                while (cardCount.getOrDefault(startCard - 1, 0) > 0) {
                    startCard--;
                }
                while (startCard <= card) {
                    while (cardCount.getOrDefault(startCard, 0) > 0) {
                        for (int nextCard = startCard; nextCard < startCard + groupSize; nextCard++) {
                            if (cardCount.getOrDefault(nextCard, 0) == 0) {
                                return false;
                            }
                            cardCount.put(nextCard, cardCount.get(nextCard) - 1);
                        }
                    }
                    startCard++;
                }
            }
            return true;
        }
    }
}
