# This is so fucking hacky, I'm almost certain there's a better way of doing it

moves = {}


edges = [list(range(8)), list(range(56, 64)), [8 * n + 7 for n in range(8)], [8 * n for n in range(8)]]

for i in range(64): 
    if i == 0: 
        moves[i] = [9 * i for i in range(8)]
    elif i == 7: 
        moves[i] = [7 * n + 7 for n in range(1, 8)]
    elif i == 56: 
        moves[i] = list(range(49, 0, -7))
    elif i == 63:
        moves[i] = list(range(54, -1, -9))
    elif i in edges[0]:
        row = [] 
        foo = i 
        while foo not in edges[3]: 
            foo += 7 
            row.append(foo)
        foo = i 
        while foo not in edges[2]:
            foo += 9 
            row.append(foo)
            

        moves[i] = row
    elif i in edges[1]: 
        row = [] 
        foo = i 
        
        while foo not in edges[3]: 
            foo -= 9 
            row.append(foo)

        foo = i 
        while foo not in edges[2]: 
            foo -= 7 
            row.append(foo)
        moves[i] = row
    elif i in edges[2]:
        row = [] 
        foo = i 
        
        while foo not in edges[0]:
            foo -= 9 
            row.append(foo)
        foo = i 
        while foo not in edges[1]:
            foo += 7 
            row.append(foo)

        moves[i] = row
    elif i in edges[3]:
        row = []
        foo = i 
        
        while foo not in edges[0]:
            foo -= 7 
            row.append(foo)
        foo = i 
        while foo not in edges[1]:
            foo += 9 
            row.append(foo)

        moves[i] = row
    else: 
        row = [] 
        foo = i 
        
        while foo not in edges[0]and foo > 0 and foo < 63: 
            foo -= 9 
            row.append(foo)

        foo = i 
        
        while foo not in edges[1]and foo > 0 and foo < 63: 
            foo += 9 
            row.append(foo)

        foo = i 
        
        while foo not in edges[2]and foo > 0 and foo < 63: 
            foo -= 7 
            row.append(foo)

        foo = i 

        while foo not in edges[3] and foo > 0 and foo < 63: 
            foo += 7 
            row.append(foo)

        moves[i] = row
            
print(edges)
for idx in moves: 
    print(f"{idx}: {list(filter(lambda n: n >= 0, moves[idx]))}") 