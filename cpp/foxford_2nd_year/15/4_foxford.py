n, k, p = map(int, input().split())
 L = [[] for i in range(k + 1)]
 f = False
 ans = 0
 max_ans = 0
 for i in range(n):
     operation, otsek, toplivo = input().split()
     otsek, toplivo = int(otsek), int(toplivo)
     if operation == '+':
         if ans == p:
             f = True
         L[otsek].append(toplivo)
         ans += 1
         if max_ans < ans:
             max_ans = ans
     else:
         if L[otsek] == []:
             f = True
         elif L[otsek][-1] != toplivo:
             f = True
         else:
             L[otsek].pop()
             ans -= 1
 if ans != 0:
     f = True
 print('Error' if f else max_ans)