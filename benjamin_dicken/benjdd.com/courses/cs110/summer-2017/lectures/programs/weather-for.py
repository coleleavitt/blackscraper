
def main():
    days = int(input('How many days\' temperatures? '))
    total_temps = 0
    all_temps = []
    for i in range(1, days+1):
        temp = float(input("Day " + str(i) + "'s high temp: "))
        total_temps += temp
        all_temps.append(temp)
    
    all_temps.sort()
    ll = len(all_temps) - 1
    middle_index = int(ll // 2)
    median = all_temps[middle_index]
    print('Median temp = ' + str(median))
    
    avg = total_temps / days
    print('Average temp = ' + str(avg))
    
    above_avg_count = 0
    for i in all_temps:
        if (i > avg):
            above_avg_count += 1
    print(str(above_avg_count) + ' days were above average')

main()
