package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day573 {

    // https://leetcode.com/problems/height-checker/description/
    static class Solution1 {
        void merge(int[] arr, int left, int mid, int right, int[] tempArr) {
            int start1 = left;
            int start2 = mid + 1;
            int n1 = mid - left + 1;
            int n2 = right - mid;

            for (int i = 0; i < n1; i++) {
                tempArr[start1 + i] = arr[start1 + i];
            }
            for (int i = 0; i < n2; i++) {
                tempArr[start2 + i] = arr[start2 + i];
            }
            int i = 0, j = 0, k = left;
            while (i < n1 && j < n2) {
                if (tempArr[start1 + i] <= tempArr[start2 + j]) {
                    arr[k] = tempArr[start1 + i];
                    i += 1;
                } else {
                    arr[k] = tempArr[start2 + j];
                    j += 1;
                }
                k += 1;
            }
            while (i < n1) {
                arr[k] = tempArr[start1 + i];
                i += 1;
                k += 1;
            }
            while (j < n2) {
                arr[k] = tempArr[start2 + j];
                j += 1;
                k += 1;
            }
        }

        void mergeSort(int[] arr, int left, int right, int[] tempArr) {
            if (left >= right) {
                return;
            }
            int mid = (left + right) / 2;
            mergeSort(arr, left, mid, tempArr);
            mergeSort(arr, mid + 1, right, tempArr);
            merge(arr, left, mid, right, tempArr);
        }

        public int heightChecker(int[] heights) {
            int[] sortedHeights = heights.clone();
            int[] tempArr = new int[heights.length];
            mergeSort(sortedHeights, 0, sortedHeights.length - 1, tempArr);

            int count = 0;
            for (int i = 0; i < sortedHeights.length; i++) {
                if (heights[i] != sortedHeights[i]) {
                    count += 1;
                }
            }
            return count;
        }
    }

    static class Solution2 {
        void heapify(int[] arr, int n, int i) {
            int largest = i;
            int left = 2 * i + 1;
            int right = 2 * i + 2;

            if (left < n && arr[left] > arr[largest]) {
                largest = left;
            }

            if (right < n && arr[right] > arr[largest]) {
                largest = right;
            }

            if (largest != i) {
                int swap = arr[i];
                arr[i] = arr[largest];
                arr[largest] = swap;
                heapify(arr, n, largest);
            }
        }

        void heapSort(int[] arr) {
            int n = arr.length;
            for (int i = n / 2 - 1; i >= 0; i--) {
                heapify(arr, n, i);
            }

            for (int i = n - 1; i >= 0; i--) {
                int swap = arr[0];
                arr[0] = arr[i];
                arr[i] = swap;
                heapify(arr, i, 0);
            }
        }

        public int heightChecker(int[] heights) {
            int[] sortedHeights = heights.clone();
            heapSort(sortedHeights);

            int count = 0;
            for (int i = 0; i < sortedHeights.length; i++) {
                if (heights[i] != sortedHeights[i]) {
                    count += 1;
                }
            }
            return count;
        }
    }

    static class Solution3 {
        void countingSort(int[] arr) {
            Map<Integer, Integer> counts = new HashMap<>();
            int minVal = Arrays.stream(arr).min().getAsInt();
            int maxVal = Arrays.stream(arr).max().getAsInt();

            for (int val : arr) {
                counts.put(val, counts.getOrDefault(val, 0) + 1);
            }

            int index = 0;

            for (int val = minVal; val <= maxVal; ++val) {
                while (counts.getOrDefault(val, 0) > 0) {
                    arr[index] = val;
                    index += 1;
                    counts.put(val, counts.get(val) - 1);
                }
            }
        }

        public int heightChecker(int[] heights) {
            int[] sortedHeights = heights.clone();
            countingSort(sortedHeights);

            int count = 0;
            for (int i = 0; i < sortedHeights.length; i++) {
                if (heights[i] != sortedHeights[i]) {
                    count += 1;
                }
            }
            return count;
        }
    }

    // https://leetcode.com/problems/height-checker/description
    static class Solution4 {
        void bucketSort(int[] arr, int placeValue) {
            List<List<Integer>> buckets = new ArrayList<>(10);
            for (int i = 0; i < 10; i++) {
                buckets.add(new ArrayList<>());
            }
            for (int val : arr) {
                int digit = Math.abs(val) / placeValue;
                digit = digit % 10;
                buckets.get(digit).add(val);
            }

            int index = 0;
            for (int digit = 0; digit < 10; ++digit) {
                for (int val : buckets.get(digit)) {
                    arr[index] = val;
                    index++;
                }
            }
        }

        void radixSort(int[] arr) {
            int maxElement = Arrays.stream(arr).map(Math::abs).max().getAsInt();
            int maxDigits = 0;
            while (maxElement > 0) {
                maxDigits += 1;
                maxElement /= 10;
            }

            int placeValue = 1;
            for (int digit = 0; digit < maxDigits; ++digit) {
                bucketSort(arr, placeValue);
                placeValue *= 10;
            }
        }

        public int heightChecker(int[] heights) {
            int[] sortedHeights = heights.clone();
            radixSort(sortedHeights);

            int count = 0;
            for (int i = 0; i < sortedHeights.length; i++) {
                if (heights[i] != sortedHeights[i]) {
                    count += 1;
                }
            }
            return count;
        }
    }
}
