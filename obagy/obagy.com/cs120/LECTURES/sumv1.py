import time

"""
sumv1 - computes the sum of the first n integers using a loop
"""
def sumv1(n):
    start = time.time()    #returns the clock time in seconds

    num = 0
    for i in range(1,n+1):
        num += i

    end = time.time()

    taken = end - start

    return "sum = %d : running time required was \t %10.7f seconds"%(num, taken)


def main():
    print(sumv1(10000))
    # add calls for 100,000 and 1,000,000
main()
