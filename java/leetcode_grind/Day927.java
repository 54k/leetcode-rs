package leetcode_grind;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;

public class Day927 {
    // https://leetcode.com/problems/maximum-candies-you-can-get-from-boxes/description/?envType=daily-question&envId=2025-06-03
    static class Solution1 {

        public int maxCandies(
                int[] status,
                int[] candies,
                int[][] keys,
                int[][] containedBoxes,
                int[] initialBoxes) {
            int n = status.length;
            boolean[] canOpen = new boolean[n];
            boolean[] hasBox = new boolean[n];
            boolean[] used = new boolean[n];

            for (int i = 0; i < n; ++i) {
                canOpen[i] = (status[i] == 1);
            }

            Queue<Integer> q = new LinkedList<>();
            int ans = 0;
            for (int box : initialBoxes) {
                hasBox[box] = true;
                if (canOpen[box]) {
                    q.offer(box);
                    used[box] = true;
                    ans += candies[box];
                }
            }

            while (!q.isEmpty()) {
                int bigBox = q.poll();
                for (int key : keys[bigBox]) {
                    canOpen[key] = true;
                    if (!used[key] && hasBox[key]) {
                        q.offer(key);
                        used[key] = true;
                        ans += candies[key];
                    }
                }
                for (int box : containedBoxes[bigBox]) {
                    hasBox[box] = true;
                    if (!used[box] && canOpen[box]) {
                        q.offer(box);
                        used[box] = true;
                        ans += candies[box];
                    }
                }
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/x-of-a-kind-in-a-deck-of-cards/description/
    static class Solution2 {
        public boolean hasGroupsSizeX(int[] deck) {
            int N = deck.length;
            int[] count = new int[10000];
            for (int c : deck) {
                count[c]++;
            }
            List<Integer> values = new ArrayList<>();
            for (int i = 0; i < 10000; i++) {
                if (count[i] > 0) {
                    values.add(count[i]);
                }
            }
            search: for (int X = 2; X <= N; ++X) {
                if (N % X == 0) {
                    for (int v : values) {
                        if (v % X != 0) {
                            continue search;
                        }
                    }
                    return true;
                }
            }

            return false;
        }
    }

    static class Solution3 {
        public boolean hasGroupsSizeX(int[] deck) {
            int[] count = new int[10000];
            for (int c : deck) {
                count[c]++;
            }
            int g = -1;
            for (int i = 0; i < 10000; i++) {
                if (count[i] > 0) {
                    if (g == -1) {
                        g = count[i];
                    } else {
                        g = gcd(g, count[i]);
                    }
                }
            }
            return g >= 2;
        }

        int gcd(int x, int y) {
            return x == 0 ? y : gcd(y % x, x);
        }
    }
}
