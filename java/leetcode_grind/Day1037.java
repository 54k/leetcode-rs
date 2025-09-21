package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeSet;

public class Day1037 {

    // https://leetcode.com/problems/design-movie-rental-system/description/?envType=daily-question&envId=2025-09-21
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

    /**
     * Your MovieRentingSystem object will be instantiated and called as such:
     * MovieRentingSystem obj = new MovieRentingSystem(n, entries);
     * List<Integer> param_1 = obj.search(movie);
     * obj.rent(shop,movie);
     * obj.drop(shop,movie);
     * List<List<Integer>> param_4 = obj.report();
     */

    // https://leetcode.com/problems/count-of-smaller-numbers-after-self/description/
    static class Solution1 {
        public List<Integer> countSmaller(int[] nums) {
            int offset = 10000;
            int size = 2 * 10000 + 1;
            int[] tree = new int[size * 2];
            List<Integer> result = new ArrayList<>();

            for (int i = nums.length - 1; i >= 0; i--) {
                int smallerCount = query(0, nums[i] + offset, tree, size);
                result.add(smallerCount);
                update(nums[i] + offset, 1, tree, size);
            }
            Collections.reverse(result);
            return result;
        }

        void update(int index, int value, int[] tree, int size) {
            index += size;
            tree[index] += value;
            while (index > 1) {
                index /= 2;
                tree[index] = tree[index * 2] + tree[index * 2 + 1];
            }
        }

        int query(int left, int right, int[] tree, int size) {
            int result = 0;
            left += size;
            right += size;

            while (left < right) {
                if (left % 2 == 1) {
                    result += tree[left];
                    left++;
                }
                left /= 2;

                if (right % 2 == 1) {
                    right--;
                    result += tree[right];
                }
                right /= 2;
            }
            return result;
        }
    }
}
