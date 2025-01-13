package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Random;
import java.util.Set;
import java.util.TreeSet;

public class Day786 {
    // https://leetcode.com/problems/minimum-length-of-string-after-operations/description/?envType=daily-question&envId=2025-01-13
    static class Solution1 {
        public int minimumLength(String s) {
            Map<Character, Integer> charFrequencyMap = new HashMap<>();
            for (char currentChar : s.toCharArray()) {
                charFrequencyMap.put(
                        currentChar,
                        charFrequencyMap.getOrDefault(currentChar, 0) + 1);
            }

            int deleteCount = 0;
            for (int frequency : charFrequencyMap.values()) {
                if (frequency % 2 == 1) {
                    deleteCount += frequency - 1;
                } else {
                    deleteCount += frequency - 2;
                }
            }

            return s.length() - deleteCount;
        }
    }

    // https://leetcode.com/problems/design-movie-rental-system/description/
    static class MovieRentingSystem {
        final Comparator<int[]> cmp = (a, b) -> a[0] != b[0] ? a[0] - b[0] : a[1] != b[1] ? a[1] - b[1] : a[2] - b[2]; // price
                                                                                                                       // shop
                                                                                                                       // movie

        final Map<Integer, Set<int[]>> available = new HashMap<>();
        final Set<int[]> rented = new TreeSet<>(cmp);
        final Map<Integer, Map<Integer, Integer>> prices = new HashMap<>();

        public MovieRentingSystem(int n, int[][] entries) {
            for (int[] i : entries) {
                available.computeIfAbsent(i[1], t -> new TreeSet<>(cmp)).add(new int[] { i[2], i[0], i[1] });
                prices.computeIfAbsent(i[0], t -> new HashMap<>()).put(i[1], i[2]);
            }
        }

        public List<Integer> search(int movie) {
            return available.getOrDefault(movie, Collections.emptySet())
                    .stream().limit(5).map(i -> i[1]).toList();
        }

        public void rent(int shop, int movie) {
            int[] item = { prices.get(shop).get(movie), shop, movie };
            available.get(movie).remove(item);
            rented.add(item);
        }

        public void drop(int shop, int movie) {
            int[] item = { prices.get(shop).get(movie), shop, movie };
            available.get(movie).add(item);
            rented.remove(item);
        }

        public List<List<Integer>> report() {
            return rented.stream().limit(5).map(i -> List.of(i[1], i[2])).toList();
        }
    }

    // https://leetcode.com/problems/random-point-in-non-overlapping-rectangles/description/
    /**
     * Your MovieRentingSystem object will be instantiated and called as such:
     * MovieRentingSystem obj = new MovieRentingSystem(n, entries);
     * List<Integer> param_1 = obj.search(movie);
     * obj.rent(shop,movie);
     * obj.drop(shop,movie);
     * List<List<Integer>> param_4 = obj.report();
     */

    static class Solution2 {
        int[][] rects;
        List<Integer> psum = new ArrayList<>();
        int tot = 0;
        Random rand = new Random();

        public Solution2(int[][] rects) {
            this.rects = rects;
            for (int[] x : rects) {
                tot += (x[2] - x[0] + 1) * (x[3] - x[1] + 1);
                psum.add(tot);
            }
        }

        public int[] pick() {
            int targ = rand.nextInt(tot);

            int lo = 0;
            int hi = rects.length - 1;

            while (lo != hi) {
                int mid = (lo + hi) / 2;
                if (targ >= psum.get(mid))
                    lo = mid + 1;
                else
                    hi = mid;
            }

            int[] x = rects[lo];
            int width = x[2] - x[0] + 1;
            int height = x[3] - x[1] + 1;
            int base = psum.get(lo) - width * height;
            return new int[] { x[0] + (targ - base) % width, x[1] + (targ - base) / width };
        }
    }

    /**
     * Your Solution object will be instantiated and called as such:
     * Solution obj = new Solution(rects);
     * int[] param_1 = obj.pick();
     */
}
