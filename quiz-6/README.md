# CSE 262 - Quiz 6

**Due: 10/14/2020 EOD**

Write your answers in this README file in the space below. Make at least one commit per question.

## Question 1

What is the relationship between LISP and Lambda Calculus?

The major relationship between LISP and lambda calculus is that LISP is an application of lambda calculus in the real world. LISP uses many of the core fundamentals of lambda calculus in order to create a functional programming language which can be used for many different tasks such as data anipulation and machine learning. Lambda calculus with its "function-based" style provides the perfect foundation for a functional language such as LISP.

## Question 2

What is the difference between a variable in a programming language like Java and a variable in Lambda Calculus?

There are a couple of important differences between a variable in a programming language like Java and in Lambda calculus. The first is that the character (or string in java) that acts as the variable name is irrelevant in Lambda calculus, where as it is the sole identifier in Java. In lambda calculus, depending on the expression, there can be multiple variables with the same name that represent different values. Unlike in Java, where variables must be initialized, variables in lambda calculus are arbitrary values that can be substituted for different values. 

## Question 3

What do we mean when we say Lambda Calculus is a "minimal" programming language? How can we call Lambda Calculus a programming language if it doesn't even include numbers?

What we mean when we say that lambda calculus is a "minimal" programming language is that lambda calculus represents the minimum grammar required in order to do things that you may do with a programming language. While lambda calculus does not have numbers, it is possible to define a function representing 0, as well as a successor function, allowing for any number to realistically be used by calling the successor function a number of times on the result of a previous successor function. The minimal grammar does not provide much functionality, but it provides enough flexibility to develop the functionality that we see in more typical programming languages such as control flow.