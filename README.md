# Rlox-AST
A Rust implementation for the interpreted language jlox from the book Crafting Interpreters by Robert Nystorm.
My implementation for the Lox language.
For AST Nodes I used dynamic dispatch which later on caused a lot of problems and slowed down the overall performance of the language.
Unlike in the book the `Expr` and `Stmt` Interface (Trait in Rust's case) we cannot return a generic. 
This is because it's not possible to make a VTable when making use of generics.

I got stuck a couple times, fortunately I found this video series from ![UncleScientist](https://github.com/UncleScientist): ![Playlist](https://www.youtube.com/watch?v=WdoAJ_ouWRM&list=PLib6-zlkjfXluRjBgK8grQH2IUSZjn-YN)

### How to run.
Run the binary `./rlox` without any arguments to use the repl.
if you add a file name as an argument it will run the script.

`./rlox my.program.lox`

### Differences from the default implementation.
The `print` keyword does not create a newline.
To do a normal print line the native function `println()` can be used.

### Repl
The Repl has a couple extra features.
Primarily usage of the arrow keys in the shell.
Arrow up and down to navigate through lines of code executed prior. Arrows left and right to navigate the cursor.
There are also a couple of commands to clear the screen or reset the input buffer.
![](https://i.imgur.com/Ji8dCE9.png)


### Examples:
```js
// OOP features.
class Person {
    init(name, age, address) {
        this.name = name;
        this.age = age;
    }
    getName() {
        return this.name;
    }
    getAge() {
        return this.age;
    }
    setName(name) {
        this.name = name;
    }
    setAge(age) {
        this.age = age;
    }
    toString() {
        return this.name + " is " + this.age +  " years old and lives in " + this.address;
    }
}
var person = Person("John", 30, "London");
person.setAge(31);
println(person.toString());
// Functions
fun addTwo(a, b) {
  // If one of these values is not a number it just turns into a concatinated string.
  return a + b;
}

fun recursion(a) {
  if (a > 10) return a;
  return recursion(a+1);
}

// For loops
for (var i = 0; i < 10; i = i +1) {
  println(i);
}
```
