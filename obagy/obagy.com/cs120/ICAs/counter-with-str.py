class Counter:
    def __init__(self, name):
        self._name = name
        self._count = 0

    def click(self):
        self._count += 1

    def count(self):
        return self._count

    def __str__(self):
        return "Counter: " + self._name + "->" + str(self._count)
