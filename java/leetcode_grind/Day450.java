package leetcode_grind;

import java.util.HashMap;
import java.util.List;
import java.util.PriorityQueue;

public class Day450 {
    // https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/description/
    static class Solution1 {
        public int[] smallestRange(List<List<Integer>> nums) {
            int n = nums.size();

            int minx = 0;
            int miny = Integer.MAX_VALUE;
            int max = Integer.MIN_VALUE;

            int[] next = new int[n];

            PriorityQueue<Integer> pq = new PriorityQueue<>(
                    (a, b) -> nums.get(a).get(next[a]) - nums.get(b).get(next[b]));
            for (int i = 0; i < n; ++i) {
                pq.offer(i);
                max = Math.max(max, nums.get(i).get(0));
            }

            outer: for (int i = 0; i < n; ++i) {
                for (int j = 0; j < nums.get(i).size(); ++j) {
                    int mini = pq.poll();

                    if (max - nums.get(mini).get(next[mini]) < miny - minx) {
                        minx = nums.get(mini).get(next[mini]);
                        miny = max;
                    }

                    next[mini] += 1;
                    if (next[mini] == nums.get(mini).size()) {
                        break outer;
                    }

                    pq.offer(mini);
                    max = Math.max(max, nums.get(mini).get(next[mini]));
                }
            }

            return new int[] { minx, miny };
        }
    }

    // https://leetcode.com/problems/first-unique-character-in-a-string/description/
    static class Solution2 {
        public int firstUniqChar(String s) {
            var map = new HashMap<Character, Integer>();
            for (int i = 0; i < s.length(); ++i) {
                map.put(s.charAt(i), map.getOrDefault(s.charAt(i), 0) + 1);
            }
            int idx = -1;
            for (int i = 0; i < s.length(); ++i) {
                if (map.get(s.charAt(i)) == 1) {
                    return i;
                }
            }
            return idx;
        }
    }
}
