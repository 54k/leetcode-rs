def memoize(f):
     memo = {}
     def helper(x):
         if x not in memo:            
             memo[x] = f(x)
         return memo[x]
     return helper
  
 @memoize
 def f(n):
      if n <= 2:
          return 1
      elif n % 2 == 1:
          return f(6 * n // 7) + f(2 * n // 3)
      else:
          return f(n - 1) + f(n - 3)
  
 print(f(int(input())) % 2**32)