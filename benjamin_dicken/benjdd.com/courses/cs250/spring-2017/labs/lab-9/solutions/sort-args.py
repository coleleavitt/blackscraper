import sys
args = []
index = 1
while index < len(sys.argv):
    args.append(sys.argv[index])
    index += 1
args.sort()
for arg in args:
    print(arg)

