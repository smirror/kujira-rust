import random

nums = list(range(1, 76))

random.shuffle(nums)
nums[12] = "*"

for y in range(5):
    for x in range(5):
        print("{:>3},".format(nums[y * 5 + x]), end="")
    print("")
