import sys
word = sys.argv[1]
reps = int(sys.argv[2])
case = sys.argv[3]

if case == 'upper':
    word = word.upper()
elif case == 'lower':
    word = word.lower()

print((word + ' ') * reps)

