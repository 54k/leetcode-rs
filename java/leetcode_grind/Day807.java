package leetcode_grind;

public class Day807 {
    // https://leetcode.com/problems/shuffle-an-array/description/
    static class Solution1 {
        int[] arr;
        int[] orig;
        Random rand = new Random();

        public Solution1(int[] nums) {
            orig = nums;
            arr = orig.clone();
        }

        public int[] reset() {
            arr = orig.clone();
            return arr;
        }

        public int[] shuffle() {
            for (int i = 0; i < arr.length; i++) {
                int idx = rand.nextInt(i, arr.length);
                int temp = arr[i];
                arr[i] = arr[idx];
                arr[idx] = temp;
            }
            return arr;
        }
    }

}
