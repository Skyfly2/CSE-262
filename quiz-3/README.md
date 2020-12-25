# CSE 262 - Quiz 2

**Due: 9/13/2020 EOD**

Write your answers in this README file in the space below. Make at least one commit per question.

## Question 1

**What is a `Result<T,E>` type?**

The Result<T, E> type is an enum in rust's standard library which is designed to be used for error handling, and is generally returned when an error is expected or able to be handled. One of the useful purposes that they
serve over just returning values that would indicate an error is the fact that the compiler requires them to be properly handled. It can also be used in place of Option<T> for error handling.

## Question 2

**What is the relationship between a language grammar and a parser?**

There is a strong relationship between the grammar of a language and the parser. This is because the grammar of the language is how the program is written, and how what is written is interpreted. Because of this,
using the grammar allows for a program to develop the structure of how it runs. The parser itself takes the stream of characters provided by the lexer and produces a parse tree out of that information which provides 
structure to the program, and allows the compiler to determine how it will be interpreted. Essentially, the grammar of the language provides the blueprint for the parser on how to build the parse tree out of the given
stream of characters.

## Question 3

**The contents of a file are printed below:**

```
[102, 110, 32, 109, 97, 105, 110, 40, 41, 32, 123, 10, 32, 32, 112, 114, 105, 110, 116, 108, 110, 33, 40, 34, 67, 83, 69, 50, 54, 50, 33, 34, 41, 59, 10, 125]
```

**Another similar but different file:**

```
[102, 110, 32, 109, 97, 105, 110, 40, 41, 123, 112, 114, 105, 110, 116, 108, 110, 33, 40, 34, 67, 83, 69, 50, 54, 50, 33, 34, 41, 59, 125]
```

**Both files are fed into a compiler, resulting in the same abstract syntax tree for each file. Why might that be the case? What does it mean if the abstract syntax trees for a programs produced by two different source files is the same?**

It is possible that the reason the two different programs produce the same abstract syntax tree while being slightly different is because of a difference in how the text version of the program has grouped everything.
variables and evaluations could be grouped or declared in such a way that the text version of the program appears to be slightly different, however, the actual operations and comparisons being computed are still happening
the same way and in the same order. If this is the case, the abstract syntax trees will be the same because the same operations and evalutations will be occurring (which is what shows up in the AST). If two source files produce
the same AST, it means that those two source programs do the same thing because they complete the same tasks. 