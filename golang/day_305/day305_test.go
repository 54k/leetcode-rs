package day305

// https://leetcode.com/problems/candy/description/
func candy(ratings []int) int {
	candies := make([]int, len(ratings))
	for i := 0; i < len(candies); i++ {
		candies[i] = 1
	}

	for i := 1; i < len(candies); i++ {
		if ratings[i] > ratings[i-1] {
			candies[i] = candies[i-1] + 1
		}
	}

	for i := len(candies) - 2; i >= 0; i-- {
		if ratings[i] > ratings[i+1] {
			if candies[i] < candies[i+1]+1 {
				candies[i] = candies[i+1] + 1
			}
		}
	}

	sum := 0
	for i := 0; i < len(candies); i++ {
		sum += candies[i]
	}
	return sum
}
