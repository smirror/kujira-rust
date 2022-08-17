for y in range(1926, 2027):
    print("西暦{}年 = ".format(y), end="")
    if y >= 2019:
        if y == 2019:
            print("令和元年")
        else:
            print("令和{}年".format(y - 2019 + 1))
    elif y >= 1989:
        if y == 1989:
            print("平成元年")
        else:
            print("平成{}年".format(y - 1989 + 1))
    elif y >= 1926:
        if y == 1926:
            print("昭和元年")
        else:
            print("昭和{}年".format(y - 1926 + 1))
