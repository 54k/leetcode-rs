package leetcode_grind;

import java.util.Arrays;
import java.util.List;

public class Day1099 {
    // https://leetcode.com/problems/find-minimum-operations-to-make-all-elements-divisible-by-three/description/?envType=daily-question&envId=2025-11-22
    static class Solution1 {
        public int minimumOperations(int[] nums) {
            return Arrays.stream(nums).map(n -> Math.min(n % 3, 1)).sum();
        }
    }

    // https://leetcode.com/problems/nested-list-weight-sum-ii/description/?envType=weekly-question&envId=2025-11-22
    // static class Solution2 {
    //     public int depthSumInverse(List<NestedInteger> nestedList) {
    //         int maxDepth = findMaxDepth(nestedList);
    //         return weightedSum(nestedList, 1, maxDepth);
    //     }

    //     int findMaxDepth(List<NestedInteger> list) {
    //         int maxDepth = 1;
    //         for (NestedInteger nested : list) {
    //             if (!nested.isInteger() && nested.getList().size() > 0) {
    //                 maxDepth = Math.max(maxDepth, 1 + findMaxDepth(nested.getList()));
    //             }
    //         }
    //         return maxDepth;
    //     }

    //     int weightedSum(List<NestedInteger> list, int depth, int maxDepth) {
    //         int answer = 0;
    //         for (NestedInteger nested : list) {
    //             if (nested.isInteger()) {
    //                 answer += nested.getInteger() * (maxDepth - depth + 1);
    //             } else {
    //                 answer += weightedSum(nested.getList(), depth + 1, maxDepth);
    //             }
    //         }
    //         return answer;
    //     }
    // }
}
