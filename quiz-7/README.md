# CSE 262 - Quiz 7

**Due: 10/20/2020 EOD**

Write your answers in this README file in the space below. Make at least one commit per question.

## Question 1

What is a lazy execution strategy versus an eager execution strategy? What are the benefits and drawbacks of each?

A lazy execution strategy is a strategy when executing code in which expressions are not evaluated until they need to be evaluated. On the flip side, eager evaluation evaluates all expressions as soon as they are introduced. The benefits of lazy evaluation are that expressions that never end up being used are never evaluated, so runtime is not wasted. You can also create data structures with infinite size because they are only evaluated to points that are necessary for use. The benefits of eager evaluation are that you know that any "dependencies" of a function have already been evaluated. Beyond this, you gain the ability to precisely control when everything is executed. The downsides of lazy evaluation are that you lose control over when specific functions are executed. The downsides of eager evaluation are that runtime may be increased on functions that may not ever actually have needed to be used, and it is not possible to define data structures of infinite size.

## Question 2

Consider the following list comprehension in Haskell:

```haskell
take 10 [ x | x <- [1..], x > 10, x /= 21, odd x ]
```

What is the output? Write a function in either Rust or Clojure that generates equivalent output. How many lines of code did it take?

The output of this function is a list of the first 10 odd numbers greater than 10 excluding the number 21.

This is a rust function that would produce the same output:
```rust
fn calculate() -> Vec<i32> {
  let mut vec = Vec::new();
  let mut count = 0;
  let mut curr = 10;
  while count < 10 {
    if curr % 2 == 0 || curr == 21 {
      curr += 1;
      continue;
    }
    else{
      vec.push(curr);
      curr += 1;
      count += 1;
    }
  }
  return vec;
}
```

This function took 17 lines of code to create because it involves loops and other control-flow components.

## Question 3

The above Haskell code contains some syntax that would be impossible to write in Rust or Clojure. What is that syntax? What makes it impossible in Rust/Clojure and why is it possible to express this Haskell?

The syntax in the haskell code that is impossible to express is the list `[1..]`. This list is an infinite list, so it goes on forever. This is impossible to express in Rust/Clojure because Rust and Clojure are eager execution languages, so they would evaluate the value of this list first, which is impossible because the list goes on infinitely. However, in Haskell, with its lazy evaluation, it only needs to evaluate the numbers that it needs to use for the take function that is called on it. Because of this, it works perfectly fine and doesn't even attempt to evaluate anymore numbers past what it needs to within the list.