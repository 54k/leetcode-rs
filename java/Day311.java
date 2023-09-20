import java.util.Arrays;
import java.util.HashSet;

public class Day311 {
    static class Solution {
        public int findDuplicateSort(int[] nums) {
            Arrays.sort(nums, 0, nums.length);
            for (var i = 1; i < nums.length; i++) {
                if (nums[i] == nums[i - 1]) {
                    return nums[i];
                }
            }
            return -1;
        }

        public int findDuplicateSet(int[] nums) {
            var set = new HashSet<Integer>();
            for (var n : nums) {
                if (set.contains(n)) {
                    return n;
                }
                set.add(n);
            }
            return -1;
        }

        public int findDuplicatNegativeMarking(int[] nums) {
            var duplicate = -1;
            for (var i = 0; i < nums.length; i++) {
                var cur = Math.abs(nums[i]);
                if (nums[cur] < 0) {
                    duplicate = cur;
                    break;
                }
                nums[cur] *= -1;
            }
            return duplicate;
        }

        public int findDuplicateArrayAsHashMap(int[] nums) {
            var store = new Object() {
                int store(int idx) {
                    if (nums[idx] != idx) {
                        var next = nums[idx];
                        nums[idx] = idx;
                        return store(next);
                    }
                    return nums[idx];
                }
            };
            return store.store(0);
        }

        public int findDuplicateArrayAsHashMapIterative(int[] nums) {
            while (nums[0] != nums[nums[0]]) {
                var tmp = nums[nums[0]];
                nums[nums[0]] = nums[0];
                nums[0] = tmp;
            }
            return nums[0];
        }
    }
}
