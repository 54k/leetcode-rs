package leetcode_grind;

public class Day389 {

    // https://leetcode.com/problems/create-sorted-array-through-instructions/description/
    static class Solution {
        int[] smaller;
        int[] larger;
        int[][] temp;

        public int createSortedArray(int[] instructions) {
            int n = instructions.length;

            smaller = new int[n];
            larger = new int[n];
            temp = new int[n][];

            long cost = 0;
            long MOD = 1_000_000_007;

            int[][] arrSmaller = new int[n][];
            int[][] arrLarger = new int[n][];

            for (int i = 0; i < n; i++) {
                arrSmaller[i] = new int[] { instructions[i], i };
                arrLarger[i] = new int[] { instructions[i], i };
            }

            sortSmaller(arrSmaller, 0, n - 1);
            sortLarger(arrLarger, 0, n - 1);

            for (int i = 0; i < n; i++) {
                cost += Math.min(smaller[i], larger[i]);
            }
            return (int) (cost % MOD);
        }

        void sortSmaller(int[][] arr, int left, int right) {
            if (left == right) {
                return;
            }

            int mid = (left + right) / 2;
            sortSmaller(arr, left, mid);
            sortSmaller(arr, mid + 1, right);
            mergeSmaller(arr, left, right, mid);
        }

        void mergeSmaller(int[][] arr, int left, int right, int mid) {
            int i = left;
            int j = mid + 1;
            int k = left;

            while (i <= mid && j <= right) {
                if (arr[i][0] < arr[j][0]) {
                    temp[k++] = arr[i];
                    i++;
                } else {
                    temp[k++] = arr[j];
                    smaller[arr[j][1]] += i - left;
                    j++;
                }
            }

            while (i <= mid) {
                temp[k++] = arr[i];
                i++;
            }

            while (j <= right) {
                temp[k++] = arr[j];
                smaller[arr[j][1]] += i - left;
                j++;
            }

            for (i = left; i <= right; i++) {
                arr[i] = temp[i];
            }
        }

        void sortLarger(int[][] arr, int left, int right) {
            if (left == right) {
                return;
            }
            int mid = (left + right) / 2;
            sortLarger(arr, left, mid);
            sortLarger(arr, mid + 1, right);
            mergeLarger(arr, left, right, mid);
        }

        void mergeLarger(int[][] arr, int left, int right, int mid) {
            int i = left;
            int j = mid + 1;
            int k = left;

            while (i <= mid && j <= right) {
                if (arr[i][0] <= arr[j][0]) {
                    temp[k++] = arr[i++];
                } else {
                    temp[k++] = arr[j];
                    larger[arr[j][1]] += mid - i + 1;
                    j++;
                }
            }
            while (i <= mid) {
                temp[k++] = arr[i++];
            }
            while (j <= right) {
                temp[k++] = arr[j];
                larger[arr[j][1]] += mid - i + 1;
                j++;
            }
            for (i = left; i <= right; i++) {
                arr[i] = temp[i];
            }
        }
    }
}
