package leetcode_grind;

import java.util.HashMap;
import java.util.Map;
import java.util.PriorityQueue;

public class Day400 {
    // https://leetcode.com/problems/design-a-food-rating-system/description
    static class FoodRatings {
        static class Food implements Comparable<Food> {
            int foodRating;
            String foodName;

            Food(int foodRating, String foodName) {
                this.foodRating = foodRating;
                this.foodName = foodName;
            }

            public int compareTo(Food other) {
                if (foodRating == other.foodRating) {
                    return foodName.compareTo(other.foodName);
                }
                return -1 * Integer.compare(foodRating, other.foodRating);
            }
        }

        Map<String, Integer> foodRatingMap;
        Map<String, String> foodCuisineMap;
        Map<String, PriorityQueue<Food>> cuisineFoodMap;

        public FoodRatings(String[] foods, String[] cuisines, int[] ratings) {
            foodRatingMap = new HashMap<>();
            foodCuisineMap = new HashMap<>();
            cuisineFoodMap = new HashMap<>();

            for (int i = 0; i < foods.length; ++i) {
                foodRatingMap.put(foods[i], ratings[i]);
                foodCuisineMap.put(foods[i], cuisines[i]);
                cuisineFoodMap.computeIfAbsent(cuisines[i], k -> new PriorityQueue<>())
                        .add(new Food(ratings[i], foods[i]));
            }
        }

        public void changeRating(String food, int newRating) {
            foodRatingMap.put(food, newRating);
            String cuisineName = foodCuisineMap.get(food);
            cuisineFoodMap.get(cuisineName).add(new Food(newRating, food));
        }

        public String highestRated(String cuisine) {
            Food highestRated = cuisineFoodMap.get(cuisine).peek();

            while (foodRatingMap.get(highestRated.foodName) != highestRated.foodRating) {
                cuisineFoodMap.get(cuisine).poll();
                highestRated = cuisineFoodMap.get(cuisine).peek();
            }

            return highestRated.foodName;
        }
    }
}
