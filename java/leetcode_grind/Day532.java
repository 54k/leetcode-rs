package leetcode_grind;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Set;

public class Day532 {
    // https://leetcode.com/problems/freedom-trail/description
    // static class Solution1 {
    //     public int findRotateSteps(String ring, String key) {
    //         int ringLen = ring.length();
    //         int keyLen = key.length();

    //         Map<Character, List<Integer>> characterIndices = new HashMap<>();
    //         for (int i = 0; i < ring.length(); i++) {
    //             char ch = ring.charAt(i);
    //             characterIndices.computeIfAbsent(ch, k -> new ArrayList<>()).add(i);
    //         }

    //         PriorityQueue<int[]> heap = new PriorityQueue<>(Comparator.comparingInt(a -> a[0]));
    //         heap.offer(new int[] { 0, 0, 0 });

    //         Set<Pair<Integer, Integer>> seen = new HashSet<>();

    //         int totalSteps = 0;
    //         while (!heap.isEmpty()) {
    //             int[] state = heap.poll();
    //             totalSteps = state[0];
    //             int ringIndex = state[1];
    //             int keyIndex = state[2];

    //             if (keyIndex == keyLen) {
    //                 break;
    //             }

    //             Pair<Integer, Integer> currentState = new Pair<>(ringIndex, keyIndex);
    //             if (seen.contains(currentState)) {
    //                 continue;
    //             }
    //             seen.add(currentState);

    //             for (int nextIndex : characterIndices.get(key.charAt(keyIndex))) {
    //                 heap.offer(
    //                         new int[] { totalSteps + countSteps(ringIndex, nextIndex, ringLen), nextIndex,
    //                                 keyIndex + 1 });
    //             }
    //         }

    //         return totalSteps + keyLen;
    //     }

    //     int countSteps(int cur, int next, int n) {
    //         int lr = Math.abs(cur - next);
    //         int rl = n - lr;
    //         return Math.min(lr, rl);
    //     }
    // }
}
