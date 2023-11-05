package day358

// https://leetcode.com/problems/climbing-stairs/description/
func climbStairsBottomUp(n int) int {
	a, b := 0, 1
	for i := 2; i <= n+1; i++ {
		c := a + b
		a, b = b, c
	}
	return b
}

func climbStairsBinets(n int) int {
	q := [][]int{{1, 1}, {1, 0}}

	multiply := func(a, b [][]int) [][]int {
		c := [][]int{{0, 0}, {0, 0}}
		for i := 0; i < 2; i++ {
			for j := 0; j < 2; j++ {
				c[i][j] = a[i][0]*b[0][j] + a[i][1]*b[1][j]
			}
		}
		return c
	}

	pow := func() [][]int {
		ret := [][]int{{1, 0}, {0, 1}}
		for n > 0 {
			if n&1 == 1 {
				ret = multiply(ret, q)
			}
			n >>= 1
			q = multiply(q, q)
		}
		return ret
	}

	res := pow()
	return res[0][0]
}

// https://leetcode.com/problems/fibonacci-number/description
func fib(n int) int {
	if n <= 1 {
		return n
	}
	a, b := 0, 1
	for i := 2; i <= n; i++ {
		c := a + b
		a, b = b, c
	}
	return b
}
