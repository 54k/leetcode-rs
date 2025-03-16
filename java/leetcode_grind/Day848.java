package leetcode_grind;

public class Day848 {
    // https://leetcode.com/problems/minimum-time-to-repair-cars/description/?envType=daily-question&envId=2025-03-16
    static class Solution1 {
        public long repairCars(int[] ranks, int cars) {
            int minRank = ranks[0], maxRank = ranks[0];
            for (int rank : ranks) {
                minRank = Math.min(minRank, rank);
                maxRank = Math.max(maxRank, rank);
            }

            int[] freq = new int[maxRank + 1];
            for (int rank : ranks) {
                minRank = Math.min(minRank, rank);
                freq[rank]++;
            }

            long low = 1, high = 1L * minRank * cars * cars;
            while (low < high) {
                long mid = (low + high) / 2;
                long carsRepaired = 0;

                for (int rank = 1; rank <= maxRank; rank++) {
                    carsRepaired += freq[rank] * (long) Math.sqrt(mid / (long) rank);
                }

                if (carsRepaired >= cars) {
                    high = mid;
                } else {
                    low = mid + 1;
                }
            }
            return low;
        }
    }

    static class Solution2 {
        public long repairCars(int[] ranks, int cars) {
            long low = 1, high = 1L * ranks[0] * cars * cars;
            while (low < high) {
                long mid = (low + high) / 2, carsRepaired = 0;
                for (int rank : ranks) {
                    carsRepaired += (long) (Math.sqrt((1.0 * mid) / rank));
                }
                if (carsRepaired < cars)
                    low = mid + 1;
                else
                    high = mid;
            }
            return low;
        }
    }

    // https://leetcode.com/problems/nth-digit/description/
    static class Solution4 {
        public int findNthDigit(int n) {
            // Шаг 1: Определяем, сколько цифр имеют числа в нужном диапазоне
            long base = 9; // Количество чисел в текущей группе (9, 90, 900, ...)
            int digits = 1; // Количество цифр в каждом числе текущей группы

            // Пока n больше, чем количество цифр в текущем диапазоне, уменьшаем n
            while (n - base * digits > 0) {
                n -= base * digits; // Вычитаем цифры, уже учтенные в предыдущих диапазонах
                base *= 10; // Переходим к следующему диапазону (1-9, 10-99, 100-999, ...)
                digits++; // Увеличиваем количество цифр в числе (1 -> 2 -> 3 ...)
            }

            // Шаг 2: Определяем, в каком числе находится искомая цифра
            int index = n % digits; // Порядковый номер цифры в числе
            if (index == 0)
                index = digits; // Если index == 0, значит цифра в конце числа

            // Определяем первое число в текущем диапазоне (10, 100, 1000...)
            long num = (long) Math.pow(10, digits - 1);

            // Определяем само число, в котором находится искомая цифра
            num += (index == digits) ? n / digits - 1 : n / digits;

            // Шаг 3: Определяем нужную цифру в числе
            for (int i = index; i < digits; i++) {
                num /= 10; // Убираем последние цифры, пока не дойдем до нужной
            }

            return (int) (num % 10); // Возвращаем последнюю оставшуюся цифру
        }
    }

    // https://leetcode.com/problems/split-array-largest-sum/description/
    static class Solution5 {
        Integer[][] memo = new Integer[1001][51];

        int getMinimumLargestSplitSum(int[] prefixSum, int currIndex, int subarrayCount) {
            int n = prefixSum.length - 1;
            if (memo[currIndex][subarrayCount] != null) {
                return memo[currIndex][subarrayCount];
            }

            if (subarrayCount == 1) {
                return memo[currIndex][subarrayCount] = prefixSum[n] - prefixSum[currIndex];
            }

            int minimalLargestSplitSum = Integer.MAX_VALUE;
            for (int i = currIndex; i <= n - subarrayCount; i++) {
                int firstSplitSum = prefixSum[i + 1] - prefixSum[currIndex];
                int largestSplitSum = Math.max(firstSplitSum,
                        getMinimumLargestSplitSum(prefixSum, i + 1, subarrayCount - 1));

                minimalLargestSplitSum = Math.min(minimalLargestSplitSum, largestSplitSum);
                if (firstSplitSum >= minimalLargestSplitSum) {
                    break;
                }
            }

            return memo[currIndex][subarrayCount] = minimalLargestSplitSum;
        }

        public int splitArray(int[] nums, int k) {
            int n = nums.length;
            int[] prefixSum = new int[n + 1];
            for (int i = 0; i < n; i++) {
                prefixSum[i + 1] = prefixSum[i] + nums[i];
            }

            return getMinimumLargestSplitSum(prefixSum, 0, k);
        }
    }
}
