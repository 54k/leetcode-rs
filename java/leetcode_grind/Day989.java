package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day989 {
    // https://leetcode.com/problems/fruit-into-baskets/description/?envType=daily-question&envId=2025-08-04
    static class Solution1 {
        public int totalFruit(int[] fruits) {
            Map<Integer, Integer> basket = new HashMap<>();
            int left = 0, right;
            for (right = 0; right < fruits.length; ++right) {
                basket.put(fruits[right], basket.getOrDefault(fruits[right], 0) + 1);
                if (basket.size() > 2) {
                    basket.put(fruits[left], basket.get(fruits[left]) - 1);
                    if (basket.get(fruits[left]) == 0) {
                        basket.remove(fruits[left]);
                    }
                    left++;
                }
            }
            return right - left;
        }
    }

    static class Solution2 {
        public int totalFruit(int[] fruits) {
            Map<Integer, Integer> basket = new HashMap<>();
            int left = 0, maxPicked = 0;
            for (int right = 0; right < fruits.length; ++right) {
                basket.put(fruits[right], basket.getOrDefault(fruits[right], 0) + 1);
                while (basket.size() > 2) {
                    basket.put(fruits[left], basket.get(fruits[left]) - 1);
                    if (basket.get(fruits[left]) == 0) {
                        basket.remove(fruits[left]);
                    }
                    left++;
                }
                maxPicked = Math.max(maxPicked, right - left + 1);
            }

            return maxPicked;
        }
    }
}
