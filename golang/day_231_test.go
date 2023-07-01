package main

func distributeCookies(cookies []int, k int) int {
	distribute := make([]int, k)
	const MAX = 2_147_483_647
	const MIN = -2_147_483_648

	var dfs func(int, int) int
	dfs = func(i int, zero_count int) int {
		if len(cookies)-i < zero_count {
			return MAX
		}

		if i == len(cookies) {
			unfairness := MIN
			for _, val := range distribute {
				if unfairness < val {
					unfairness = val
				}
			}
			return unfairness
		}

		answer := MAX
		for j := 0; j < k; j++ {
			if distribute[j] == 0 {
				zero_count -= 1
			}
			distribute[j] += cookies[i]
			d := dfs(i+1, zero_count)
			if answer > d {
				answer = d
			}
			distribute[j] -= cookies[i]
			if distribute[j] == 0 {
				zero_count += 1
			}
		}

		return answer
	}

	return dfs(0, k)
}
