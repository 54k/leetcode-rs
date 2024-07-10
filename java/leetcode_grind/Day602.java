package leetcode_grind;

import java.util.Stack;

public class Day602 {
    // https://leetcode.com/problems/crawler-log-folder/description/?envType=daily-question&envId=2024-07-10
    static class Solution1 {
        public int minOperations(String[] logs) {
            Stack<String> folderStack = new Stack<>();
            for (String currentOperation : logs) {
                if (currentOperation.equals("../")) {
                    if (!folderStack.isEmpty()) {
                        folderStack.pop();
                    }
                } else if (!currentOperation.equals("./")) {
                    folderStack.push(currentOperation);
                }
            }
            return folderStack.size();
        }
    }
}
