# CSE 262 - Quiz 9

**Due: 11/10/2020 EOD**

Write your answers in this README file in the space below. Make at least one commit per question.

## Question 1

What is first-order predicate calculus? How does it relate to Prolog?

First-order predicate calculus is a set of rules for for the resolution of propositions, which contain predicate operators and quantifiers. A predicate is a function mapping atoms to true and false. Essentially, we can use first-order predicate calculus in order to evaluate and solve specific propositions. This is a very important concept in terms of prolog, because by using the rules of first-order predicate calculus, mathematical propositions can be rewritten and interpreted in such a way that rules and horn-clauses can be written based off of the proposition in a Prolog program for evaluation. These logic rules coming from first-order predicate calculus form the basis for the "logic" behind a logical programming language such as prolog.

## Question 2

Why is it important to pay attention to the order of terms in a Prolog clause?

There are a couple of reasons why it is important to pay attention to the order of terms in a Prolog clause. The major reason has to do with the termination of a program. When a prolog program evaluates a clause, it has to evaluate each part of the clause in order. Because of this, depending on the clause and what it contains, it is possible that a certain part of the clause may rely on data that will be put into the database by another part of the same clause, and would otherwise not be able to be resolved. In this event, it is possible that an infinite loop could occur with the ecpression that can not be resolved, which would eventually lead to a stack overflow error as new stack-frames are added to the stack in an attempt to evaluate what can not be resolved. This is the major reason why it is important to pay attention to the order of terms in a Prolog clause.

## Question 3

Prolog is homoiconic. What does this tell us about the design of the Prolog runtime?

Since Prolog is a homoiconic language, it means that it treats all of the code of a program as data, and it can be accessed as data by the language. This makes sense, since prolog programs are essentially just querying a database of facts and rules that has been developed. What this tells us about the runtime of prolog, is that because all of the code can be accessed by the queries and the rules and facts are treated as data, it means that the runtime must be designed so that everything is evaluated during the running-time of the program, and the runtime is able to interpret and access the data exactly as it is written in the source code.