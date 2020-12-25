# CSE262 - Programming Languages - Homework 8

**Due Date: 11/05/2020 by EOD**

**This assignment is out of 8 points. All questions are weighted equally.**

## Instructions

1. Fork this repository into your CSE262 project namespace. [Instructions](https://docs.gitlab.com/ee/workflow/forking_workflow.html#creating-a-fork)
2. Clone your newly forked repository onto your development machine. [Instructions](https://docs.gitlab.com/ee/gitlab-basics/start-using-git.html#clone-a-repository) 
3. As you are writing code you should commit patches along the way. *i.e.* don't just submit all your code in one big commit when you're all done. Commit your progress as you work. You should have at least one commit per question. **The assignment will not be accepted if you do not follow this procedure.**
4. When you've committed all of your work, there's nothing left to do to submit the assignment.

## Assignment

Solve the following 8 problems using the Haskell programming language. For each problem, commit a new file with the name as the question number. e.g. for Question one, commit a file with your solution as `question_1.hs`. Within each file you can define as many functions as you want, but the "main" function in the file (that will be called by the graders to check your work) should be named according to the question i.e. the first question function would be `question_1`

Each question is weighted equally. 1 point per question for a correct solution, partial points for an attempt that isn't correct.

*Note: If you want to write any formatted text in the Readme, make sure it is surrounded by a Markdown code fence to maintain the formatting in the rendered view (there are examples of how to do so in the source of this document). We will deduct 10% for poor formatting from here on out.*

For each problem I will ask you to write down the function signature in this Readme file. This is worth 20% of the points for each problem (i.e. since each problem is worth 1 point, writing down the function signature here is worth .2 points). surround the function signature in a code fence as in the following example:

```
question_0 :: (Num a) => a -> a -> [a]
```

**Also:** for each problem, you must write an explanation for how your solution works. A complete explanation will include a discussion of the design (e.g. why did you design it the way you did, what alternatives did you consider), a description of what the code does, and any shortcomings or sharp edges to your solution (e.g. are there any edge cases that are not handled, will the function fail on some input, is the implementation efficient, etc.). As part of your explanation, give an example of how the function is called, and what is the expected output. This is worth another 20% of points for each problem.

## Question 1

Given an unsorted array of numbers (you can assume they are of type `Int`), find all pairs with the given sum. For example, your function will have two inputs: 

1. An array of any length e.g. `[1, 8, 5, 2, 5, 6, 7, 3, 9, 4]`
2. A target sum e.g. `10`. 

The output will be an array of all pairs of numbers that sum to the target e.g. `[(1,9),(8,2),(5,5),(7,3),(6,4)]`. 

Before you go about implementing this function, write down the function signature here in the Readme:

```
question_1 :: (Eq a, Num a) => [a] -> a -> [(a, a)]
```

Explanation:

When I originally looked at this problem, I first considered how I would solve the problem with an imperative programming language like Java. In Java, my plan would be to iterate over the list, saving previous values within a hashmap for quick lookup to see if the necessary value to reach the target has been traversed. I was originally going to write my haskell program similarly by using recursion for iteration and also using the Data.Map data structure that haskell has as a hashmap for previous elements, however, I then realized that a list comprehension could end up solving the problem with much cleaner code. So, what I ended up doing was building a recursive function that uses a `let` statement in order to build and store a list comprehension which created tuple pairs `(x, y)` where `x` was defined by the head of the input list, and `y` is defined by all values in the tail of the input list where `x + y = target` where `target` is the input target value. The function then uses the haskell `++` syntactic sugar to add the result of this to the result of the recursive call of `question_1` where the list input is the tail list of the input, and the input target is equal to `target`. The function is called as 

```
question_1 [list] target
```
 
For example, it can be called like this: 

```
question_1 [1,2,3,3,4,5] 6
```

and this call will return:

```
[(1,5),(2,4),(3,3)]
```

This algorithm should return the correct result on any input with the exception of an integer overflow, where it will end up throwing an exception. I believe this solution to be relatively efficient because comparitive to the naive O(n^2) solution of just checking each value with each other value, this solution is more efficient.

## Question 2

Given an unsorted array of numbers (you can assume they are of type `Int`), find all numbers such that the number is greater than all elements that occur after it in the array. For example, given the array:

```
[10, 9, 1, 5, 7, 3, 6, 2]
```

Your function will return:

```
[10, 9, 7, 6]
```

Before you go about implementing this function, write down the function signature here in the Readme:

```
question_2 :: Ord a => [a] -> [a]
```

Explanation:

When looking at this problem, I initially came up with a brute force solution that I could think of in an imperative-style way. This solution involved iterating through each list checking each value and the values after it to see if the current value is the largest. I then adapted this approach to a haskell form of the program using some of the built-in functions that haskell has such as maximum (which after some research appears to be a very fast function). Based upon this, I developed a recursive solution with pattern matching using guard rails where in the case that the current value `x` (the head of the input list) is greater than the maximum of the tail of the input list, a list of length 1 with `x` in it is added to the result of a recursive call to the function with the input being the tail of the current list. The otherwise guard would return the result of just a recursive call on the tail of the current list because the head isn't larger than every other element. This function should work for any list of integers passed into it, with the exception of an overflow which result in an integer overflow exception. While based on the brute force solution, the implementation is still somewhat efficient because of the efficiency of the built-in `maximum` function in haskell. Everything runs in constant time except for `maximum` and the recursive calls (which will be called n - 1 times). In order to run this function, you use input like this:

```
question_2 [list]
```

for example:

```
question_2 [3,4,2,1,7,2,3,5,2]
```

will end up resulting in:

```
[7,5]
```

## Question 3

Given a string that represents the current time on a continuous 12 hour analog clock, calculate the smaller angle in degrees between the two hands.

For example given the string "9:00", the output will be 90.

Notice I've described the clock as operating continuously. That means after the hour, as the minute hand moves, the hour hand continues to move as well. So at 9:30, the hour hand will be half way between 9 and 10. Assume that the hour hand moves at a constant rate between numbers, so that at 10:00 it is pointing exactly to the 10, and at 10:01 it has moved 1/60th of the way to 11.

For example, given the input "2:30" the output will be 105.

Before you go about implementing this function, write down the function signature here in the Readme:

```
question_3 :: [Char] -> Int
```

Explanation:

I began looking at this problem mathematically, instead of in an algorithm context. When looking at the problem mathematically, we can see that the angle between the two hands is the difference between the angle of the hour hand (relative to 0) and the angle of the minute hand (relative to 0). The angle of the minute hand can be calculated by multiplying 360 by the fraction of the value of the minute hand over the 60 minute maximum. The angle of the hour hand can be calculated by multiplying 360 by the fraction of the value of the hour hand over the 12 hour maximum and then adding the product of 30 (because 360 / 12 = 30) and the fraction resulting from the value of the minute hand divided by the maximum value of 60 (this is the part that gets the hour hand movement from the minutehand location). After having the concept of the math complete, I started by implementing an input reader. I did this by using a list comprehension to delimit the `:` out of the string, then passing it to another function which returns a list of 2 double values (the hour value and minute value) using the length to determine if the hour value is one or 2 digits. I then use this list to be able to easily access the hour and minute values during calculation. I then applied the math I stated above in order to calculate the angle for the hour hand and minute hand (then passing these angles into a `fixAngle` function in order to adjust the angle if they are `>=` 360 degrees). Finally, I passed these two angles into and returned the `calcAngle` function which finds the difference between the two angles by subtracting the larger angle from the smaller angle (and adjusts it to the smaller version of the angle if the angle is `>` 180 degrees). This solution will work for any input that would be considered correct as a standard 12 hour time, although for inputs that are not correct on a 12 hour clock, the solution is likely not correct. The function will fail on any input with an integer overflow, unexpected character, and some non-valid 12 hour clock inputs. This implementation is relatively efficient. It will run in `O(n)` because the dominating runtime portions of the code run in `O(n)`. Inputs from a problem like this are very small (at the biggest), so running time is not extremely crucial either. The function is called as 

```
question_3 input
```

Where `input` is a string formatted as `X:YZ` or `WX:YZ`. For example, it can be called like this: 

```
question_3 "9:45"
```

and this call will return:

```
22
```

## Question 4

Given a M x N matrix, where each element is a number (you can assume they are of type `Int`), find the least cost path from the top left element to the bottom right element. Valid movements are only to adjacent elements (i.e. no diagonal movements). You can't visit an element more than once. For each path, calculate the sum of the elements in that path. The function should return the sequence corresponding to the lowest cost path. For example, let's say your input is a 2 x 3 matrix:

(this isn't Haskell code)

```
[6 9 3
 2 5 7]
```

The valid paths are:

```
6 -> 9 -> 3 -> 7 = 25
6 -> 9 -> 5 -> 7 = 27
6 -> 2 -> 5 -> 7 = 20
6 -> 2 -> 5 -> 9 -> 3 -> 7 = 32
```

The function will therefore return:

```
[6, 2, 5, 7]
```

Before you go about implementing this function, write down the function signature here in the Readme:

```
question_4 :: (Eq a, Num a) => [[a]] -> [a]
```

Explanation:

When approaching this problem, I initially thought about the shportest path problem that we have been studying in CSE 340 and using Dijkstra's algorithm or the Bellman Ford algorithm in order to solve this problem. Unfortunately, I was unable to convert input into a proper graph in haskell. What I instead decided to do was opt for a sub-optimal solution. Instead of Dijkstra's algorithm or the Bellman Ford algorithm, I instead decided to develop my own greedy algorithm to solve the solution. The way I did this was by setting the function `question_4` equal to the function call of `greedySolve` (when not a default case such as an empty matrix which returns an empty list, or a single row/column matrix which just returns the list) so that I could add some extra useful data bits to use in the algorithm. `greedySolve` is a recursive function which uses multiple nested `if` statements to determine which greedy decision to make. It has if statements to handle whether or not the the current location is at one of the edges of the matrix, and at each point, it uses this information in order to make the "greedy" decision to move itself towards the end-goal (i.e. the move that will result in the smallest increase in the total path sum). Each of the `if` blocks within this function handles each scenario and what possible decision can be made next. Unfortunately however, there is a bug within the function, which results in `Exception: Prelude.!!: index too large` error, which I was unable to pinpoint in my debugging, so as it sits, it is not functional, but theoretically correct for a greedy algorithm. That being said, because it is a greedy algorithm and the problem does not contain a greedy choice property, while the algorithm can offer a good (and most of the time correct) guess to the solution, it is possible that in certain edge cases where an early greedy choice takes the path away from where it should be, that the wrong output will be given by the algorithm. On the bright side, the algorithm is very efficient as due to the greedy choice, it only evaluates for the surrounding paths for the spot that it chooses to move to, so the running time is a O(mP) = O(P) where m is a scalar and P is the number of elements in the determined path. The function is called as

```
question_4 matrix1
```

Where each matrix is represented by a list of lists (each of the same size) where each subsequent list represents a subsequent row and each value within the internal lists represents the value in the column of that index for that particular row. For example 

```
[6 9 3
2 5 7]
```

is represented in input as: 

```
[[6, 9, 3], [2, 5, 7]]
```

For example, it can be called like this: 

```
question_5 [[6, 9, 3], [2, 5, 7]]
```

and by this greedy algorithm this should be returned:

```
[6, 2, 5, 7]
``` 

However, because there is a bug that I could not find, this will instead be returned:

```
*** Exception: Prelude.!!: index too large
``` 

## Question 5

Write a function that multiplies two matrices together. If you're not familiar with matrix multiplication, you can read about that [here](https://en.wikipedia.org/wiki/Matrix_multiplication).

Don't use any built-in libraries or functions to do this. I want you to calculate the result at a granular level.

For example:

Let matrix A be:

```
[1 2 3
 4 5 6]
```

Let matrix B be:

```
[7 8
 9 10
 11 12]
```

Then A * B is:

```
1*7 + 2*9 + 3*11  = 58
1*8 + 2*10 + 3*12 = 64
4*7 + 5*9 + 6*11  = 139
4*8 + 5*10 + 6*12 = 154
A * B = [58  64
         139 154]
```

Notice that in order to do this calculation, we need the same number of rows in B as there are columns in A. If A were 2 x 4 and B were 3 x 2, we wouldn't be able to complete this computation. Your function should handle this, by returning the result of the matrix multiplication if it can be calculated, or something else if it can't.

Before you go about implementing this function, write down the function signature here in the Readme:

```
question_5 :: (Eq a, Num a) => [[a]] -> [[a]] -> [[a]]
```

Explanation:

Because I came into this problem with an already solid understanding of matrix multiplication, I was able to come in with an idea about how I wanted to build the algorithm. Because we were not allowed to use built-in functions, and I knew that I wanted to use certain prebuilt functions, I built my own versions of these functions using simple recursion and pattern-matching (these functions were `length'`, `take'`, `sum'`, and `drop'`). I then realized that I would like to get any 2D matrix input into a 1D list, so that I could more easily manipulate elements without any indexing issues. I did this by building a function called `adjustMatrix1` which goes row by row and returns a list of all of the numbers in the row by row order. Another function called `adjustMatrix2` does the same thing, but going column by column instead. I did this by looping through each row in a loop of each column to get each specific column value in the correct order. Because of these functions I developed, two separate lists are created from the two matrices, each of which is in the proper order for multiplication. I then created a `multiply` function which takes the length of the rows in the first matrix and multiplies those rows with each of the column vectors in chunks which are the size of the length of the rows in the first matrix (i.e. in an (M x N)(N x K) multiplication, `length = N`) using pattern matching guard rails to iterate through the rows and columns. This function recursively calls itself with updated row and column values (previously used values are dropped, and the column list is reset to the original state anytime it is empty, since each row multiplies by every column before moving to the next row) to the `sum'` of the current value. This function then returns a list of all of the results of the multiplication and addition. After this was done, I developed a function that uses the length `K` (as previously shown) and delimits the resulting list into a list of lists with length `K`. It does this recursively by adding the list containing a single list of length `K` of the current values to the recursive call of the function where `drop'` is used on the first `K` indecies of the input. This in turn is the resulting matrix from the multiplication. When I built the `question_5` function itself, I set it equal to a series of function calls starting with `reformat` with a length of `K` and an input list as the result of calling `multiply` with a length of `M` (as described earlier) and input lists as the matrix 1 built using `adjustMatrix1` and matrix 2 built using `adjustMatrix2`. This algorithm should work on any input which does not contain an overflowed integer value within the input matrices or an invalid matrix (i.e. different rows of different lengths in the same matrix). In the event that the matrices are not multipliable, `[[]]` will be returned (this is handled through an if statement checking to see if the dimensions are compatible at the beginning). This implementation is not super efficient as my custom functions that I built to represent the built-in haskell function are likely not as optimized as those that are built in. Other than that however, the algorithm only traverses each element in each matrix exactly the number of times necessary to mathematically compute the result, so it is as efficient as it can be in that respect. The function is called as 

```
question_5 matrix1 matrix2
```

Where each matrix is represented by a list of lists (each of the same size) where each subsequent list represents a subsequent row and each value within the internal lists represents the value in the column of that index for that particular row. For example 

```
[6 9 3
2 5 7]
```

is represented in input as: 

```
[[6, 9, 3], [2, 5, 7]]
```

For example, it can be called like this: 

```
question_5 [[1,2,3],[4,5,6]] [[7,8],[9,10],[11,12]]
```

and this call will return:

```
[[58,64],[139,154]]
``` 

## Question 6

Given an array of numbers (you can assume they are of type `Int`), find the equilibrium index. The equilibrium index is the index of the array such that the sum of the numbers to the left of the equilibrium index is equal to the sum of the numbers to the right.

For example, given the following array:

```
[5 8 7 2 4 3 10 9 7]
```

The equilibrium index is 5, because `5 + 8 + 7 + 2 + 4 = 26` and `10 + 9 + 7 = 26`

It is possible that the given array does not contain an equilibrium index. Your function should handle this.

Before you go about implementing this function, write down the function signature here in the Readme:

```
question_6 :: (Eq a, Num a) => [a] -> a
```

Explanation:

When looking at this problem, I initially began considering the solution by attempting to come up with a list comprehension that would just be able to solve it in one sweep, however, I could not figure out a way to do this. Instead of doing this, I moved to a more brute force style solution. I made it so that the `question_6` function returns the call of another function called `findEquilibrium`. I did this so that I could add some more data values to pass into the function to keep track of, without making it an awkward function call for the user (the intial value of these values is the same for every instance of the function). Within the `findEquilibrium` function, I used pattern matching to return the `index` if the sum of both sides of the list were equal to each other, return `-1` if the entire list had been traversed and no equilibrium point existed, and return a recursive call of `findEquilibrium` with an updated total, incremented list, and the tail of the current list in the `otherwise` case. I also have `question_6` return `-1` for an empty list because there is no equilibrium point, and `0` in the case of a list of length `1` because it is the trivial solution to the problem. This implementation of the function should produce the correct result for any input of valid integers, although it will throw an exception if the integers overflow. This implementation is somewhat inefficient as in the worst-case it will run at `O(n^2)`, which is not the best, however, unless the input is VERY large, this shouldn't be a major issue. The function is called as 

```
question_6 [input]
```

For example, it can be called like this: 

```
question_6 [1,2,3,4,5,7,3,3,9]
```

and this call will return:

```
5
```

## Question 7

Given an input string, remove all adjacent duplicate characters until no adjacent duplicates remain.

For example, given the string:

```
BZXYYXYYZA
```

The output will be `BA`.

This result will be found through multiple passes. In the first pass, duplicates will be removed to obtain `BZXXZA`

In the second pass the function will obtain `BZZA`

In the third and final pass the function will remove the final pair of duplicate letters to obtain `BA`

Before you go about implementing this function, write down the function signature here in the Readme:

```
question_7 :: [Char] -> [Char]
```

Explanation:

Based upon the prompt and it talking about each "pass" through the string, I knew that I wanted to use recursion to solve this problem. What I ended up doing was building a recursive algorithm where when passed an empty string returns an empty string, when passed a one character string returns that character, and in any other case uses pattern matching to determine what to do. I developed a function called `containsDuplicates` which uses multiple different nested `if` statements to keep track of the last character, as well as the beginning and ending index for a repeated string of characters. This function ends up returning a tuple containing a character that is repeated, the beginning index of the repetition, and the ending index of the repetition. I also developed a function called `remove` to remove characters of a specified value within a specified index range. The "main" function `question_7` first checks if the string contains any duplicates by running `containsDuplicates` and comparing the returned value of the first index of the tuple with the default return value (if the default == returned value, there are no repeated characters). In the event that they are equivalent, the string has no repeating characters, and so the current string is returned. In the `otherwise` case, `question_7` is recursively called on a string that is the result of the `remove` function being called on the appropriate character with the appropriate indecies. While this function will work for nearly every case, it will fail in the edge case of repeated ' ' characters because I ended up using the  ' ' character as the default in the `containsDuplicates` function. Other than this one instance however, as long as a valid string is input, the function will perform correctly. The function is not extraordinarily efficient because it calls `containsDuplicates` 3 times in the `otherwise` case instead of saving the result as a variable (I tried to do this, however, I got errors in every implementation I did), however, it only multiplies the runtime by a scalar (which is not a dominating factor), so the big O running time is still the same as it otherwise would be. The function is called as 

```
question_7 input-str
```

For example, it can be called like this: 

```
question_7 "abbcgskjlllllammntt"
```

and this call will return:

```
"acgskjan"
```

## Question 8

Implement the [mergesort](https://en.wikipedia.org/wiki/Merge_sort) algorithm in Haskell. Your function should be able to accept characters and numbers.

Before you go about implementing this function, write down the function signature here in the Readme:

```
question_8 :: (Ord a) => [a] -> [a]
```

Since you can easily look this up on the internet (there are a billion implementations out there), and it's kind of difficult, I expect you'll probably glace at a solution. If you do, cite it here. That's fine. But for this problem the explanation of the code will be worth **60%** of the total points for this problem. We will be scrutinizing this one much more heavily, so give a good explanation. A cursory explanation will earn you very few points.

Explanation:

In order to solve this problem, I consulted [this source](https://smthngsmwhr.wordpress.com/2012/11/09/sorting-algorithms-in-haskell/#mergesort) to help me develop my answer. Coming in with an understanding of merge sort, I knew that I needed two different steps (one to split the list and one to merge the splitted lists). For developing the codebase, I used my previous knowledge as well as consulting the above source for syntactical help on some of the implementation. I began by developing the `merge` and `split` functions separately. The `merge` function uses a pattern-matching declaration to deal with the base cases (merging 2 empty lists results in an empty list or merging one non-empty list with an empty list in any order results in the non-empty list), as well as the case where the two lists are both non-empty. When the two lists are non-empty, the function compares the head of each list to each other and then returns whichever head is smaller in its own list added (using `++` syntactic sugar) to a recursive call of the function where the tail of the list whose head was smaller is passed in as a list and the entirety of the other list is passed in, that way no elements of the second list are left out of the final result. In practice, this function will continue to recursively call itself until one of the base cases is reached. Next, I developed the `split` function, which is a function that returns two halves of a list as two separate elements in a tuple (I got the idea to return it as a two element tuple from the source listed above). I also defined this function using pattern matching for the input where the base cases are an empty list or a list with one element (the empty list returns a tuple of 2 empty lists, the list with one element returns a tuple with the first list having one element, and then an empty list). In the catch-all case where there is more than 1 element in the list, the function uses the built-in `take` function to access the first half of the elements in the list, and return that as the first element in the tuple and then use the `drop` function to return the rest of the elements in the list as the second "half" in the returned tuple. Having these two functions means that when put together correctly, a merge-sort can be completed. Even in a regular imperative programming language like Java, mergesort is generally implemented recursively, so I knew that I wanted to implement the main function as a recursive function. Because of this, what I ended up doing was defining the function once again using pattern matching, where calling the main function (`question_8`) on an empty list would return an empty list and calling it on a list with one element would just return the input list. When calling mergesort on a list with more than one element is where the recursion comes in. Based on the idea of mergesort, a list is split in half until each list contains only one element (and are therefore trivially sorted by themselves) and then the merging occurs between already sorted lists in order to create another sorted list. Because of this, the algorithm merges two recursive calls of itself where the input list for the first recursive call is the first half of the list and the input list for the second recursive call is the second half of the current list (I got these halves by using the built-in `fst` and `snd` methods on the tuple return of calling the `split` function on the input list). This recursive call continues until each half hits the base case (where there will be stack-frames for each element within the initial list) and since the base case will have been reached, the stack-frames can be popped and each of the recursive calls can be evaluated from the bottom up, using the `merge` function. When each sorted half is combined using merge, the resulting array will be sorted, and this continues until the algorithm finished, returning all of those recursive calls and therefore returning a sorted list. This function will work properly for any input list containing elements of only items within the `Ord` class (meaning that it will work with Integers, Doubles, Floats, Characters, etc.). The function will only fail on input that has an error in it (such as an integer overflow). The implementation is quite efficient. Despite it calling the `split` function twice within the `question_8` function, this only increases the runtime by a scalar, which because isn't the dominating term actually results in the same theoretical running time as any other mergesort algorithm, which is O(nlgn). Because it maintains this theoretical running time, it is an efficient solution. The function is called as 

```
question_8 [input]
```

For example, it can be called like this: 

```
question_8 [2,3,31,54,3,23,5435,1,21,4,4,55,34,6,234,65]
```

and this call will return:

```
[1,2,3,3,4,4,6,21,23,31,34,54,55,234,5435]
```
