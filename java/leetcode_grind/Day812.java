package leetcode_grind;

import java.util.HashMap;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.SortedSet;
import java.util.TreeSet;

public class Day812 {
    // https://leetcode.com/problems/design-a-number-container-system/description/?envType=daily-question&envId=2025-02-08
    static class NumberContainers1 {
        Map<Integer, SortedSet<Integer>> nti = new HashMap<>();
        Map<Integer, Integer> itn = new HashMap<>();

        public NumberContainers1() {

        }

        public void change(int index, int number) {
            var prev = itn.put(index, number);
            nti.putIfAbsent(number, new TreeSet<>());
            nti.get(number).add(index);
            if (prev != null && prev != number) {
                nti.get(prev).remove(index);
                if (nti.get(prev).isEmpty()) {
                    nti.remove(prev);
                }
            }
        }

        public int find(int number) {
            if (nti.getOrDefault(number, new TreeSet<>()).isEmpty()) {
                return -1;
            }
            return nti.get(number).iterator().next();
        }
    }

    static class NumberContainers2 {
        // Map to store number -> min heap of indices
        private Map<Integer, PriorityQueue<Integer>> numberToIndices = new HashMap<>();
        // Map to store index -> number
        private Map<Integer, Integer> indexToNumbers = new HashMap<>();

        public NumberContainers2() {

        }

        public void change(int index, int number) {
            indexToNumbers.put(index, number);
            numberToIndices.computeIfAbsent(number, k -> new PriorityQueue<>()).add(index);
        }

        public int find(int number) {
            if (!numberToIndices.containsKey(number)) {
                return -1;
            }
            PriorityQueue<Integer> minHeap = numberToIndices.get(number);
            while (!minHeap.isEmpty()) {
                int index = minHeap.peek();
                if (indexToNumbers.get(index) == number) {
                    return index;
                }
                minHeap.poll();
            }
            return -1;
        }
    }
}
