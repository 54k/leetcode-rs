package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Day1154 {
    // https://leetcode.com/problems/maximum-square-area-by-removing-fences-from-a-field/description/?envType=daily-question&envId=2026-01-16
    static class Solution1 {

        public int maximizeSquareArea(int m, int n, int[] hFences, int[] vFences) {
            Set<Integer> hEdges = getEdges(hFences, m);
            Set<Integer> vEdges = getEdges(vFences, n);

            long res = 0;
            for (int e : hEdges) {
                if (vEdges.contains(e)) {
                    res = Math.max(res, e);
                }
            }

            if (res == 0) {
                return -1;
            } else {
                return (int) ((res * res) % 1000000007);
            }
        }

        private Set<Integer> getEdges(int[] fences, int border) {
            Set<Integer> set = new HashSet<>();
            List<Integer> list = new ArrayList<>();

            for (int fence : fences) {
                list.add(fence);
            }

            list.add(1);
            list.add(border);
            Collections.sort(list);

            for (int i = 0; i < list.size(); i++) {
                for (int j = i + 1; j < list.size(); j++) {
                    set.add(list.get(j) - list.get(i));
                }
            }

            return set;
        }
    }

}
