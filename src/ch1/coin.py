price = 3950
for i500 in range(0, 11):
    for i100 in range(0, 4):
        for i50 in range(0, 11):
            total = i500 * 500 + i100 * 100 + i50 * 50
            if price == total:
                print("500円: {}, 100円: {}, 50円: {}".format(i500, i100, i50))
