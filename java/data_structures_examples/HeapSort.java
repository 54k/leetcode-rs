package data_structures_examples;

public class HeapSort {
    // https://leetcode.com/problems/height-checker/description/
    static class Solution1 {
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
}
