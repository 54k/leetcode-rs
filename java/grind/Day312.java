package grind;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.function.Function;
import java.util.function.Supplier;

public class Day312 {
    // https://leetcode.com/problems/find-the-duplicate-number/description/
    static class Solution1 {
        public int findDuplicateBinarySearch(int[] nums) {
            var left = 1;
            var right = nums.length;
            var duplicate = -1;
            while (left <= right) {
                var mid = (left + right) / 2;

                var count = 0;
                for (var num : nums) {
                    if (num <= mid) {
                        count++;
                    }
                }

                if (count > mid) {
                    duplicate = mid;
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            return duplicate;
        }

        public int findDuplicateCountOfBitSet(int[] nums) {
            Supplier<Integer> maxNum = () -> {
                var ans = 0;
                for (var num : nums) {
                    ans = Math.max(ans, num);
                }
                return ans;
            };
            Function<Integer, Integer> maxBit = (num) -> {
                var ans = 0;
                while (num > 0) {
                    ans++;
                    num >>= 1;
                }
                return ans;
            };

            var duplicate = 0;
            var max = maxBit.apply(maxNum.get());
            for (var bit = 0; bit < max; bit++) {
                var mask = 1 << bit;
                var baseCount = 0;
                var bitCount = 0;

                for (var i = 0; i < nums.length; i++) {
                    if ((mask & i) > 0) {
                        baseCount++;
                    }
                    if ((mask & nums[i]) > 0) {
                        bitCount++;
                    }
                }

                if (bitCount > baseCount) {
                    duplicate |= mask;
                }
            }
            return duplicate;
        }

        public int findDuplicateFloyd(int[] nums) {
            var slow = nums[0];
            var fast = nums[0];

            for (;;) {
                slow = nums[slow];
                fast = nums[nums[fast]];

                if (slow == fast) {
                    slow = nums[0];
                    while (slow != fast) {
                        slow = nums[slow];
                        fast = nums[fast];
                    }
                    return slow;
                }
            }
        }
    }

    // https://leetcode.com/problems/minimum-operations-to-reduce-x-to-zero/description/
    static class Solution2 {
        public int minOperationsIndirect(int[] nums, int x) {
            var sum = 0;
            for (var num : nums) {
                sum += num;
            }
            var current = 0;
            var ans = -1;
            var left = 0;
            for (var right = 0; right < nums.length; right++) {
                current += nums[right];

                while (current > sum - x && left <= right) {
                    current -= nums[left++];
                }

                if (sum - current == x) {
                    ans = Math.max(ans, right - left + 1);
                }
            }

            if (ans == -1) {
                return -1;
            }
            return nums.length - ans;
        }

        public int minOperationsDirect(int[] nums, int x) {
            var current = 0;
            for (var num : nums) {
                current += num;
            }

            var ans = Integer.MAX_VALUE;
            var left = 0;

            for (var right = 0; right < nums.length; right++) {
                current -= nums[right];

                while (current < x && left <= right) {
                    current += nums[left++];
                }

                if (current == x) {
                    ans = Math.min(ans, (nums.length - 1 - right) + left);
                }
            }

            if (ans == Integer.MAX_VALUE) {
                return -1;
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/maximum-size-subarray-sum-equals-k/
    static class Solution {
        public int maxSubArrayLen(int[] nums, int k) {
            var indices = new HashMap<Integer, Integer>();
            var current = 0;
            var ans = 0;

            for (var right = 0; right < nums.length; right++) {
                current += nums[right];
                if (!indices.containsKey(current)) {
                    indices.put(current, right);
                }

                if (current == k) {
                    ans = right + 1;
                }

                if (indices.containsKey(current - k)) {
                    ans = Math.max(ans, right - indices.get(current - k));
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/sparse-matrix-multiplication/
    static class Solution3 {
        public int[][] multiplyNaive(int[][] mat1, int[][] mat2) {
            var m = mat1.length;
            var n = mat2[0].length;
            var k = mat1[0].length;

            var result = new int[m][n];
            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    for (int z = 0; z < k; z++) {
                        result[i][j] += mat1[i][z] * mat2[z][j];
                    }
                }

            }
            return result;
        }

        public int[][] multiplyOptimized(int[][] mat1, int[][] mat2) {
            var m = mat1.length;
            var n = mat2[0].length;
            var k = mat1[0].length;

            var result = new int[m][n];
            for (var i = 0; i < m; i++) {
                for (var elementPos = 0; elementPos < k; elementPos++) {
                    for (var j = 0; j < n; j++) {
                        if (mat1[i][elementPos] != 0) {
                            result[i][j] += mat1[i][elementPos] * mat2[elementPos][j];
                        }
                    }
                }
            }

            return result;
        }

        public int[][] multiplyOptimized2(int[][] mat1, int[][] mat2) {
            Function<int[][], List<List<int[]>>> compressMatrix = (int[][] mat) -> {
                var rows = mat.length;
                var cols = mat[0].length;
                var comressedMatrix = new ArrayList<List<int[]>>();

                for (var row = 0; row < rows; row++) {
                    var currRow = new ArrayList<int[]>();
                    for (int col = 0; col < cols; col++) {
                        if (mat[row][col] != 0) {
                            currRow.add(new int[] { mat[row][col], col });
                        }
                    }
                    comressedMatrix.add(currRow);
                }

                return comressedMatrix;
            };

            var m = mat1.length;
            var n = mat2[0].length;

            var result = new int[m][n];
            var A = compressMatrix.apply(mat1);
            var B = compressMatrix.apply(mat2);

            for (var mat1Row = 0; mat1Row < m; mat1Row++) {
                for (var mat1Element : A.get(mat1Row)) {
                    var element1 = mat1Element[0];
                    var mat1Col = mat1Element[1];

                    for (var mat2Element : B.get(mat1Col)) {
                        var element2 = mat2Element[0];
                        var mat2Col = mat2Element[1];

                        result[mat1Row][mat2Col] += element1 * element2;
                    }
                }
            }

            return result;
        }
    }
}
