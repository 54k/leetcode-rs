package leetcode_grind;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day859 {
    // https://leetcode.com/problems/minimum-index-of-a-valid-split/description/?envType=daily-question&envId=2025-03-27
    static class Solution1 {
        public int minimumIndex(List<Integer> nums) {
            Map<Integer, Integer> firstMap = new HashMap<>();
            Map<Integer, Integer> secondMap = new HashMap<>();
            int n = nums.size();

            for (int num : nums) {
                secondMap.put(num, secondMap.getOrDefault(num, 0) + 1);
            }

            for (int index = 0; index < n; index++) {
                int num = nums.get(index);
                secondMap.put(num, secondMap.get(num) - 1);
                firstMap.put(num, firstMap.getOrDefault(num, 0) + 1);

                if (firstMap.get(num) * 2 > index + 1 && secondMap.get(num) * 2 > n - index - 1) {
                    return index;
                }
            }

            return -1;
        }
    }

    static class Solution2 {
        public int minimumIndex(List<Integer> nums) {
            int x = nums.get(0), count = 0, xCount = 0, n = nums.size();

            for (int num : nums) {
                if (num == x) {
                    count++;
                } else {
                    count--;
                }
                if (count == 0) {
                    x = num;
                    count = 1;
                }
            }

            for (int num : nums) {
                if (num == x) {
                    xCount++;
                }
            }

            count = 0;
            for (int index = 0; index < n; index++) {
                if (nums.get(index) == x) {
                    count++;
                }
                int remainingCount = xCount - count;
                if (count * 2 > index + 1 && remainingCount * 2 > n - index - 1) {
                    return index;
                }
            }

            return -1;
        }
    }

    

}
