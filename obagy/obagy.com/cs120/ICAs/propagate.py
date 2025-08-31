
def fun1(x):
    return 1/x

def fun2(x):
    return 1 + fun1(x)

def main():

    z = fun2(3)
    print(z)
    z = fun2(0)
    print(z)

main()
