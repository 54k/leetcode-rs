package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;

public class Day584 {
    // https://leetcode.com/problems/design-in-memory-file-system/description/
    static class FileSystem1 {
        class Dir {
            HashMap<String, Dir> dirs = new HashMap<>();
            HashMap<String, String> files = new HashMap<>();
        }

        Dir root;

        public FileSystem1() {
            root = new Dir();
        }

        public List<String> ls(String path) {
            Dir t = root;
            List<String> files = new ArrayList<>();

            if (!path.equals("/")) {
                String[] d = path.split("/");
                for (int i = 1; i < d.length - 1; i++) {
                    t = t.dirs.get(d[i]);
                }
                if (t.files.containsKey(d[d.length - 1])) {
                    files.add(d[d.length - 1]);
                    return files;
                } else {
                    t = t.dirs.get(d[d.length - 1]);
                }
            }
            files.addAll(new ArrayList<>(t.dirs.keySet()));
            files.addAll(new ArrayList<>(t.files.keySet()));
            Collections.sort(files);
            return files;
        }

        public void mkdir(String path) {
            Dir t = root;
            String[] d = path.split("/");
            for (int i = 1; i < d.length; i++) {
                if (!t.dirs.containsKey(d[i])) {
                    t.dirs.put(d[i], new Dir());
                }
                t = t.dirs.get(d[i]);
            }
        }

        public void addContentToFile(String filePath, String content) {
            Dir t = root;
            String[] d = filePath.split("/");
            for (int i = 1; i < d.length - 1; i++) {
                t = t.dirs.get(d[i]);
            }
            t.files.put(d[d.length - 1], t.files.getOrDefault(d[d.length - 1], "") + content);
        }

        public String readContentFromFile(String filePath) {
            Dir t = root;
            String[] d = filePath.split("/");
            for (int i = 1; i < d.length - 1; i++) {
                t = t.dirs.get(d[i]);
            }
            return t.files.get(d[d.length - 1]);
        }

        static class FileSystem2 {
            class File {
                boolean isFile = false;
                HashMap<String, File> files = new HashMap<>();
                String content = "";
            }

            File root;

            public FileSystem2() {
                root = new File();
            }

            public List<String> ls(String path) {
                File t = root;
                List<String> files = new ArrayList<>();
                if (!path.equals("/")) {
                    String[] d = path.split("/");
                    for (int i = 1; i < d.length; i++) {
                        t = t.files.get(d[i]);
                    }
                    if (t.isFile) {
                        files.add(d[d.length - 1]);
                        return files;
                    }
                }
                List<String> resFiles = new ArrayList<>(t.files.keySet());
                Collections.sort(resFiles);
                return resFiles;
            }

            public void mkdir(String path) {
                File t = root;
                String[] d = path.split("/");
                for (int i = 1; i < d.length; i++) {
                    if (!t.files.containsKey(d[i])) {
                        t.files.put(d[i], new File());
                    }
                    t = t.files.get(d[i]);
                }
            }

            public void addContentToFile(String filePath, String content) {
                File t = root;
                String[] d = filePath.split("/");
                for (int i = 1; i < d.length - 1; i++) {
                    t = t.files.get(d[i]);
                }
                if (!t.files.containsKey(d[d.length - 1])) {
                    t.files.put(d[d.length - 1], new File());
                }
                t = t.files.get(d[d.length - 1]);
                t.isFile = true;
                t.content = t.content + content;
            }

            public String readContentFromFile(String filePath) {
                File t = root;
                String[] d = filePath.split("/");
                for (int i = 1; i < d.length - 1; i++) {
                    t = t.files.get(d[i]);
                }
                return t.files.get(d[d.length - 1]).content;
            }
        }

        // https://leetcode.com/problems/grumpy-bookstore-owner/description
        static class Solution1 {
            public int maxSatisfied(int[] customers, int[] grumpy, int minutes) {
                int n = customers.length;
                int unrealizedCustomers = 0;

                for (int i = 0; i < minutes; i++) {
                    unrealizedCustomers += customers[i] * grumpy[i];
                }

                int maxUnrealizedCustomers = unrealizedCustomers;
                for (int i = minutes; i < n; i++) {
                    unrealizedCustomers += customers[i] * grumpy[i];
                    unrealizedCustomers -= customers[i - minutes] * grumpy[i - minutes];
                    maxUnrealizedCustomers = Math.max(maxUnrealizedCustomers, unrealizedCustomers);
                }

                int totalCustomers = maxUnrealizedCustomers;
                for (int i = 0; i < customers.length; i++) {
                    totalCustomers += customers[i] * (1 - grumpy[i]);
                }
                return totalCustomers;
            }
        }
    }
}
