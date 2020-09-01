# CSE 262 - Homework 1

**Due Date:** 08/31/2020 by EOD.

## Assignment Description

For this assignment, you will follow the guessing game tutorial in [Chapter 2 of the Rust Book](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html).

The first thing you'll want to do is a set up a Rust toolchain. This is covered in [Chapter 1 of the Rust Book](https://doc.rust-lang.org/book/ch01-01-installation.html). Alternatively, you could use an [online development environment like Repl.it](https://repl.it/languages/rust) to do your work.

## Instructions

I want you to do this assignment a particular way. To begin, you'll fork this repository into the namespace for the group you just created for this course. For example, if my group is named `cmontella-cse262`, then you will fork this project into `cmontella-cse262/homework-1`. Do not fork it into your personal namespace (e.g. `cmontella/homework-1`), as the graders will not be able to see it there.

Next, clone this repository onto your desktop (or onto Repl.it if that's what you're using.) Now start following the tutorial. It will start out by telling you to make a directory called `guessing_game`. Follow the instructions to use the `cargo` tool to create this directory and all the boilerplate files within. Follow the rest of the instructions in this section, and then pause when you reach the end. When you've gotten to this point in the tutorial, I want you to commit all the changes you made with the commit message `"Setting Up a New Project"`.

This is something you'll have to get used to for this semester. Many students are used to working on an assignment and then submitting it at the end. In this remote environment, I want more of a log to prove that the work you've done is your own. Therefore, I will require you to periodically commit your work by breaking an assignment into steps or sections.

For instance, in this assignment I want you to just follow along with the code as it's explained in the tutorial. When the tutorial demonstrates new code to you, I want you to modify your source to match. Then at the end of each section, I want you to commit your progress with the message being the title of the section.

Here are the sections you need to commit:

- Setting Up a New Project
- Processing a Guess
- Storing Values with Variables
- Handling Potential Failure with the Result Type
- Printing Values with println! Placeholders
- Using a Crate to Get More Functionality
- Updating a Crate to Get a New Version
- Generating a Random Number
- Comparing the Guess to the Secret Number
- Allowing Multiple Guesses with Looping
- Quitting After a Correct Guess
- Handling Invalid Input

In total you should have at least 12 commits to your repository. You can always have more, but I want you to have at least the above.

For the first couple assignments it will be okay if you get this wrong. You'll have a chance to get used to this process. For HW3 we will start taking it more seriously and you will lose 1 letter grade if you forget to commit your work until the end. Later assignments, including exams, will not be accepted if there is no git history to prove the provenance of your work. This reasoning also applies for submission methods outside of Gitlab. All assignments and exams must be submitted through git, otherwise I cannot see the history of the repository.

By the end of this assignment, you should have a working guessing game. For full credit, graders should be able to run the following commands to compile and run your assignment:

```
git clone https://gitlab.com/<<your-id>>-cse262/homework-1
cd homework-1/guessing_game
cargo run
```
