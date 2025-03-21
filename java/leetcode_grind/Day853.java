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

    static class Solution2 {
        public List<String> findAllRecipes(String[] recipes, List<List<String>> ingredients, String[] supplies) {
            Set<String> available = new HashSet<>();
            for (String supply : supplies) {
                available.add(supply);
            }

            Queue<Integer> recipeQueue = new LinkedList<>();
            for (int idx = 0; idx < recipes.length; ++idx) {
                recipeQueue.offer(idx);
            }

            List<String> createdRecipes = new ArrayList<>();
            int lastSize = -1;

            while (available.size() > lastSize) {
                lastSize = available.size();
                int queueSize = recipeQueue.size();

                while (queueSize-- > 0) {
                    int recipeIdx = recipeQueue.poll();
                    boolean canCreate = true;

                    for (String ingredient : ingredients.get(recipeIdx)) {
                        if (!available.contains(ingredient)) {
                            canCreate = false;
                            break;
                        }
                    }

                    if (!canCreate) {
                        recipeQueue.offer(recipeIdx);
                    } else {
                        available.add(recipes[recipeIdx]);
                        createdRecipes.add(recipes[recipeIdx]);
                    }
                }
            }

            return createdRecipes;
        }
    }

    static class Solution3 {
        public List<String> findAllRecipes(String[] recipes, List<List<String>> ingredients, String[] supplies) {
            List<String> possibleRecipes = new ArrayList<>();
            Map<String, Boolean> canMake = new HashMap<>();
            Map<String, Integer> recipeToIndex = new HashMap<>();

            for (String supply : supplies) {
                canMake.put(supply, true);
            }

            for (int idx = 0; idx < recipes.length; idx++) {
                recipeToIndex.put(recipes[idx], idx);
            }

            for (String recipe : recipes) {
                checkRecipe(
                        recipe,
                        ingredients,
                        new HashSet<String>(),
                        canMake,
                        recipeToIndex);
                if (canMake.get(recipe)) {
                    possibleRecipes.add(recipe);
                }
            }
            return possibleRecipes;
        }

        void checkRecipe(
                String recipe,
                List<List<String>> ingredients,
                Set<String> visited,
                Map<String, Boolean> canMake,
                Map<String, Integer> recipeToIndex) {
            if (canMake.containsKey(recipe) && canMake.get(recipe)) {
                return;
            }

            if (!recipeToIndex.containsKey(recipe) || visited.contains(recipe)) {
                canMake.put(recipe, false);
                return;
            }

            visited.add(recipe);

            List<String> neededIngredients = ingredients.get(recipeToIndex.get(recipe));

            for (String ingredient : neededIngredients) {
                checkRecipe(ingredient, ingredients, visited, canMake, recipeToIndex);
                if (!canMake.get(ingredient)) {
                    canMake.put(recipe, false);
                    return;
                }
            }

            canMake.put(recipe, true);
        }
    }

}
