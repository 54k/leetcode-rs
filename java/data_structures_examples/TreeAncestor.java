package data_structures_examples;

import java.util.Arrays;

// https://leetcode.com/problems/kth-ancestor-of-a-tree-node/description/
public class TreeAncestor {

    int maxD = 0;
    int[][] height;
    int[][] kthAncestor;

    public TreeAncestor(int n, int[] parent) {
        while ((1 << maxD) <= n) {
            maxD += 1;
        }

        kthAncestor = new int[n][maxD];
        for (int i = 0; i < n; i++) {
            Arrays.fill(kthAncestor[i], -1);
        }

        for (int i = 0; i < maxD; i++) {
            for (int j = 0; j < n; j++) {
                if (i == 0) {
                    kthAncestor[j][i] = parent[j];
                } else if (kthAncestor[j][i - 1] != -1) {
                    kthAncestor[j][i] = kthAncestor[kthAncestor[j][i - 1]][i - 1];
                }
            }
        }
    }

    public int getKthAncestor(int node, int k) {
        for (int i = 0; i < maxD; i++) {
            if (((1 << i) & k) != 0) {
                node = kthAncestor[node][i];
                if (node == -1) {
                    break;
                }
            }
        }
        return node;
    }
}
