package leetcode_grind;

public class Day929 {
    // https://leetcode.com/problems/lexicographically-smallest-equivalent-string/description/?envType=daily-question&envId=2025-06-05
    static class Solution1 {
        int minChar;

        void dfs(int src, Integer[][] adjMatrix, Integer visited[], List<Integer> component) {
            visited[src] = 1;
            component.add(src);
            minChar = Math.min(minChar, src);
            for (int i = 0; i < 26; i++) {
                if (adjMatrix[src][i] != null && visited[i] == null) {
                    dfs(i, adjMatrix, visited, component);
                }
            }
        }

        public String smallestEquivalentString(String s1, String s2, String baseStr) {
            Integer adjMatrix[][] = new Integer[26][26];
            for (int i = 0; i < s1.length(); i++) {
                adjMatrix[s1.charAt(i) - 'a'][s2.charAt(i) - 'a'] = 1;
                adjMatrix[s2.charAt(i) - 'a'][s1.charAt(i) - 'a'] = 1;
            }

            int[] mappingChar = new int[26];
            for (int i = 0; i < 26; i++) {
                mappingChar[i] = i;
            }

            Integer visited[] = new Integer[26];
            for (int c = 0; c < 26; c++) {
                if (visited[c] == null) {
                    List<Integer> component = new ArrayList<>();
                    minChar = 27;
                    dfs(c, adjMatrix, visited, component);
                    for (int vertex : component) {
                        mappingChar[vertex] = minChar;
                    }
                }
            }

            String ans = "";
            for (char c : baseStr.toCharArray()) {
                ans += (char) (mappingChar[c - 'a'] + 'a');
            }
            return ans;
        }
    }

    static class Solution2 {
        int[] representative = new int[26];

        int find(int x) {
            if (representative[x] == x) {
                return x;
            }
            return representative[x] = find(representative[x]);
        }

        void performUnion(int x, int y) {
            x = find(x);
            y = find(y);

            if (x == y) {
                return;
            }

            if (x < y) {
                representative[y] = x;
            } else {
                representative[x] = y;
            }
        }

        public String smallestEquivalentString(String s1, String s2, String baseStr) {
            for (int i = 0; i < 26; i++) {
                representative[i] = i;
            }

            for (int i = 0; i < s1.length(); i++) {
                performUnion(s1.charAt(i) - 'a', s2.charAt(i) - 'a');
            }

            String ans = "";
            for (char c : baseStr.toCharArray()) {
                ans += (char) (find(c - 'a') + 'a');
            }
            return ans;
        }
    }

}
