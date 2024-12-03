def is_digit(c):
    return '0' <= c <= '9'

with open('input') as f:
    input = f.read()
    ans = 0
    do = True

    for i in range(len(input)):
        if input[i:i + 4] == "do()":
            do = True
        elif input[i:i + 7] == "don't()":
            do = False
        elif do and input[i:i + 4] == "mul(":
            i += 4
            j = i
            while j < len(input) and is_digit(input[j]):
                j += 1

            if j == len(input) or input[j] != ',':
                continue

            a = int(input[i:j])

            j += 1
            i = j
            while j < len(input) and is_digit(input[j]):
                j += 1
            
            if j == len(input) or input[j] != ')':
                continue

            b = int(input[i:j])
            ans += a * b
    
    print(ans)
