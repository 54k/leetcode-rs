package leetcode_grind;

public class Day613 {
    // https://leetcode.com/problems/sort-the-people/description/?envType=daily-question&envId=2024-07-22
    static class Solution1 {
        public String[] sortPeople(String[] names, int[] heights) {
            quickSort(heights, names, 0, heights.length - 1);
            return names;
        }

        void swap(int[] heights, String[] names, int index1, int index2) {
            int tempHeight = heights[index1];
            heights[index1] = heights[index2];
            heights[index2] = tempHeight;

            String tempName = names[index1];
            names[index1] = names[index2];
            names[index2] = tempName;
        }

        int partition(int[] heights, String[] names, int start, int end) {
            int pivot = heights[end];
            int i = start - 1;

            for (int j = start; j < end; j++) {
                if (heights[j] >= pivot) {
                    i++;
                    swap(heights, names, i, j);
                }
            }

            swap(heights, names, i + 1, end);
            return i + 1;
        }

        void quickSort(int[] heights, String[] names, int start, int end) {
            if (start < end) {
                int partitionIndex = partition(heights, names, start, end);

                quickSort(heights, names, start, partitionIndex - 1);
                quickSort(heights, names, partitionIndex + 1, end);
            }
        }
    }
}
