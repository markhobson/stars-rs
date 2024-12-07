for size in range(1, 10):
    for y in range(0, size):
        d = -1 if y < size / 2 else 0
        x0 = abs(int((size + d) / 2) - y)
        x1 = size - x0

        for x in range(0, x0):
            print(".", end="")

        for x in range(x0, x1):
            print("#", end="")

        print()

    print()
