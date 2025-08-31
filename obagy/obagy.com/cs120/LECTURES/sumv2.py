import time

"""
sumv2 - computes the sum of the first n integers using a closed equation
        
"""
def sumv2(n):
    start = time.time()    #returns the clock time in seconds

    num = (n*(n+1))/2

    end = time.time()

    taken = end - start

    return "sum = %d : running time required was \t %10.7f seconds"%(num, taken)

def main():
    print(sumv2(10000))
    # add calls for 100,000 and 1,000,000

main()
