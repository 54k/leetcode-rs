package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day974 {
    // https://leetcode.com/problems/delete-duplicate-folders-in-system/description/?envType=daily-question&envId=2025-07-20
    static class Solution1 {
        static class Trie {
            String serial;
            Map<String, Trie> children = new HashMap<>();
        }

        public List<List<String>> deleteDuplicateFolder(List<List<String>> paths) {
            Trie root = new Trie();

            for (List<String> path : paths) {
                Trie cur = root;
                for (String node : path) {
                    cur.children.putIfAbsent(node, new Trie());
                    cur = cur.children.get(node);
                }
            }

            Map<String, Integer> freq = new HashMap<>();
            construct(root, freq);
            List<List<String>> ans = new ArrayList<>();
            List<String> path = new ArrayList<>();
            operate(root, freq, path, ans);
            return ans;
        }

        void construct(Trie node, Map<String, Integer> freq) {
            if (node.children.isEmpty())
                return;

            List<String> v = new ArrayList<>();
            for (Map.Entry<String, Trie> entry : node.children.entrySet()) {
                construct(entry.getValue(), freq);
                v.add(entry.getKey() + "(" + entry.getValue().serial + ")");
            }

            Collections.sort(v);
            StringBuilder sb = new StringBuilder();
            for (String s : v) {
                sb.append(s);
            }
            node.serial = sb.toString();
            freq.put(node.serial, freq.getOrDefault(node.serial, 0) + 1);
        }

        void operate(Trie node, Map<String, Integer> freq, List<String> path, List<List<String>> ans) {
            if (freq.getOrDefault(node.serial, 0) > 1)
                return;

            if (!path.isEmpty()) {
                ans.add(new ArrayList<>(path));
            }

            for (Map.Entry<String, Trie> entry : node.children.entrySet()) {
                path.add(entry.getKey());
                operate(entry.getValue(), freq, path, ans);
                path.remove(path.size() - 1);
            }
        }
    }

}
