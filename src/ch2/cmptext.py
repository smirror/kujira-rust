afile = "./fizzbuzz_py.txt"
bfile = "./fizzbuzz_rs.txt"

with open(afile, "r") as fp:
    astr = fp.read()
with open(bfile, "r") as fp:
    bstr = fp.read()

astr = astr.strip()
bstr = bstr.strip()

if astr == bstr:
    print("OK")
else:
    print("NG")
