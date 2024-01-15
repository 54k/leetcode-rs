package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;

public class Day429 {

    // https://leetcode.com/problems/find-players-with-zero-or-one-losses/description/
    static class Solution1 {
        public List<List<Integer>> findWinners(int[][] matches) {
            var win = new HashMap<Integer, Integer>();
            var lost = new HashMap<Integer, Integer>();

            for (var m : matches) {
                var winner = m[0];
                var loser = m[1];

                win.put(winner, win.getOrDefault(winner, 0) + 1);
                lost.put(loser, lost.getOrDefault(loser, 0) + 1);
            }

            var left = new ArrayList<Integer>();
            var right = new ArrayList<Integer>();

            for (var w : win.keySet()) {
                if (!lost.containsKey(w)) {
                    left.add(w);
                }
            }

            for (var e : lost.entrySet()) {
                if (e.getValue() == 1) {
                    right.add(e.getKey());
                }
            }
            Collections.sort(left);
            Collections.sort(right);
            return List.of(left, right);
        }
    }
}
