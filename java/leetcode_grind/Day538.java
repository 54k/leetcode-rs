package leetcode_grind;

import java.util.Arrays;

public class Day538 {
    static class Solution1 {
        public int numRescueBoats(int[] people, int limit) {
            int ans = 0;
            Arrays.sort(people);
            int left = 0, right = people.length - 1;
            while (left <= right) {
                int l = people[left];
                int r = people[right];

                ans += 1;
                if (l + r <= limit) {
                    left++;
                }
                right--;

            }
            return ans;
        }
    }
}
