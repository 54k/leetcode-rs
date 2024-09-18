package leetcode_grind;

import java.util.*;

public class Day671 {
    // https://leetcode.com/problems/largest-number/description/?envType=daily-question&envId=2024-09-18
    static class Solution1 {
        public String largestNumber(int[] nums) {
            quickSort(nums, 0, nums.length - 1);
            StringBuilder largestNum = new StringBuilder();
            for (int num : nums) {
                largestNum.append(num);
            }
            return largestNum.charAt(0) == '0' ? "0" : largestNum.toString();
        }

        void quickSort(int[] nums, int left, int right) {
            if (left >= right)
                return;
            int pivotIndex = partition(nums, left, right);
            quickSort(nums, left, pivotIndex - 1);
            quickSort(nums, pivotIndex + 1, right);
        }

        int partition(int[] nums, int left, int right) {
            int pivot = nums[right];
            int lowIndex = left;
            for (int i = left; i < right; ++i) {
                if (compare(nums[i], pivot)) {
                    swap(nums, i, lowIndex);
                    ++lowIndex;
                }
            }
            swap(nums, lowIndex, right);
            return lowIndex;
        }

        boolean compare(int firstNum, int secondNum) {
            String str1 = String.valueOf(firstNum) + String.valueOf(secondNum);
            String str2 = String.valueOf(secondNum) + String.valueOf(firstNum);
            return str1.compareTo(str2) > 0;
        }

        void swap(int[] nums, int i, int j) {
            int temp = nums[i];
            nums[i] = nums[j];
            nums[j] = temp;
        }
    }

    static class Solution2 {
        public String largestNumber(int[] nums) {
            List<Integer> sortedNums = mergeSort(nums, 0, nums.length - 1);
            StringBuilder largestNum = new StringBuilder();
            for (int num : sortedNums) {
                largestNum.append(num);
            }
            return largestNum.charAt(0) == '0' ? "0" : largestNum.toString();
        }

        List<Integer> mergeSort(int[] nums, int left, int right) {
            if (left >= right)
                return List.of(nums[left]);
            int mid = left + (right - left) / 2;
            List<Integer> leftHalf = mergeSort(nums, left, mid);
            List<Integer> rightHalf = mergeSort(nums, mid + 1, right);
            return merge(leftHalf, rightHalf);
        }

        List<Integer> merge(List<Integer> leftHalf, List<Integer> rightHalf) {
            List<Integer> sortedNums = new ArrayList<>();
            int leftIndex = 0, rightIndex = 0;
            while (leftIndex < leftHalf.size() && rightIndex < rightHalf.size()) {
                if (compare(leftHalf.get(leftIndex), rightHalf.get(rightIndex))) {
                    sortedNums.add(leftHalf.get(leftIndex++));
                } else {
                    sortedNums.add(rightHalf.get(rightIndex++));
                }
            }
            while (leftIndex < leftHalf.size())
                sortedNums.add(leftHalf.get(leftIndex++));
            while (rightIndex < rightHalf.size())
                sortedNums.add(rightHalf.get(rightIndex++));
            return sortedNums;
        }

        boolean compare(int firstNum, int secondNum) {
            String s1 = String.valueOf(firstNum) + String.valueOf(secondNum);
            String s2 = String.valueOf(secondNum) + String.valueOf(firstNum);
            return s1.compareTo(s2) > 0;
        }
    }

    static class Solution3 {
        public String largestNumber(int[] nums) {
            PriorityQueue<String> maxHeap = new PriorityQueue<>(
                    new Comparator<String>() {
                        @Override
                        public int compare(String first, String second) {
                            return (second + first).compareTo(first + second);
                        }
                    });

            int totalLength = 0;

            for (int num : nums) {
                String strNum = Integer.toString(num);
                totalLength += strNum.length();
                maxHeap.offer(strNum);
            }

            StringBuilder result = new StringBuilder(totalLength);
            while (!maxHeap.isEmpty()) {
                result.append(maxHeap.poll());
            }

            return result.charAt(0) == '0' ? "0" : result.toString();
        }
    }

    static class Solution4 {

        static final int RUN = 32;

        public String largestNumber(int[] nums) {
            Integer[] numsArray = Arrays.stream(nums)
                    .boxed()
                    .toArray(Integer[]::new);
            timSort(numsArray);
            StringBuilder largestNum = new StringBuilder();
            for (int num : numsArray) {
                largestNum.append(num);
            }
            return largestNum.charAt(0) == '0' ? "0" : largestNum.toString();
        }

        void insertionSort(Integer[] nums, int left, int right) {
            for (int i = left + 1; i <= right; i++) {
                int temp = nums[i];
                int j = i - 1;
                while (j >= left && compare(temp, nums[j])) {
                    nums[j + 1] = nums[j];
                    --j;
                }
                nums[j + 1] = temp;
            }
        }

        void merge(Integer[] nums, int left, int mid, int right) {
            Integer[] leftArr = Arrays.copyOfRange(nums, left, mid + 1);
            Integer[] rightArr = Arrays.copyOfRange(nums, mid + 1, right + 1);
            int i = 0, j = 0, k = left;
            while (i < leftArr.length && j < rightArr.length) {
                if (compare(leftArr[i], rightArr[j])) {
                    nums[k++] = leftArr[i++];
                } else {
                    nums[k++] = rightArr[j++];
                }
            }
            while (i < leftArr.length)
                nums[k++] = leftArr[i++];
            while (j < rightArr.length)
                nums[k++] = rightArr[j++];
        }

        void timSort(Integer[] nums) {
            int n = nums.length;
            for (int i = 0; i < n; i += RUN) {
                insertionSort(nums, i, Math.min(i + RUN - 1, n - 1));
            }
            for (int size = RUN; size < n; size *= 2) {
                for (int left = 0; left < n; left += 2 * size) {
                    int mid = left + size - 1;
                    int right = Math.min(left + 2 * size - 1, n - 1);
                    if (mid < right) {
                        merge(nums, left, mid, right);
                    }
                }
            }
        }

        boolean compare(int firstNum, int secondNum) {
            String firstStr = Integer.toString(firstNum);
            String secondStr = Integer.toString(secondNum);
            return (firstStr + secondStr).compareTo(secondStr + firstStr) > 0;
        }
    }
}
