@ctci07 this is a really useful technique to map a given 'state' to a unique code. This save us from have to set up a multi-dimensional dp (in this case - we are tracking 4 parameters - idx, lastChar, lastCharCount, k), and can have a 1D dp which simplifies the code.

the idea is:
0 <= lastChar - 'a' < 27
0 <= lastCharCount < 101
0 <= k < 101

hence idx * 27 * 101 *101 + (lastChar - 'a') * (101 * 101) + lastCharCount * 101 + k

gives a unique.

to simplify,
suppose you have an M x N matrix,
0 <= i < M, 0 <= j < N

in this case, a unique code for every cell (i, j) in the matrix would be = i * N + j
(OR even j * M + i)