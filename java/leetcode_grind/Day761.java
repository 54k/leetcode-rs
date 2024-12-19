package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
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

    // https://leetcode.com/problems/max-chunks-to-make-sorted-ii/description/
    static class Solution5 {
        public int maxChunksToSorted(int[] arr) {
            Map<Integer, Integer> count = new HashMap<>();
            int ans = 0, nonzero = 0;

            int[] expect = arr.clone();
            Arrays.sort(expect);

            for (int i = 0; i < arr.length; i++) {
                int x = arr[i], y = expect[i];

                count.put(x, count.getOrDefault(x, 0) + 1);
                if (count.get(x) == 0)
                    nonzero--;
                if (count.get(x) == 1)
                    nonzero++;

                count.put(y, count.getOrDefault(y, 0) - 1);
                if (count.get(y) == -1)
                    nonzero++;
                if (count.get(y) == 0)
                    nonzero--;

                if (nonzero == 0)
                    ans++;
            }
            return ans;
        }
    }

    static class Solution6 {
        public int maxChunksToSorted(int[] arr) {
            Map<Integer, Integer> count = new HashMap<>();
            List<Pair> counted = new ArrayList<>();
            for (int x : arr) {
                count.put(x, count.getOrDefault(x, 0) + 1);
                counted.add(new Pair(x, count.get(x)));
            }

            List<Pair> expect = new ArrayList<>(counted);
            Collections.sort(expect, (a, b) -> a.compare(b));

            Pair cur = counted.get(0);
            int ans = 0;
            for (int i = 0; i < arr.length; i++) {
                Pair X = counted.get(i), Y = expect.get(i);
                if (X.compare(cur) > 0)
                    cur = X;
                if (cur.compare(Y) == 0)
                    ans++;
            }
            return ans;
        }

        static class Pair {
            int val, count;

            Pair(int v, int c) {
                val = v;
                count = c;
            }

            int compare(Pair that) {
                return this.val != that.val ? this.val - that.val : this.count - that.count;
            }
        }
    }
}
