# CSE 262 - Quiz 4

**Due: 9/21/2020 EOD**

Write your answers in this README file in the space below. Make at least one commit per question.

## Question 1

List 3 ways to destructure a `std::Result<T,E>` in Rust (i.e. how do you get at the `T` or `E` inside of the `Result<T,E>`?).


1.) One way that you can destructure a `std::Result<T,E>` is to use the unwrap() method, which returns the T within the "OK(T)" if it has a value and if not panics and displays the E within "Err(E)" on the console. You can also use the expect() method to display exactly what you want on the console.


2.) Another way that you can destructure a `std::Result<T,E>` is to use a match statement on the `std::Result<T,E>` and create an arm for an "Ok(T)" and an arm for an "Err(E)" and the functions that these arms point to can use T or E in any way that they want.

3.) Finally, you can use the "?" operator in order to automatically unwrap the "Ok(T)" if T is set, or return error E out of "Err(E)" if the value of "Err" is set.

## Question 2

What is the purpose of the terminating character in an EBNF grammar?


In EBNF, the terminating character (";") represents the end of the definition of one of the rules in the grammar. This way, the parser reading the grammar knows when to expect there to be nothing else as a portion of a specific rule and can therefore move on from the rule in the parsing sequence.

## Question 3

What is a recursive descent parser and how does it work?


A recursive descent parser is a type of parser that works as a top-down parser within the program, using the grammar to call lower-level rules in order to build the parse tree.
Essentially, this causes the program to call a defined protion (rule) of grammar which then calls a defined portion of the grammar, and so on, until the specific value of the current token matches
the type non-recursively. After doing this, the parser backtracks to continue recursive calls back at a higher level. The parse tree that results from this kind of parse can be rebuilt into the source code
by simply running  adepth-first search through the parse tree.