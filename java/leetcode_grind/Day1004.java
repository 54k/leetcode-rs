package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day1004 {
    // https://leetcode.com/problems/number-of-zero-filled-subarrays/description/?envType=daily-question&envId=2025-08-19
    static class Solution1 {
        public long zeroFilledSubarray(int[] nums) {
            long ans = 0, numSubarray = 0;

            for (int num : nums) {
                if (num == 0) {
                    numSubarray++;
                } else {
                    numSubarray = 0;
                }
                ans += numSubarray;
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/description/
    static class Solution2 {
        public int removeStones(int[][] stones) {
            int n = stones.length;
            List<Integer>[] adjacencyList = new List[n];
            for (int i = 0; i < n; i++) {
                adjacencyList[i] = new ArrayList<>();
            }

            for (int i = 0; i < n; i++) {
                for (int j = i + 1; j < n; j++) {
                    if (stones[i][0] == stones[j][0] || stones[i][1] == stones[j][1]) {
                        adjacencyList[i].add(j);
                        adjacencyList[j].add(i);
                    }
                }
            }

            int numOfConnectedComponents = 0;
            boolean[] visited = new boolean[n];

            for (int i = 0; i < n; i++) {
                if (!visited[i]) {
                    depthFirstSearch(adjacencyList, visited, i);
                    numOfConnectedComponents++;
                }
            }

            return n - numOfConnectedComponents;
        }

        void depthFirstSearch(List<Integer>[] adjacenyList, boolean[] visited, int stone) {
            visited[stone] = true;
            for (int neighbor : adjacenyList[stone]) {
                if (!visited[neighbor]) {
                    depthFirstSearch(adjacenyList, visited, neighbor);
                }
            }
        }
    }
}
