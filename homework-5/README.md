# CSE 262 - Programming Languages - Homework 5

**Due Date: 9/30/2020 by EOD**

**This assignment is out of 100 points. All questions are weighted equally.**

## Instructions

1. Fork this repository into your CSE262 project namespace. [Instructions](https://docs.gitlab.com/ee/workflow/forking_workflow.html#creating-a-fork)
2. Clone your newly forked repository onto your development machine. [Instructions](https://docs.gitlab.com/ee/gitlab-basics/start-using-git.html#clone-a-repository) 
3. As you are writing code you should commit patches along the way. *i.e.* don't just submit all your code in one big commit when you're all done. Commit your progress as you work. You should have at least one commit per question. **The assignment will not be accepted if you do not follow this procedure.**
4. When you've committed all of your work, there's nothing left to do to submit the assignment.

## Assignment

This assignment asks you to familiarize yourself with the syntax and semantics of the Clojure programming language. There are 12 problems in total located in the `src` directory.

Each question presents Rust code at the top (commented out) that you are to translate into equivalent Clojure code. Wherever possible, you should keep the spirit of the original Rust code, but Clojure is a different language after all, so your Clojure solution may go about things a different way. The result of each problem is a single value. The value produced by your code will be tested against the expected value using the `is` function in `core_test.clj`. A correct program will pass this test. A program that produces an incorrect result will fail.

## Running

To run the tests, use the following command in the homework-5 root directory.

```bash
> lein test
```

To run the main function, which you may use for testing purposes as you do your assignment (but will not be considered as part of your grade).

```bash
> lein run
```