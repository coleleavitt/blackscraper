import sys
print('Script name:    ' + sys.argv[0])
print('Argument count: ' + str(len(sys.argv) - 1))
print('First argument: ' + sys.argv[1])
print('Last argument:  ' + sys.argv[len(sys.argv)-1])

