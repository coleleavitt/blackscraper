import time

# l is the list to search in
# x is what to search for
def sequential_search(l, x):
    for elem in l:
        if elem == x:
            return True
    return False

# l is the list to search in
# x is what to search for
def binary_search(l, x):
    sl = l
    first = 0
    last = len(sl)-1
    while first <= last:
        check = (first + last) // 2
        if sl[check] == x:
            return True
        elif sl[check] < x:
            first = check+1
        else:
            last = check-1
    return False

start = time.time()
items = [10] * 1000000000
items.append(43)
end = time.time()
print('Creating the list took ' + str(round(end - start, 3)) + 's')

start = time.time()
res = binary_search(items, 43)
end = time.time()
print('Binary search took ' + str(round(end - start, 3)) + 's')

start = time.time()
res = sequential_search(items, 43)
end = time.time()
print('Sequential search took ' + str(round(end - start, 3)) + 's')
