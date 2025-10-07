package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import java.util.TreeSet;

public class Day1052 {
    // https://leetcode.com/problems/avoid-flood-in-the-city/description/?envType=daily-question&envId=2025-10-07
    static class Solution1 {
        public int[] avoidFlood(int[] rains) {
            int[] ans = new int[rains.length];
            Arrays.fill(ans, 1);
            TreeSet<Integer> st = new TreeSet<>();
            Map<Integer, Integer> mp = new HashMap<>();
            for (int i = 0; i < rains.length; i++) {
                if (rains[i] == 0) {
                    st.add(i);
                } else {
                    ans[i] = -1;
                    if (mp.containsKey(rains[i])) {
                        Integer it = st.ceiling(mp.get(rains[i]));
                        if (it == null) {
                            return new int[0];
                        }
                        ans[it] = rains[i];
                        st.remove(it);
                    }
                    mp.put(rains[i], i);
                }
            }
            return ans;
        }
    }

}
