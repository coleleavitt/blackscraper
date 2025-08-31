initial = 100
other = 5

def do_stuff_1(p1):
    new = p1 * 2 / 10
    if (new > 100):
        new = new + 100
    return int(new)

def do_stuff_2(p1):
    x = p1 + p1 + 10
    y = x * 2 + 4
    z = x + y * 2 - 100
    ret_val = x + y + z
    return ret_val

def main():
    r1 = do_stuff_1(initial + 1)
    r2 = do_stuff_2(r1)
    print("The result is: " + str(r1+r2))

main()

