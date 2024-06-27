package leetcode_grind;

public class Day589 {
    // https://leetcode.com/problems/find-center-of-star-graph/description/?envType=daily-question&envId=2024-06-27
    static class Solution1 {
        public int findCenter(int[][] edges) {
            var e1 = edges[0];
            var e2 = edges[1];
            if (e1[0] == e2[0] || e1[0] == e2[1]) {
                return e1[0];
            }
            return e1[1];
        }
    }
}
