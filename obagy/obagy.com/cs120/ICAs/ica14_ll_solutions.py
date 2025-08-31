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

    # students will define this
    def incr(self):
        current = self._head
        while current != None:
            current._value = current._value + 1
            current = current._next

    # students will define this
    def replace(self, val1, val2):
        current = self._head
        while current != None:
            if current._value == val1:
                current._value = val2
            current = current._next

    # students will get this from the slides (slide 106)
    def add_to_end(self, new):

        # they have not been told to check for an empty
        # linked list, so they may not think of this!
        if self._head == None:
            self.add(new)
            return

        current = self._head
        prev = current
        while current != None:
            prev = current
            current = current._next
        prev._next = new

    # students will attemp to write this
    def remove_first(self):
        if self._head == None:
            return None
        else:
            n = self._head
            self._head = n._next
            # set n's next reference to None
            n._next = None  
            return n

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
    # make a linked list with three elements
    my_ll = LinkedList()
    my_ll.add(Node(2))
    my_ll.add(Node(4))
    my_ll.add(Node(6))

    # use the method print_elements() to print out the elements
    my_ll.print_elements()
    # use print() to print the linked list
    print(my_ll)

    # define incr(), call it, print the linked list
    my_ll.incr()
    print(my_ll)

    # define replace(), call it, print the linked list
    my_ll.replace(5,8)
    print(my_ll)


    # get add_to_end(self, new) from the slides (slide 118 or 119)
    # then make a node n and add it to the end of the linked list
    n = Node(50)
    my_ll.add_to_end(n)
    print(my_ll)


    # define remove_first(), call it, print the linked list
    my_ll.remove_first()
    print(my_ll)
   
    # tests for remove_first()
    print("more tests for remove_first()")
    blist = LinkedList()
    print(blist.remove_first())
    blist.add(Node(2))
    print(blist)
    print(blist.remove_first())
    print(blist)
    print()

    
main()
