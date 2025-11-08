package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;

public class Day1085 {

    // https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero/description/?envType=daily-question&envId=2025-11-08
    static class Solution1 {
        public int minimumOneBitOperations(int n) {
            int ans = n;
            ans ^= ans >> 16;
            ans ^= ans >> 8;
            ans ^= ans >> 4;
            ans ^= ans >> 2;
            ans ^= ans >> 1;
            return ans;
        }
    }

    // https://leetcode.com/problems/two-sum-iii-data-structure-design/description/?envType=weekly-question&envId=2025-11-08
    static class TwoSum {
        ArrayList<Integer> nums;
        boolean is_sorted;

        public TwoSum() {
            this.nums = new ArrayList<Integer>();
            this.is_sorted = true;
        }

        public void add(int number) {
            this.nums.add(number);
            this.is_sorted = false;
        }

        public boolean find(int value) {
            if (!this.is_sorted) {
                Collections.sort(this.nums);
                this.is_sorted = true;
            }
            int low = 0, high = this.nums.size() - 1;
            while (low < high) {
                int twosum = this.nums.get(low) + this.nums.get(high);
                if (twosum < value)
                    low += 1;
                else if (twosum > value)
                    high -= 1;
                else
                    return true;
            }
            return false;
        }
    }

    /**
     * Your TwoSum object will be instantiated and called as such:
     * TwoSum obj = new TwoSum();
     * obj.add(number);
     * boolean param_2 = obj.find(value);
     */

}
