package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.function.Function;

public class Day366 {
    // https://leetcode.com/problems/sort-vowels-in-a-string/description
    static class Solution1 {
        public String sortVowels(String s) {
            Function<Character, Boolean> isVowel = (vow) -> {
                return vow == 'a' || vow == 'e' || vow == 'i' || vow == 'o' || vow == 'u' || vow == 'i' ||
                        vow == 'A' || vow == 'E' || vow == 'I' || vow == 'O' || vow == 'U' || vow == 'I';
            };
            int[] count = new int[1000];
            for (var ch : s.toCharArray()) {
                if (isVowel.apply(ch)) {
                    count[ch - 'A']++;
                }
            }

            var vi = 0;
            var ans = new StringBuilder();
            for (var ch : s.toCharArray()) {
                if (isVowel.apply(ch)) {
                    while (count[vi] == 0) {
                        vi++;
                    }
                    count[vi]--;
                    ans.append((char) (vi + 'A'));
                } else {
                    ans.append(ch);
                }
            }

            return ans.toString();
        }
    }

    // https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/description/
    static class Solution2 {
        Map<Integer, List<Integer>> graph = new HashMap<>();
        List<Integer> result = new ArrayList<>();
        Set<Integer> visited = new HashSet<>();
        int k;

        void toGraph(TreeNode root, TreeNode parent) {
            if (root == null) {
                return;
            }

            if (!graph.containsKey(root.val)) {
                graph.put(root.val, new ArrayList<>());
            }

            if (parent != null) {
                if (!graph.containsKey(parent.val)) {
                    graph.put(parent.val, new ArrayList<>());
                }
                graph.get(parent.val).add(root.val);
                graph.get(root.val).add(parent.val);
            }

            toGraph(root.left, root);
            toGraph(root.right, root);
        }

        void dfs(int v, int d) {
            if (d == k) {
                result.add(v);
                return;
            }
            visited.add(v);
            for (var u : graph.get(v)) {
                if (!visited.contains(u)) {
                    dfs(u, d + 1);
                }
            }
        }

        public List<Integer> distanceK(TreeNode root, TreeNode target, int k) {
            this.k = k;
            toGraph(root, null);
            dfs(target.val, 0);
            return result;
        }
    }
}
