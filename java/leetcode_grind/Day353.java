package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Random;
import java.util.Set;

public class Day353 {

    // https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/description/
    static class Solution {
        class RandomizedCollection {
            List<Integer> lst = new ArrayList<>();
            Map<Integer, Set<Integer>> map = new HashMap<>();
            Random rand = new Random();

            public RandomizedCollection() {
            }

            public boolean insert(int val) {
                if (!map.containsKey(val)) {
                    map.put(val, new HashSet<>());
                }

                map.get(val).add(lst.size());
                lst.add(val);
                return map.get(val).size() == 1;
            }

            public boolean remove(int val) {
                if (!map.containsKey(val) || map.get(val).size() == 0) {
                    return false;
                }
                var removeIdx = map.get(val).iterator().next();
                map.get(val).remove(removeIdx);

                var last = lst.get(lst.size() - 1);
                lst.set(removeIdx, last);

                map.get(last).add(removeIdx);
                map.get(last).remove(lst.size() - 1);
                lst.remove(lst.size() - 1);

                return true;
            }

            public int getRandom() {
                return lst.get(rand.nextInt(lst.size()));
            }
        }
    }
}
