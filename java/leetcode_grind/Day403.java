package leetcode_grind;

public class Day403 {
    // https://leetcode.com/problems/rle-iterator/description/
    static class RLEIterator {
        int[] arr;
        int j = 0;

        public RLEIterator(int[] encoding) {
            arr = encoding;
        }

        public int next(int n) {
            while (j < arr.length && arr[j] < n) {
                n -= arr[j];
                j += 2;
            }

            if (j < arr.length) {
                int v = arr[j + 1];
                arr[j] -= n;
                return v;
            }

            return -1;
        }
    }
}
