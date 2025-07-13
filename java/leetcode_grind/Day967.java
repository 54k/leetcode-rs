package leetcode_grind;

import java.util.Arrays;

public class Day967 {
    // https://leetcode.com/problems/maximum-matching-of-players-with-trainers/description/?envType=daily-question&envId=2025-07-13
    static class Solution1 {
        public int matchPlayersAndTrainers(int[] players, int[] trainers) {
            Arrays.sort(players);
            Arrays.sort(trainers);

            int m = players.length;
            int n = trainers.length;

            int count = 0;
            int i = 0;
            int j = 0;

            while (i < m && j < n) {
                if (players[i] <= trainers[j]) {
                    count++;
                    i++;
                    j++;
                } else {
                    j++;
                }
            }

            return count;
        }
    }
}
