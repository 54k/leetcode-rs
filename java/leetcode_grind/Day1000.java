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

    /**
     * Your FileSystem object will be instantiated and called as such:
     * FileSystem obj = new FileSystem();
     * boolean param_1 = obj.createPath(path,value);
     * int param_2 = obj.get(path);
     */
}
