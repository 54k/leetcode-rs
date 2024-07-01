package leetcode_grind;

import java.util.Arrays;
import java.util.Comparator;

public class Day593 {
    // https://leetcode.com/problems/three-consecutive-odds/description/?envType=daily-question&envId=2024-07-01
    static class Solution1 {
        public boolean threeConsecutiveOdds(int[] arr) {
            int n = arr.length;
            for (int i = 0; i < n - 2; i++) {
                if ((arr[i] * arr[i + 1] * arr[i + 2]) % 2 == 1) {
                    return true;
                }
            }
            return false;
        }
    }

    // https://leetcode.com/problems/the-earliest-moment-when-everyone-become-friends/description/?envType=weekly-question&envId=2024-07-01
    static class Solution2 {
        public int earliestAcq(int[][] logs, int n) {
            Arrays.sort(logs, new Comparator<int[]>() {
                @Override
                public int compare(int[] log1, int[] log2) {
                    Integer tsp1 = new Integer(log1[0]);
                    Integer tsp2 = new Integer(log2[0]);
                    return tsp1.compareTo(tsp2);
                }
            });

            int groupCount = n;
            UnionFind uf = new UnionFind(n);

            for (int[] log : logs) {
                int timestamp = log[0], friendA = log[1], friendB = log[2];
                if (uf.union(friendA, friendB)) {
                    groupCount -= 1;
                }
                if (groupCount == 1) {
                    return timestamp;
                }
            }

            return -1;
        }

        static class UnionFind {
            int[] group;
            int[] rank;

            UnionFind(int size) {
                this.group = new int[size];
                this.rank = new int[size];
                for (int person = 0; person < size; ++person) {
                    this.group[person] = person;
                    this.rank[person] = 0;
                }
            }

            int find(int person) {
                if (this.group[person] != person) {
                    this.group[person] = this.find(this.group[person]);
                }
                return this.group[person];
            }

            boolean union(int a, int b) {
                int groupA = this.find(a);
                int groupB = this.find(b);
                if (groupA == groupB) {
                    return false;
                }
                if (this.rank[groupA] > this.rank[groupB]) {
                    this.group[groupB] = groupA;
                } else if (this.rank[groupA] < this.rank[groupB]) {
                    this.group[groupA] = groupB;
                } else {
                    this.group[groupA] = groupB;
                    this.rank[groupB] += 1;
                }
                return true;
            }
        }
    }
}