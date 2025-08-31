import csv

infilename = "dummy.csv"
infile = open(infilename)

csvreader = csv.reader(infile)

for itemlist in csvreader:
    print("itemlist: " + str(itemlist))

infile.close()
