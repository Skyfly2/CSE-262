# CSE 262 - Quiz 8

**Due: 11/04/2020 EOD**

Write your answers in this README file in the space below. Make at least one commit per question.

## Question 1

What is a Horn clause, and how does this term relate to Prolog?

A horn clause could be described as a formula or logical definition describing the properties of a statement. It relates to prolog by the fact that horn clauses are used within prolog in order to develop rules and build relationships between facts and objects in a prolog database. Because of this, horn clauses are key components of many logic programming languages, including Prolog.

## Question 2

If all men are mortal, and Socrates is a man, what conclusions can we draw about Socrates? In order to answer this question, you need to perform a logical procedure which is the basis for the execution of Prolog programs. What is that procedure called?

If all men are mortal, and Socrates is a man, we can conclude that Socrates is mortal. The process of performing the logical procedure in order to compute this answer is called resolution.

## Question 3

Prolog databases are composed of two things: rules and facts. What's is a rule, and what is a fact, and how do they relate?

A rule in prolog is pretty much the application of a horn clause in prolog, in that it has a head and a body, where the body defines the statements that must be true in order to ev aluate to the head value. Rules are often used in prolog with facts making up the head and the body of the statement. A fact in prolog is essentially just a statement that can introduce something new into the program or be based off of a relationship of objects. Facts are also stored in the database and can be utilized later within a prolog program including within rules that are written.