class LinkedList:
    def __init__(self):
        self._head = None

    def add(self,new):
        new._next = self._head
        self._head = new
        
    def print_elements(self):
        current = self._head
        while current != None:
            print(str(current._value))
            current = current._next

    # define this
    def incr(self):
        pass

    # define this
    def replace(self, val1, val2):
        pass

    # define this
    def add_to_end(self, new):
        pass

    # define this
    def remove_first(self):
        pass

    def __str__(self):
        string = 'LList -> '
        current = self._head
        while current != None:
            string += str(current)
            current = current._next
        return string
        

class Node:
    def __init__(self,value):
        self._value = value
        self._next = None

    def __str__(self):
        if self._next == None:
            nxt = "None"
        else:
            nxt = "->"
        return " |" + str(self._value) + "|:" + nxt


def main():
    # ICA-14 prob 2,a thru d. Do all of the following in main():
    #  a) make a linked list called my_ll and add three elements that are ints

    #  b) use the method print_elements() to print out linked list
    #     use my_ll.print_elements()

    #  c) now use print() to print your linked list
    #     remember that print() implicitly calls str() if it's defined
    #     use print(my_ll)
 
    #  d) take a pic of the output generated for this problem so far to use as the solution

    # ICA-14 prob 3
    #  Define incr(self) in the LinkedList class; this method increments all elements by 1
    #  a) call incr() on your linked list

    #  b) use print() to print the list and see the changes 

    #  c) take a pic of the output for the solution to this problem

    # ICA-14 prob 4
    #  Define replace(self, val1, val2) in the LinkdedList class; this method replaces of 
    #  the _value attributes that equal val1 with val2
    #  a) call replace() on your linked list


    #  b) use  print() to print the list and take a pic

    # ICA-14 prob 5
    #  Define add_to_end(self, new)  see slide 106
    #  a) create a new node n and call add_to_end() to add it to the end of the linked list

    #  b) use print() to print the linked list and take a pic

   
    # ICA-14 prob 6 
    # (Challenge) Define remove_first()
    # a) call remove_first() on your linked list

    # b) use print() to show how the linked list changed
   
    # make sure to cover all test cases:
    #    an empty list
    #    a list of one element
    #    a list of many elements
    print("more tests for remove_first()")
    
main()
