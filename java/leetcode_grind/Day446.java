package leetcode_grind;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;

public class Day446 {
    // https://leetcode.com/problems/people-whose-list-of-favorite-companies-is-not-a-subset-of-another-list/description/
    static class Solution1 {
        public List<Integer> peopleIndexes(List<List<String>> favoriteCompanies) {
            var fc = favoriteCompanies.stream().map(x -> {
                return new HashSet<String>(x);
            }).toList();

            var ans = new ArrayList<Integer>();

            for (int i = 0; i < fc.size(); ++i) {
                var str = fc.get(i);

                boolean unique = true;
                for (int j = 0; j < fc.size(); ++j) {
                    if (i == j)
                        continue;

                    if (fc.get(j).containsAll(str)) {
                        unique = false;
                    }
                }

                if (unique) {
                    ans.add(i);
                }
            }

            return ans;
        }
    }
}
