def encrypt(text, shift):
    a = ord("a")
    conv = lambda n: chr((ord(n) - a + shift) % 26 + a)
    enc1 = lambda n: conv(n) if "a" <= n <= "z" else n
    return "".join(map(enc1, text))


enc = encrypt("hello, world!", 3)
dec = encrypt(enc, -3)
print(enc, "=>", dec)
