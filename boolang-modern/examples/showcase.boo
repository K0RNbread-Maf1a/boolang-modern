namespace Examples

import System
import System.Collections.Generic

# This is an example showcasing BooLang Modern features

# Class with properties and methods
class Person:
    # Fields with type annotations
    private _name: string
    private _age: int
    
    # Property with getter/setter
    Name as string:
        get:
            return _name
        set:
            _name = value
    
    # Constructor
    def constructor(name as string, age as int):
        _name = name
        _age = age
    
    # Method with return type
    def Greet() as string:
        return "Hello, I'm ${_name} and I'm ${_age} years old"
    
    # Method with default parameter
    def Celebrate(message as string = "Happy Birthday!"):
        _age += 1
        print "${message} Now I'm ${_age}!"

# Inheritance example
class Employee(Person):
    private _company: string
    
    def constructor(name as string, age as int, company as string):
        super(name, age)
        _company = company
    
    # Override method
    override def Greet() as string:
        return super.Greet() + " and I work at ${_company}"

# Generic class
class Container[T]:
    private _items: List[T] = List[T]()
    
    def Add(item as T):
        _items.Add(item)
    
    def Get(index as int) as T:
        return _items[index]
    
    def Count as int:
        get:
            return _items.Count

# Interface definition
interface IDrawable:
    def Draw():
        pass

# Struct for value types
struct Point:
    X as int
    Y as int
    
    def constructor(x as int, y as int):
        X = x
        Y = y
    
    def Distance(other as Point) as double:
        dx = X - other.X
        dy = Y - other.Y
        return Math.Sqrt(dx * dx + dy * dy)

# Enum
enum Color:
    Red = 1
    Green = 2
    Blue = 3

# Main entry point
class Program:
    static def Main():
        # Variable declarations with type inference
        person = Person("Alice", 30)
        print person.Greet()
        
        # Type annotation
        employee: Employee = Employee("Bob", 25, "TechCorp")
        print employee.Greet()
        
        # Collections with generics
        numbers = Container[int]()
        numbers.Add(1)
        numbers.Add(2)
        numbers.Add(3)
        
        # For loop
        for i in range(numbers.Count):
            print "Number ${i}: ${numbers.Get(i)}"
        
        # Control flow
        if numbers.Count > 0:
            print "Container has items"
        elif numbers.Count == 0:
            print "Container is empty"
        else:
            print "This shouldn't happen"
        
        # While loop
        counter = 0
        while counter < 3:
            print "Counter: ${counter}"
            counter += 1
        
        # Lambda expressions
        double = lambda (x): x * 2
        print "Double of 5: ${double(5)}"
        
        # Array literal
        colors = [Color.Red, Color.Green, Color.Blue]
        
        # Hash literal (dictionary)
        ages = {"Alice": 30, "Bob": 25, "Charlie": 35}
        
        # Try-except-finally
        try:
            result = 10 / 0
        except e as DivideByZeroException:
            print "Cannot divide by zero!"
        finally:
            print "Cleanup complete"
        
        # Struct usage
        p1 = Point(0, 0)
        p2 = Point(3, 4)
        distance = p1.Distance(p2)
        print "Distance: ${distance}"
        
        # Pattern matching (ternary)
        status = "active" if employee.Name.Length > 0 else "inactive"
        print "Status: ${status}"

# Async method example
class AsyncExample:
    async def FetchDataAsync() as Task[string]:
        # Simulate async operation
        await Task.Delay(1000)
        return "Data fetched!"
    
    static async def Main():
        example = AsyncExample()
        data = await example.FetchDataAsync()
        print data

# Extension methods example
[Extension]
static class StringExtensions:
    [Extension]
    static def Reverse(self as string) as string:
        chars = self.ToCharArray()
        Array.Reverse(chars)
        return string(chars)

# Using extension method
class ExtensionDemo:
    static def Main():
        text = "Hello"
        reversed = text.Reverse()
        print reversed  # Prints "olleH"
