
def main():
    days = int(input('How many days\' temperatures? '))
    i = 1
    total_temps = 0
    all_temps = []
    while i <= days:
        temp = float(input("Day " + str(i) + "'s high temp: "))
        total_temps += temp
        all_temps.append(temp)
        i += 1
    
    all_temps.sort()
    print(all_temps)
    ll = len(all_temps) - 1
    middle_index = int(ll // 2)
    median = all_temps[middle_index]
    print('Median temp = ' + str(median))
    
    avg = total_temps / days
    print('Average temp = ' + str(avg))
    
    above_avg_count = 0
    i = 0
    while i < len(all_temps):
        if (all_temps[i] > avg):
            above_avg_count += 1
        i += 1
    print(str(above_avg_count) + ' days were above average')

main()
