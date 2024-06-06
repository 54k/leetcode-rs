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
}
