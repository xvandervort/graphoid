"""Class-Like Graphs — Python equivalent of classes_vs_clg.gr

Python uses traditional classes with __init__, @property, inheritance.
Graphoid uses graph declarations with configure directives.
"""

# --- Basic class with properties ---
class Person:
    def __init__(self, name="Unknown", age=0):
        self._name = name
        self._age = age

    @property
    def name(self):
        return self._name

    @property
    def age(self):
        return self._age

    @age.setter
    def age(self, value):
        self._age = value

    def greet(self):
        return f"Hi, I'm {self._name}"

alice = Person(name="Alice", age=30)
print(alice.greet())  # "Hi, I'm Alice"
print(alice.name)     # "Alice"
alice.age = 31
print(alice.age)      # 31

# --- Inheritance ---
class Animal:
    def __init__(self, name="unknown", sound="..."):
        self.name = name
        self.sound = sound

    def speak(self):
        return f"{self.name} says {self.sound}"

class Dog(Animal):
    def __init__(self, name="unknown", breed="unknown"):
        super().__init__(name, "woof!")
        self.breed = breed

    def fetch(self):
        return f"{self.name} fetches!"

fido = Dog(name="Fido", breed="Labrador")
print(fido.speak())   # "Fido says woof!"
print(fido.fetch())   # "Fido fetches!"

# --- Validation in methods ---
class BankAccount:
    def __init__(self, balance=0):
        self._balance = balance

    @property
    def balance(self):
        return self._balance

    def deposit(self, amount):
        if amount <= 0:
            raise ValueError("Deposit must be positive")
        self._balance += amount

    def withdraw(self, amount):
        if amount > self._balance:
            raise ValueError("Insufficient funds")
        self._balance -= amount

acct = BankAccount(balance=1000)
acct.deposit(500)
print(f"Balance: {acct.balance}")  # 1500
acct.withdraw(200)
print(f"Balance: {acct.balance}")  # 1300

# Python requires:
# - __init__ boilerplate for every class
# - @property decorator for getters
# - @x.setter decorator for setters
# - Explicit self in every method
# - Manual super().__init__() calls

print("Classes demo complete!")
