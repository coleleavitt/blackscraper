
def compute_grades():

    grades = []
    num_grades = int(input('How many students? '))
    for i in range(0, num_grades+1):
        grade = int(input('Enter grade ' + str(i) + ': '))
        grades.append(grade)

    grades.sort()

    # Report the lowest grade
    lowest = grades[0]
    print('The lowest grade is ' + str(lowest))

    # Report the highest grade
    last_index = len(grades)-1
    highest = grades[last_index]
    print('The highest grade is ' + str(highest))

    # Report the median (middle) value
    mid_index = int( last_index / 2 )
    median = grades[mid_index]
    print('The median grade is ' + str(median))
    
    # How could we compute the average?

def main():
    compute_grades()

main()
