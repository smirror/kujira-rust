def encrypt(text, shift):
    code_a = ord("a")
    code_z = ord("z")

    result = ""
    for ch in text:
        code_ch = ord(ch)
        if code_a <= code_ch <= code_z:
            code_ch = (code_ch - code_a + shift) % 26 + code_a
            if code_ch > code_z:
                code_ch = code_ch - 26
        result += chr(code_ch)

    return result


enc = encrypt("hello, world!", 3)
dec = encrypt(enc, -3)
print(enc, "=>", dec)
