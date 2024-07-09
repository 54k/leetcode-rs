start1 = input() 
end1 = input() 

start2 = input() 
end2 = input() 

def add(cell, move): 
    return chr(ord(cell[0]) + move[0]) + chr(ord(cell[1]) + move[1]) 

def correct(cell): 
    return "a" <= cell[0] <= "h" and "1" <= cell[1] <= "8" 

Moves = [[1, 2], [1, -2], [-1, 2], [-1, -2], [2, 1], [2, -1], [-2, 1], [-2, -1]] 

D = dict() 
P = dict() 

for x in "abcdefgh": 
    for y in "12345678": 
        for i in "abcdefgh": 
            for j in "12345678": 
                D[(x + y, i + j)] = -1 
                P[(x + y, i + j)] = None 

D[(start1,start2)] = 0 
Q = [(start1, start2)] 
Qstart = 0 

while Qstart < len(Q): 
    u1, u2 = Q[Qstart] 
    Qstart += 1 

    moves = []
    for m in Moves:
        v = add(u1, m) 
        if v != u2 and correct(v):
            moves.append((v, u2))

    for m in Moves:
        v = add(u2, m) 
        if v != u1 and correct(v):
            moves.append((u1, v))

    for move in moves: 
        if D[move] == -1: 
            D[move] = D[(u1, u2)] + 1 
            P[move] = (u1, u2)
            Q.append(move) 

Ans = [] 
curr = (end1, end2) 
while curr is not None: 
    nxt = P[curr] 
    if nxt is None:
        break

    if nxt[1] == curr[1]:
        Ans.append(f"1 {curr[0]}") 
    elif nxt[0] == curr[0]:
        Ans.append(f"2 {curr[1]}") 
        
    curr = nxt

print("\n".join(Ans[::-1]))
