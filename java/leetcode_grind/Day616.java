package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;

public class Day616 {
    // https://leetcode.com/problems/sort-an-array/description/?envType=daily-question&envId=2024-07-25
    static class Solution1 {
        public int[] sortArray(int[] nums) {
            var n = nums.length;
            var ms = new Object() {
                int[] tmp = new int[n];

                void apply(int[] nums, int l, int r) {
                    if (l + 1 == r) {
                        return;
                    }
                    int mid = (l + r) / 2;
                    apply(nums, l, mid);
                    apply(nums, mid, r);
                    merge(nums, l, mid, r);
                }

                void merge(int[] nums, int l, int m, int r) {
                    int k = l;
                    int i = l;
                    int j = m;
                    while (i < m && j < r) {
                        if (nums[i] <= nums[j]) {
                            tmp[k++] = nums[i++];
                        } else {
                            tmp[k++] = nums[j++];
                        }
                    }
                    while (i < m) {
                        tmp[k++] = nums[i++];
                    }
                    while (j < r) {
                        tmp[k++] = nums[j++];
                    }
                    while (l < r) {
                        nums[l] = tmp[l];
                        l++;
                    }
                }
            };
            ms.apply(nums, 0, n);
            return nums;
        }
    }

    static class Solution2 {
        public int[] sortArray(int[] nums) {
            int n = nums.length;
            var qs = new Object() {
                void apply(int l, int r) {
                    if (l + 1 >= r) {
                        return;
                    }

                    int mid = partition(l, r);
                    apply(l, mid);
                    apply(mid + 1, r);
                }

                int partition(int l, int r) {
                    int pp = nums[r - 1];
                    int i = l;
                    int j = l;
                    for (; j < r; j++) {
                        if (nums[j] < pp) {
                            int t = nums[i];
                            nums[i++] = nums[j];
                            nums[j] = t;
                        }
                    }
                    int t = nums[i];
                    nums[i] = pp;
                    nums[j - 1] = t;
                    return i;
                }
            };
            qs.apply(0, n);
            return nums;
        }
    }

    static class Solution3 {
        void swap(int[] arr, int index1, int index2) {
            int temp = arr[index1];
            arr[index1] = arr[index2];
            arr[index2] = temp;
        }

        void heapify(int[] arr, int n, int i) {
            int largest = i;
            int left = 2 * i + 1;
            int right = 2 * i + 2;

            if (left < n && arr[left] > arr[largest]) {
                largest = left;
            }

            if (right < n && arr[right] > arr[largest]) {
                largest = right;
            }

            if (largest != i) {
                swap(arr, i, largest);
                heapify(arr, n, largest);
            }
        }

        void heapSort(int[] arr) {
            int n = arr.length;
            for (int i = n / 2 - 1; i >= 0; i--) {
                heapify(arr, n, i);
            }

            for (int i = n - 1; i >= 0; i--) {
                swap(arr, 0, i);
                heapify(arr, i, 0);
            }
        }

        public int[] sortArray(int[] nums) {
            heapSort(nums);
            return nums;
        }
    }

    static class Solution4 {
        public int[] sortArray(int[] nums) {
            int[] arr = nums;

            HashMap<Integer, Integer> counts = new HashMap<>();
            int minVal = arr[0], maxVal = arr[0];

            for (int i = 0; i < arr.length; i++) {
                minVal = Math.min(arr[i], minVal);
                maxVal = Math.max(arr[i], maxVal);
                counts.put(arr[i], counts.getOrDefault(arr[i], 0) + 1);
            }

            int index = 0;
            for (int val = minVal; val <= maxVal; ++val) {
                while (counts.getOrDefault(val, 0) > 0) {
                    arr[index] = val;
                    index += 1;
                    counts.put(val, counts.get(val) - 1);
                }
            }

            return nums;
        }
    }

    static class Solution5 {
        void bucketSort(int[] arr, int placeValue) {
            ArrayList<List<Integer>> buckets = new ArrayList<>(10);
            for (int digit = 0; digit < 10; ++digit) {
                buckets.add(digit, new ArrayList<Integer>());
            }

            for (int val : arr) {
                int digit = Math.abs(val) / placeValue;
                digit = digit % 10;
                buckets.get(digit).add(val);
            }

            int index = 0;
            for (int digit = 0; digit < 10; ++digit) {
                for (int val : buckets.get(digit)) {
                    arr[index] = val;
                    index++;
                }
            }
        }

        void radixSort(int[] arr) {
            int maxElement = arr[0];
            for (int val : arr) {
                maxElement = Math.max(Math.abs(val), maxElement);
            }
            int maxDigits = 0;
            while (maxElement > 0) {
                maxDigits += 1;
                maxElement /= 10;
            }

            int placeValue = 1;
            for (int digit = 0; digit < maxDigits; ++digit) {
                bucketSort(arr, placeValue);
                placeValue *= 10;
            }

            ArrayList<Integer> negatives = new ArrayList<>();
            ArrayList<Integer> positives = new ArrayList<>();
            for (int val : arr) {
                if (val < 0) {
                    negatives.add(val);
                } else {
                    positives.add(val);
                }
            }

            Collections.reverse(negatives);
            int index = 0;
            for (int val : negatives) {
                arr[index] = val;
                index++;
            }
            for (int val : positives) {
                arr[index] = val;
                index++;
            }
        }

        public int[] sortArray(int[] nums) {
            radixSort(nums);
            return nums;
        }
    }
}
