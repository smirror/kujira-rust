a, b = 1, 1
print(a)
print(b)
for _ in range(30):
    print(a + b)
    a, b = b, a + b
