# CSE 262 - Quiz 2

**Due: 9/5/2020 EOD**

Write your answers in this README file in the space below. Make at least one commit per question.

## Question 1

**What is a stack vs. a heap?**

The stack and heap are different sections of memory that have some key differences to complement each other. The stack is a contiguous piece of memory of fixed size which is typically
relatively small but very fast to access. As opposed to this, the heap is a generally much larger chunk of memory that has the ability to grow in size. However, due to its much larger size
and growability, it is slower to access the memory on the heap than it is on the stack. The stack also removes items no longer in use from itself, where as it is the programmer's job to 
clear memory from the heap unless they are using a language which features a garbage collector (such as Java).

## Question 2

**What is an enum vs. a struct?**

There are a couple of differences between an enum and a struct. A struct is similar to an object and acts as a collection of data values of which individual instances can be created and modified.
On the other hand, and enum acts as a collection of data values that act more like constants and as such, you cannot create instances of an enum. Enums allow for the data values to be stored more efficiently
in memory as opposed to just using constants.

## Question 3

**Describe the following Rust types:**

- Rc

An Rc is a reference counter, which essentially allows for two different names to have shared ownership over a specific value. For example, if there is a linkedlist of size one, the head and tail would both share
ownership of that node.

- Box

A box is a "data owner" that allows for immutable memory allocation on the heap allocation of a data value or collection of data values (like a struct). 

- Cell

A cell is similar to a box because it allows for memory allocation on the heap of a data value or collection of data values (like a struct), however, Cell's allow their memory to be mutable, so the values can be changed.
