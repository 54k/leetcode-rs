package day305

import (
	"fmt"
	"testing"
)

// https://leetcode.com/problems/candy/description/
func candyTwoPasses(ratings []int) int {
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

func candySinglePassConstantSpace(ratings []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	count := func(n int) int {
		return (n * (n + 1)) / 2
	}

	if len(ratings) <= 1 {
		return len(ratings)
	}

	candies := 0
	up := 0
	down := 0
	oldSlope := 0

	for i := 1; i < len(ratings); i++ {
		newSlope := 0
		if ratings[i] > ratings[i-1] {
			newSlope++
		} else if ratings[i] < ratings[i-1] {
			newSlope--
		}

		if oldSlope > 0 && newSlope == 0 || oldSlope < 0 && newSlope >= 0 {
			candies += count(up) + count(down) + max(up, down)
			up, down = 0, 0
		}

		if newSlope > 0 {
			up++
		} else if newSlope < 0 {
			down++
		} else {
			candies++
		}

		oldSlope = newSlope
	}

	candies += count(up) + count(down) + max(up, down) + 1
	return candies
}

func TestCandy(t *testing.T) {
	res := candySinglePassConstantSpace([]int{1, 2, 3, 4, 5, 3, 2, 1, 2, 6, 5, 4, 3, 3, 2, 1, 1, 3, 3, 3, 4, 2})
	fmt.Println(res)
}
