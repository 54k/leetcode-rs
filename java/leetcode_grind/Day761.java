package leetcode_grind;

import java.util.Stack;

public class Day761 {
    // https://leetcode.com/problems/max-chunks-to-make-sorted/description/
    static class Solution1 {
        public int maxChunksToSorted(int[] arr) {
            int n = arr.length;
            int[] prefixMax = arr.clone();
            int[] suffixMin = arr.clone();

            for (int i = 1; i < n; i++) {
                prefixMax[i] = Math.max(prefixMax[i - 1], prefixMax[i]);
            }

            for (int i = n - 2; i >= 0; i--) {
                suffixMin[i] = Math.min(suffixMin[i + 1], suffixMin[i]);
            }

            int chunks = 0;
            for (int i = 0; i < n; i++) {
                if (i == 0 || suffixMin[i] > prefixMax[i - 1]) {
                    chunks++;
                }
            }
            return chunks;
        }
    }

    static class Solution2 {
        public int maxChunksToSorted(int[] arr) {
            int n = arr.length;
            int chunks = 0, prefixSum = 0, sortedPrefixSum = 0;

            for (int i = 0; i < n; i++) {
                prefixSum += arr[i];
                sortedPrefixSum += i;

                if (prefixSum == sortedPrefixSum) {
                    chunks++;
                }
            }
            return chunks;
        }
    }

    static class Solution3 {
        public int maxChunksToSorted(int[] arr) {
            int n = arr.length;
            Stack<Integer> monotonicStack = new Stack<>();
            for (int i = 0; i < n; i++) {
                if (monotonicStack.isEmpty() || arr[i] > monotonicStack.peek()) {
                    monotonicStack.push(arr[i]);
                } else {
                    int maxElement = monotonicStack.peek();
                    while (!monotonicStack.isEmpty() && arr[i] < monotonicStack.peek()) {
                        monotonicStack.pop();
                    }
                    monotonicStack.push(maxElement);
                }
            }
            return monotonicStack.size();
        }
    }

    static class Solution4 {
        public int maxChunksToSorted(int[] arr) {
            int n = arr.length;
            int chunks = 0, maxElement = 0;
            for (int i = 0; i < n; i++) {
                maxElement = Math.max(maxElement, arr[i]);
                if (maxElement == i) {
                    chunks++;
                }
            }
            return chunks;
        }
    }
}
