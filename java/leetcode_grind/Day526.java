package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Stack;

public class Day526 {
    // https://leetcode.com/problems/find-if-path-exists-in-graph/description
    static class Solution1 {
        public boolean validPath(int n, int[][] edges, int source, int destination) {
            var adj = new HashMap<Integer, List<Integer>>();
            for (var e : edges) {
                adj.putIfAbsent(e[0], new ArrayList<>());
                adj.get(e[0]).add(e[1]);
                adj.putIfAbsent(e[1], new ArrayList<>());
                adj.get(e[1]).add(e[0]);
            }
            var vis = new HashSet<Integer>();
            var st = new Stack<Integer>();
            st.push(source);
            vis.add(source);
            while (!st.isEmpty()) {
                int v = st.pop();
                if (v == destination) {
                    return true;
                }
                for (var u : adj.getOrDefault(v, List.of())) {
                    if (!vis.contains(u)) {
                        vis.add(u);
                        st.push(u);
                    }
                }
            }
            return false;
        }
    }

}
