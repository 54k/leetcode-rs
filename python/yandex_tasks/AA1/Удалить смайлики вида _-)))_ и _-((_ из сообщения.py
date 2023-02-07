from pprint import pprint


def remove_smiles(inp):
    i = 0
    result = []
    while i < len(inp):
        if i + 2 < len(inp) and inp[i] == ':' and inp[i + 1] == '-':
            if inp[i + 2] == ')':
                i += 3
                while i < len(inp):
                    if inp[i] == ')':
                        i += 1
                        continue
                    else:
                        result.append(inp[i])
                        break
            elif inp[i + 2] == '(':
                i += 3
                while i < len(inp):
                    if inp[i] == '(':
                        i += 1
                        continue
                    else:
                        result.append(inp[i])
                        break
            else:
                result.append(inp[i])
        else:
            result.append(inp[i])
        i += 1
    return ''.join(result)


pprint(remove_smiles("Ааааа!!!!! :-))(())"))
# how to be noob, oops, wrong result
pprint(remove_smiles(":-)):-(("))
