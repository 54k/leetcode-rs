package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;
import java.util.Set;

public class Day853 {
    // https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies/description/?envType=daily-question&envId=2025-03-21
    static class Solution1 {
        public List<String> findAllRecipes(String[] recipes, List<List<String>> ingredients, String[] supplies) {
            Set<String> availableSupplies = new HashSet<>();
            Map<String, Integer> recipeToIndex = new HashMap<>();
            Map<String, List<String>> dependencyGraph = new HashMap<>();

            for (String supply : supplies) {
                availableSupplies.add(supply);
            }

            for (int idx = 0; idx < recipes.length; idx++) {
                recipeToIndex.put(recipes[idx], idx);
            }

            int[] inDegree = new int[recipes.length];

            for (int recipeIdx = 0; recipeIdx < recipes.length; recipeIdx++) {
                for (String ingredient : ingredients.get(recipeIdx)) {
                    if (!availableSupplies.contains(ingredient)) {
                        dependencyGraph.putIfAbsent(ingredient, new ArrayList<>());
                        dependencyGraph.get(ingredient).add(recipes[recipeIdx]);
                        inDegree[recipeIdx]++;
                    }
                }
            }

            Queue<Integer> queue = new LinkedList<>();
            for (int recipeIdx = 0; recipeIdx < recipes.length; recipeIdx++) {
                if (inDegree[recipeIdx] == 0) {
                    queue.add(recipeIdx);
                }
            }

            List<String> createdRecipes = new ArrayList<>();
            while (!queue.isEmpty()) {
                int recipeIdx = queue.poll();
                String recipe = recipes[recipeIdx];
                createdRecipes.add(recipe);

                if (!dependencyGraph.containsKey(recipe))
                    continue;

                for (String dependentRecipe : dependencyGraph.get(recipe)) {
                    if (--inDegree[recipeToIndex.get(dependentRecipe)] == 0) {
                        queue.add(recipeToIndex.get(dependentRecipe));
                    }
                }

            }

            return createdRecipes;
        }
    }

}
