for j in range(1, 10):
    a = ["{:3}".format(i * j) for i in range(1, 10)]
    print(",".join(a))
