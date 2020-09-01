# CSE 262 - Quiz 1

Due: 8/28/2020 End of Day

Make at least one commit per question. You can write your answers in the README.md file. You can use the Gitlab interface to edit this file if you prefer.

## Question 1

What is a version control system? Why are they useful? What is git? What is gitlab?

A version control system is a program that allows for collaboration in the development of a codebase while also tracking how much each user has contributed to the codebase and tracks the progress of development. These systems are useful because
by allowing for multiple people to work on the same code repository, it makes it so that different people can split up different tasks when it comes to writing code. It is also useful to see who has contributed and how much they have
contributed. Git is probably the most popular version control system and is generally considered the industry standard. Gitlab is a website that stores repositories remotely and runs through git in order to act as an online hub for which
people can collaborate and write code using git.

## Question 2

What is the difference between rustc and cargo?

There are a couple of differences between rustc and cargo. rustc is a compiler for the rust language, so it will read rust code and turn use that to create machine readable code, so that the program can be run. Cargo is better described as a build-system
because it uses rustc to compile programs, however, it also keeps track of things like dependencies that your program relies on, and then downloads, compiles, and links them automatically to your program, when you do not already have them. It does this all automatically
for you, whereas if you were only using rustc, you would have to compile and link each dependency with your program yourself.

## Question 3

What do the following commands do?

- git add
The git add command adds a specified file or folder that has been changed to be staged and prepared to be committed.

- git pull
The git pull command retrieves the most recently committed code from a specific repository that is not stored locally, and in the process updates the locally stored code where there are no conflicts. 

- git commit
The git commit command creates a commit with the specified files that are currently staged. A commit essentially represents a unit of change in code or a codebase and is therefore used to incrementally build software.

- git push
The git push command sends recently committed code from a repository that is stored locally to its counterpart which is not stored locally, and is updated with the new code.
