package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.PriorityQueue;

public class Day492 {
    // https://leetcode.com/problems/task-scheduler/
    static class Solution1 {
        public int leastInterval(char[] tasks, int n) {
            int[] freq = new int[26];
            for (var t : tasks) {
                freq[t - 'A']++;
            }

            var pq = new PriorityQueue<Integer>(Collections.reverseOrder());
            for (int i = 0; i < 26; i++) {
                if (freq[i] > 0) {
                    pq.offer(freq[i]);
                }
            }

            int time = 0;
            while (!pq.isEmpty()) {
                int cycle = n + 1;
                List<Integer> store = new ArrayList<>();
                int taskCount = 0;

                while (cycle-- > 0 && !pq.isEmpty()) {
                    int currentFreq = pq.poll();
                    if (currentFreq > 1) {
                        store.add(currentFreq - 1);
                    }
                    taskCount++;
                }

                store.forEach(pq::offer);
                time += (pq.isEmpty() ? taskCount : n + 1);
            }
            return time;
        }
    }
}
