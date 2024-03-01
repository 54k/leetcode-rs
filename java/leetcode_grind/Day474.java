package leetcode_grind;

public class Day474 {
    // https://leetcode.com/problems/maximum-odd-binary-number/description
    static class Solution1 {
        public String maximumOddBinaryNumber(String s) {
            char[] arr = s.toCharArray();
            int i = arr.length - 1;
            while (i >= 0) {
                if (arr[i] == '1') {
                    char t = arr[arr.length - 1];
                    arr[arr.length - 1] = arr[i];
                    arr[i] = t;
                    break;
                }
                i--;
            }
            int k = 0;
            int j = 0;
            while (j < i) {
                if (arr[j] == '1') {
                    char t = arr[k];
                    arr[k++] = arr[j];
                    arr[j] = t;
                }
                j++;
            }
            return new String(arr);
        }
    }
}
