import sys
args = []
index = 2

while index < len(sys.argv):
    args.append(sys.argv[index])
    index += 1

if sys.argv[1] == 'ascending':
    args.sort()
    args.reverse()
elif sys.argv[1] == 'descending':
    args.sort()

for arg in args:
    print(arg)

