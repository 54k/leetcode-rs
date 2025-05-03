package leetcode_grind;

public class Day896 {
    // https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/description/?envType=daily-question&envId=2025-05-03
    static class Solution1 {
        public int minDominoRotations(int[] tops, int[] bottoms) {
            int n = tops.length;        
            int rotations = check(tops[0], bottoms, tops, n);
            if (rotations != -1 || tops[0] == bottoms[0]) return rotations;
            else return check(bottoms[0], bottoms, tops, n);
        }

        int check(int x, int[] A, int[] B, int n) {
            int rotations_a = 0, rotations_b = 0;
            for (int i = 0; i < n; i++) {
                if (A[i] != x && B[i] != x) return -1;
                else if (A[i] != x) rotations_a++;
                else if (B[i] != x) rotations_b++;
            }
            return Math.min(rotations_a, rotations_b);
        }
    }
}