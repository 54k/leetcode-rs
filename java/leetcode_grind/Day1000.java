package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day1000 {
    // https://leetcode.com/problems/power-of-four/description/?envType=daily-question&envId=2025-08-15
    static class Solution1 {
        public boolean isPowerOfFour(int num) {
            return (num > 0) && (num & (num - 1)) == 0 && (num & 0xaaaaaaaa) == 0;
        }
    }

    // https://leetcode.com/problems/design-file-system/description/?envType=weekly-question&envId=2025-08-15
    static class FileSystem1 {

        Map<String, Integer> paths;

        public FileSystem1() {
            paths = new HashMap<>();
        }

        public boolean createPath(String path, int value) {
            if (path.isEmpty() || (path.length() == 1 && path.equals("/")) || this.paths.containsKey(path)) {
                return false;
            }

            int delimIndex = path.lastIndexOf("/");
            String parent = path.substring(0, delimIndex);

            if (parent.length() > 1 && !this.paths.containsKey(parent)) {
                return false;
            }

            this.paths.put(path, value);
            return true;
        }

        public int get(String path) {
            return this.paths.getOrDefault(path, -1);
        }
    }

    static class FileSystem2 {
        static class TrieNode {
            String name;
            int val = -1;
            Map<String, TrieNode> map = new HashMap<>();

            TrieNode(String name) {
                this.name = name;
            }
        }

        TrieNode root;

        public FileSystem2() {
            this.root = new TrieNode("");
        }

        public boolean createPath(String path, int value) {
            String[] components = path.split("/");
            TrieNode cur = root;

            for (int i = 1; i < components.length; i++) {
                String currentComponent = components[i];

                if (cur.map.containsKey(currentComponent) == false) {
                    if (i == components.length - 1) {
                        cur.map.put(currentComponent, new TrieNode(currentComponent));
                    } else {
                        return false;
                    }
                }

                cur = cur.map.get(currentComponent);
            }

            if (cur.val != -1) {
                return false;
            }
            cur.val = value;
            return true;
        }

        public int get(String path) {
            String[] components = path.split("/");
            TrieNode cur = root;

            for (int i = 1; i < components.length; i++) {
                String currentComponent = components[i];

                if (cur.map.containsKey(currentComponent) == false) {
                    return -1;
                }

                cur = cur.map.get(currentComponent);
            }

            return cur.val;
        }
    }
}
