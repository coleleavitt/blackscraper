class LinkedList:
    def __init__(self):
        self._head = None

    def add(self,new):
        new._next = self._head
        self._head = new

    def remove_value(self, value):
        if self._head == None:
            return None
        current = self._head

        #remove first node if its value 
        #attribute equals value
        if current._value == value:
            self._head = current._next
            current._next = None
            return

        prev = current
        current = current._next
        while current != None:
            if current._value == value:
                prev._next = current._next
                current._next = None
                return
            prev = current
            current = current._next 
            
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
    # make a linked list with four elements
    my_ll = LinkedList()
    my_ll.add(Node(2))
    my_ll.add(Node(4))
    my_ll.add(Node(4))
    my_ll.add(Node(6))

    # use print() to print the linked list
    print(my_ll)

    my_ll.remove_value(4)
    print("removed 4")
    print(my_ll)
    print("removed non-existant value")
    my_ll.remove_value(200)
    print(my_ll)
    my_ll.remove_value(6)
    print("removed 6")
    print(my_ll)
    my_ll.remove_value(2)
    print("removed 2")
    print(my_ll)

    ll2 = LinkedList()
    print(ll2)
    print("remove from empty list ")
    ll2.remove_value(200)
    print("now add 4")
    ll2.add(Node(4))
    print(ll2)
    print("now remove 4")
    ll2.remove_value(4)
    print(ll2)

main()
