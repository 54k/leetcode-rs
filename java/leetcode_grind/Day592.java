package leetcode_grind;

public class Day592 {
    // https://leetcode.com/problems/remove-max-number-of-edges-to-keep-graph-fully-traversable/description/?envType=daily-question&envId=2024-06-30
    static class Solution1 {
        public int maxNumEdgesToRemove(int n, int[][] edges) {
            UnionFind Alice = new UnionFind(n);
            UnionFind Bob = new UnionFind(n);

            int edgesRequired = 0;
            for (int[] edge : edges) {
                if (edge[0] == 3) {
                    edgesRequired += (Alice.performUnion(edge[1], edge[2]) | Bob.performUnion(edge[1], edge[2]));
                }
            }
            for (int[] edge : edges) {
                if (edge[0] == 1) {
                    edgesRequired += Alice.performUnion(edge[1], edge[2]);
                } else if (edge[0] == 2) {
                    edgesRequired += Bob.performUnion(edge[1], edge[2]);
                }
            }
            if (Alice.isConnected() && Bob.isConnected()) {
                return edges.length - edgesRequired;
            }
            return -1;
        }

        class UnionFind {
            int[] representatives;
            int[] componentSize;
            int components;

            UnionFind(int n) {
                components = n;
                representatives = new int[n + 1];
                componentSize = new int[n + 1];
                for (int i = 0; i <= n; i++) {
                    representatives[i] = i;
                    componentSize[i] = 1;
                }
            }

            int findRepresentative(int x) {
                if (representatives[x] == x) {
                    return x;
                }
                return representatives[x] = findRepresentative(representatives[x]);
            }

            int performUnion(int x, int y) {
                x = findRepresentative(x);
                y = findRepresentative(y);
                if (x == y) {
                    return 0;
                }
                if (componentSize[x] > componentSize[y]) {
                    componentSize[x] += componentSize[y];
                    representatives[y] = x;
                } else {
                    componentSize[y] += componentSize[x];
                    representatives[x] = y;
                }
                components--;
                return 1;
            }

            boolean isConnected() {
                return components == 1;
            }
        }
    }
}
