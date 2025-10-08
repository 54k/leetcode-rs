package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Day1054 {
    // https://leetcode.com/problems/destination-city/description/?envType=company&envId=yandex&favoriteSlug=yandex-all
    static class Solution1 {
        public String destCity(List<List<String>> paths) {
            Set<String> hasOutgoing = new HashSet<>();
            for (int i = 0; i < paths.size(); i++) {
                hasOutgoing.add(paths.get(i).get(0));
            }

            for (int i = 0; i < paths.size(); i++) {
                String candidate = paths.get(i).get(1);
                if (!hasOutgoing.contains(candidate)) {
                    return candidate;
                }
            }

            return "";
        }
    }

    // https://leetcode.com/problems/successful-pairs-of-spells-and-potions/description/?envType=daily-question&envId=2025-10-08
    static class Solution2 {
        int lowerBound(int[] arr, int key) {
            int lo = 0;
            int hi = arr.length;

            while (lo < hi) {
                int mid = lo + (hi - lo) / 2;
                if (arr[mid] < key) {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            return lo;
        }

        public int[] successfulPairs(int[] spells, int[] potions, long success) {
            Arrays.sort(potions);
            int[] answer = new int[spells.length];

            int m = potions.length;
            int maxPotion = potions[m - 1];

            for (int i = 0; i < spells.length; i++) {
                int spell = spells[i];
                long minPotion = (long) Math.ceil((1.0 * success) / spell);

                if (minPotion > maxPotion) {
                    answer[i] = 0;
                    continue;
                }

                int index = lowerBound(potions, (int) minPotion);
                answer[i] = m - index;
            }
            return answer;
        }
    }

    static class Solution3 {
        public int[] successfulPairs(int[] spells, int[] potions, long success) {
            int n = spells.length;
            int m = potions.length;

            int[][] sortedSpells = new int[n][2];
            for (int i = 0; i < n; i++) {
                sortedSpells[i][0] = spells[i];
                sortedSpells[i][1] = i;
            }

            Arrays.sort(sortedSpells, (a, b) -> Integer.compare(a[0], b[0]));
            Arrays.sort(potions);

            int[] answer = new int[n];
            int potionIndex = m - 1;

            for (int[] sortedSpell : sortedSpells) {
                int spell = sortedSpell[0];
                int index = sortedSpell[1];

                while (potionIndex >= 0 && (long) spell * potions[potionIndex] >= success) {
                    potionIndex -= 1;
                }
                answer[index] = m - (potionIndex + 1);
            }
            return answer;
        }
    }

    // https://leetcode.com/problems/minimize-max-distance-to-gas-station/description/
    static class Solution4 {
        public double minmaxGasDist(int[] stations, int k) {
            double lo = 0, hi = 1e8;
            while (hi - lo > 1e-6) {
                double mi = (lo + hi) / 2.0;
                if (possible(mi, stations, k)) {
                    hi = mi;
                } else {
                    lo = mi;
                }
            }
            return lo;
        }

        boolean possible(double D, int[] stations, int K) {
            int used = 0;
            for (int i = 0; i < stations.length - 1; i++) {
                used += (int) ((stations[i + 1] - stations[i]) / D);
            }
            return used <= K;
        }
    }

    // https://leetcode.com/problems/find-k-closest-elements/description/?envType=company&envId=yandex&favoriteSlug=yandex-all
    static class Solution5 {
        public List<Integer> findClosestElements(int[] arr, int k, int x) {
            List<Integer> sortedArr = new ArrayList<>();
            for (int num : arr) {
                sortedArr.add(num);
            }
            Collections.sort(sortedArr, (num1, num2) -> Math.abs(num1 - x) - Math.abs(num2 - x));
            sortedArr = sortedArr.subList(0, k);
            Collections.sort(sortedArr);
            return sortedArr;
        }
    }

}
