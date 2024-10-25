package leetcode_grind;

import java.util.*;

public class Day708 {
    // https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/description/?envType=daily-question&envId=2024-10-25
    static class Solution1 {
        public List<String> removeSubfolders(String[] folder) {
            Set<String> folderSet = new HashSet<>(Arrays.asList(folder));
            List<String> result = new ArrayList<>();
            for (String f : folder) {
                boolean isSubFolder = false;
                String prefix = f;

                while (!prefix.isEmpty()) {
                    int pos = prefix.lastIndexOf('/');
                    if (pos == -1)
                        break;
                    prefix = prefix.substring(0, pos);
                    if (folderSet.contains(prefix)) {
                        isSubFolder = true;
                        break;
                    }
                }

                if (!isSubFolder) {
                    result.add(f);
                }
            }
            return result;
        }
    }

    static class Solution2 {
        public List<String> removeSubfolders(String[] folder) {
            Arrays.sort(folder);
            List<String> result = new ArrayList<>();
            result.add(folder[0]);

            for (int i = 1; i < folder.length; i++) {
                String lastFolder = result.get(result.size() - 1);
                lastFolder += '/';

                if (!folder[i].startsWith(lastFolder)) {
                    result.add(folder[i]);
                }
            }

            return result;
        }
    }

    static class Solution3 {
        static class TrieNode {
            boolean isEndOfFolder;
            HashMap<String, TrieNode> children;

            TrieNode() {
                this.isEndOfFolder = false;
                this.children = new HashMap<>();
            }
        }

        TrieNode root;

        Solution3() {
            root = new TrieNode();
        }

        public List<String> removeSubfolders(String[] folder) {
            for (String path : folder) {
                TrieNode currentNode = root;
                String[] folderNames = path.split("/");
                for (String folderName : folderNames) {
                    if (folderName.equals(""))
                        continue;

                    if (!currentNode.children.containsKey(folderName)) {
                        currentNode.children.put(folderName, new TrieNode());
                    }
                    currentNode = currentNode.children.get(folderName);
                }

                currentNode.isEndOfFolder = true;
            }

            List<String> result = new ArrayList<>();
            for (String path : folder) {
                TrieNode currentNode = root;
                String[] folderNames = path.split("/");
                boolean isSubfolder = false;
                for (int i = 0; i < folderNames.length; i++) {
                    if (folderNames[i].equals(""))
                        continue;
                    TrieNode nextNode = currentNode.children.get(folderNames[i]);

                    if (nextNode.isEndOfFolder && i != folderNames.length - 1) {
                        isSubfolder = true;
                        break;
                    }

                    currentNode = nextNode;
                }
                if (!isSubfolder)
                    result.add(path);
            }
            return result;
        }
    }

}
