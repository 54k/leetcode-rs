package leetcode_grind;

public class Day701 {
    // https://leetcode.com/problems/maximum-swap/description/?envType=daily-question&envId=2024-10-17
    static class Solution1 {
        public int maximumSwap(int num) {
            var ans = num;
            var arr = Integer.toString(num).toCharArray();
            for (int i = 0; i < arr.length - 1; i++) {
                for (int j = i + 1; j < arr.length; j++) {
                    if (arr[i] < arr[j]) {
                        swap(arr, i, j);
                        var next = Integer.valueOf(new String(arr));
                        if (next > ans) {
                            ans = next;
                        }
                        swap(arr, i, j);
                    }
                }
            }
            return ans;
        }

        void swap(char[] arr, int i, int j) {
            var t = arr[i];
            arr[i] = arr[j];
            arr[j] = t;
        }
    }

    static class Solution2 {
        public int maximumSwap(int num) {
            char[] numArr = Integer.toString(num).toCharArray();
            int n = numArr.length;
            int[] maxRightIndex = new int[n];

            maxRightIndex[n - 1] = n - 1;
            for (int i = n - 2; i >= 0; i--) {
                maxRightIndex[i] = (numArr[i] > numArr[maxRightIndex[i + 1]]) ? i : maxRightIndex[i + 1];
            }

            for (int i = 0; i < n; ++i) {
                if (numArr[i] < numArr[maxRightIndex[i]]) {
                    char temp = numArr[i];
                    numArr[i] = numArr[maxRightIndex[i]];
                    numArr[maxRightIndex[i]] = temp;
                    return Integer.parseInt(new String(numArr));
                }
            }

            return num;
        }
    }

}
