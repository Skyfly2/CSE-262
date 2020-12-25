# CSE262 - Programming Languages - Homework 9

**Due Date: 11/30/2020 by EOD**

**This assignment is out of 12 points. All questions are weighted equally.**

## Instructions

1. Fork this repository into your CSE262 project namespace. [Instructions](https://docs.gitlab.com/ee/workflow/forking_workflow.html#creating-a-fork)
2. Clone your newly forked repository onto your development machine. [Instructions](https://docs.gitlab.com/ee/gitlab-basics/start-using-git.html#clone-a-repository) 
3. As you are writing code you should commit patches along the way. *i.e.* don't just submit all your code in one big commit when you're all done. Commit your progress as you work. You should have at least one commit per question. **The assignment will not be accepted if you do not follow this procedure.**
4. When you've committed all of your work, there's nothing left to do to submit the assignment.

## Assignment

The purpose of this assignment is to model the game of Checkers in Prolog. You don't have to write any code that is meant to run for this assignment, although if you want you may. For this assignment you can assume that the set of built-in predicates available to you are [the same as SWI-Prolog](https://www.swi-prolog.org/pldoc/man?section=builtin). 

Some useful predicates:

- [retract](https://www.swi-prolog.org/pldoc/man?predicate=retract/1) - The fact or clause is removed from the database.
- [arithmatic](https://www.swi-prolog.org/pldoc/man?section=arith) - Pay attention to the use of the [is](https://www.swi-prolog.org/pldoc/doc_for?object=(is)/2) predicate to assign values to an unbound value e.g. `X is 1 + 2`
- [not a.k.a. \+](https://www.swi-prolog.org/pldoc/doc_for?object=(%5C%2B)/1) - Returns `true` if a target goal cannot be reached i.e. a fact or predicate cannot be found in the database.

In this assignment I give you a number of suggested predicates to use. You may model your problem any way you like, including adding any predicates you need. For any code you write, also include an explanation of the code. There will be few points given for a solution without an explanation. If you can't write the code necessary, write an explanation of what the code might look like to get partial points. You'll get more points for an explanation without code than you will for code without an explanation.

## The Game Board

The standard Checkers game board is an 8x8 grid of cells. Starting at (1,1), every other cell is restricted from play. Player 1 is indicated by `x` markers. These occupy the first three rows. Player 2 is indicated by `o` markers. These occupy rows 6 through 8. Rows 4 and 5 are empty, indicated by underscores.

```
   1  2  3  4  5  6  7  8  
1     x     x     x     x  
2  x     x     x     x     
3     x     x     x     x  
4  _     _     _     _     
5     _     _     _     _  
6  o     o     o     o     
7     o     o     o     o  
8  o     o     o     o    
```

We can call x at (1,2) x1; x at (1,4) x2; x at (2,1) x5 etc.

We can call o at (6,1) o1; o at (6,3) o2; o at (7,2) o5 etc.

### Valid Moves

A player's pieces may only move away from the player if the pieces are not "kinged" (imbued with additional abilities). 

There are two kinds of valid moves: a move into a free space and a "capture". A piece may move into a free space if the space is adjacent diagonally from it, and it is away from the player. If the piece is kinged, it may move into an adjacent free space that is closer to the player.

A capture occurs when a player's piece moves over a diagonally adjacent opponent's piece into a free space. The opponent's piece is removed from play following a capture.

If a player's piece moves into the furthest row from the player (row 1 for player `o`, row 8 for player `x`), then that piece is considered "kinged", which allows it greater freedom in movement (it can now move toward the player instead of strictly away).

### Winning

The game ends when one player can no longer make any moves, either by being blocked from making moves, or by having all pieces captured (and removed from play).

## Questions

### Question 1

Write down all of the facts representing the game pieces given their positions at the start of the game. We can use the `occupied(A,B)` predicate for this, where A is the game piece and B is the `cell`. e.g. `occupied(x1, cell(1,2))`. 

```
x1.
x2.
x3.
x4.
x5.
x6.
x7.
x8.
x9.
x10.
x11.
x12.
o1.
o2.
o3.
o4.
o5.
o6.
o7.
o8.
o9.
o10.
o11.
o12.

x(x1).
x(x2).
x(x3).
x(x4).
x(x5).
x(x6).
x(x7).
x(x8).
x(x9).
x(x10).
x(x11).
x(x12).
o(o1).
o(o2).
o(o3).
o(o4).
o(o5).
o(o6).
o(o7).
o(o8).
o(o9).
o(o10).
o(o11).
o(o12).

cell(1,2).
cell(1,4).
cell(1,6).
cell(1,8).
cell(2,1).
cell(2,3).
cell(2,5).
cell(2,7).
cell(3,2).
cell(3,4).
cell(3,6).
cell(3,8).
cell(4,1).
cell(4,3).
cell(4,5).
cell(4,7).
cell(5,2).
cell(5,4).
cell(5,6).
cell(5,8).
cell(6,1).
cell(6,3).
cell(6,5).
cell(6,7).
cell(7,2).
cell(7,4).
cell(7,6).
cell(7,8).
cell(8,1).
cell(8,3).
cell(8,5).
cell(8,7).

occupied(x1, cell(1,2)).
occupied(x2, cell(1,4)).
occupied(x3, cell(1,6)).
occupied(x4, cell(1,8)).
occupied(x5, cell(2,1)).
occupied(x6, cell(2,3)).
occupied(x7, cell(2,5)).
occupied(x8, cell(2,7)).
occupied(x9, cell(3,2)).
occupied(x10, cell(3,4)).
occupied(x11, cell(3,6)).
occupied(x12, cell(3,8)).
occupied(o1, cell(6,1)).
occupied(o2, cell(6,3)).
occupied(o3, cell(6,5)).
occupied(o4, cell(6,7)).
occupied(o5, cell(7,2)).
occupied(o6, cell(7,4)).
occupied(o7, cell(7,6)).
occupied(o8, cell(7,8)).
occupied(o9, cell(8,1)).
occupied(o10, cell(8,3)).
occupied(o11, cell(8,5)).
occupied(o12, cell(8,7)).
```

###### Explanation

The prolog code that I wrote above is just a series of facts that will be stored in the database and can later be queried upon. I split the facts into three distinct sections, one defining the 24 pieces in the game. The next section is a series of facts defining what team each piece belongs to. The third section defines the cells that exist within the gameboard which could eventually be occupied by game pieces. The final section is a list of facts using the `occupied` predicate, which defines the starting position of each of the 24 game pieces based upon the example gameboard given above. When defining the cells, I skipped over combinations of rows and columns which would be impossible for a piece to ever move into. Because those spaces on the gameboard essentially act as "spacer" cells to make the game more understandable for the human player, they are not actually necessary to include within the program because they have no logical relevance to the game because their state will never end up changing. 

### Question 2

Write a horn clause (or clauses) to introduce the free cells with the predicate `free(A)`, where `A` is the free `cell`. For example, in the initial game state we would have `free(cell(4,1))`, `free(cell(4,3))`, `free(cell(4,5))` etc.

```
free(A) :- \+ occupied(_, A).
```

###### Explanation

The prolog code that I wrote above is a singular horn clause which can be used in order to dynamically generate all of the free cells (which a game piece could potentially move into) at any given time based upon the current state of the occupied cells on the game board. It does this by using the predicate `free(A)` where `A` is a free cell in the current board state, and sets this as a rule that is true if that cell `A` does not appear as the `cell` within any `occupied(piece, cell)` facts that are within the database. For any cell `A` that does not appear within an `occupied` fact in the database, the fact `free(A)` is generated by this horn clause and added to the database.

### Question 3

Write a horn clause (or clauses) to mark the cells which are not free as `occupied(A)`, where `A` is a cell with either an `x` or an `o` piece on it. e.g. `occupied(cell(1,2))`.

```
occupied(A) :- \+ free(A).
```

###### Explanation

The prolog code that I wrote above is a singular horn clause which can be used to dynamically generate all of the occupied cells (disregarding what piece it is occupied with) at any given time based on the current state of the free cells on the game board. It does this by setting a rule that the `occupied(A)` predicate is true for any cell `A` which does not exist as a fact in the database as the `cell` in `free(cell)`. Because any cell that is not `free` must be occupied, the facts can be generated and added to the database for all of the cells that are occupied. 

### Question 4

Consider the piece `o3` at `cell(6,5)`. It has two valid moves: it can move into either `cell(5,4)` or `cell(5,6)` because they are both free. We can use the following predicate to express this: `valid_move(A,B)`, where A is a player's piece and B is a free space. e.g. `valid_move(o3, cell(5,6))`.

Write a horn clause (or clauses) to generate the set of all valid free moves for all of player `o's` pieces.

Don't considered kinged pieces for this question.

```
valid_move(A, B) :- (kinged(A); o(A)), occupied(A, cell(X, Y)), cell(U is X-1, V is Y-1), free(cell(U, V)), B == cell(U, V).
valid_move(A, B) :- (kinged(A); o(A)), occupied(A, cell(X, Y)), cell(U is X-1, V is Y+1), free(cell(U, V)), B == cell(U, V).
valid_move(A, B) :- (kinged(A); x(A)), occupied(A, cell(X, Y)), cell(U is X+1, V is Y+1), free(cell(U, V)), B == cell(U, V).
valid_move(A, B) :- (kinged(A); x(A)), occupied(A, cell(X, Y)), cell(U is X+1, V is Y-1), free(cell(U, V)), B == cell(U, V).
```

###### Explanation

For this problem, I wrote 2 different horn clauses both describing the `valid_move(A, B)` predicate. Both horn clauses have the same base logic, they just use slightly different arithmetic in one of the expressions. These clauses are made up of multiple different premises which must all be true in order for the clause to be true in general. The first premise is that a `cell` with the arbitrary coordinates `(X, Y)` is occupied by `A`. These coordinates are then useable as `X` and `Y` for the rest of the horn clause. The next expression is where the two horn clauses differ. In the first rule defining of `valid_move(A, B)`, the fact that the `cell` at coordinates `(X - 1, Y - 1)` is validated to exist on the gameboard. In the second rule defining of `valid_move(A, B)`, the fact that the `cell` at coordinates `(X - 1, Y + 1)` is validated to exist on the gameboard. These coordinates are saved as `(U, V)` for use later. After this expression, the rest of the two horn clauses are the same. `free(A)` is called on the `cell(U, V)` to check that it is free, because a piece can only be moved there if it is not already occupied by another piece. Finally, the unevaluated value of B must be equal to `cell(U, V)` in order for `valid_move(A, B)` to be true because at this point, `cell(U, V)` is proven to be valid cell to move the piece to because it is in the movement range of the piece, and it is not already occupied. In `question 12` I added `kinged`, `x` and `o` expressions to the horn clauses. You can see the explanation under that explanation section.

### Question 5

Consider the following game state

```
   1  2  3  4  5  6  7  8  
1     x     x     x     x  
2  x     x     x     x     
3     _     x     _     x  
4  _     x     x     _     
5     o     o     _     _  
6  _     _     o     o     
7     o     o     o     o  
8  o     o     o     o    
```

Write a horn clause (or clauses) to express all adjacent opponent pieces. Use the predicate `adjacent_to(A,B)` for this, where A is the player's piece, and B is the opponent's piece. e.g. `adjacent_to(o2,x11)`.

```
adjacent_to(A, B) :- (kinged(A); o(A)), occupied(A, cell(X, Y)), occupied(B, cell(U is X-1, V is Y-1)), o(A), x(B).
adjacent_to(A, B) :- (kinged(A); o(A)), occupied(A, cell(X, Y)), occupied(B, cell(U is X-1, V is Y+1)), o(A), x(B).
adjacent_to(A, B) :- (kinged(A); x(A)), occupied(A, cell(X, Y)), occupied(B, cell(U is X+1, V is Y-1)), o(A), x(B).
adjacent_to(A, B) :- (kinged(A); x(A)), occupied(A, cell(X, Y)), occupied(B, cell(U is X+1, V is Y+1)), o(A), x(B).
```

###### Explanation

In order to solve this problem, I wrote 2 horn clauses defining `adjacent_to(A, B)`. These horn clauses are identical except for one arithmetic difference within them. The first premise within both of them is that a `cell` with arbitrary coordinates `(X, Y)` is occupied with the game piece `A`. These coordinates are then reusable as `X` and `Y` for the rest of the horn clause. In the next premise of the horn clause, the cell located at coordinates `(X - 1, Y - 1)` for the first horn clause and the cell located at coordinates `(X - 1, Y + 1)` for the second horn clause is occupied with the game piece `B`. The next 2 premises consist of checking whether `A` and `B` are x and o pieces respectively by checking to see if those facts already exist in the database. `A` must be o since we are assuming that `A` is the player's piece and `B` is the opponent's piece. Because `A` is o, consequently by the problem, `B` must be x. If these conditions are satisfied, then `adjacent_to(A, B)` is valid for a specific `A` and `B`. I also added two more horn clauses representing the same statements, but in opposite directions for `question 12`.

### Question 6

For the piece `o2`, it has two potential capture moves: either it can take `x9` at `cell(4,3)` or it can take `x11` at `cell(4,5)`. We can use the following predicate to express this: `valid_capture(A,B)`, where A is the player's piece, B is the opponent's piece: e.g. `valid_capture(o2,x9)`. Write a horn clause (or clauses) to generate every `valid_capture(A,B)` for a given game state.

Don't considered kinged pieces or double jumps for this question.

```
valid_capture(A, B) :- (kinged(A); o(A)), adjacent_to(A, B), occupied(A, cell(X, Y)), occupied(B, cell(R, C)), cell(O is X-1, P is Y-1), cell(R, C) == cell(O, P), cell(U is X-2, V is Y-2), free(cell(U, V)).
valid_capture(A, B) :- (kinged(A); o(A)), adjacent_to(A, B), occupied(A, cell(X, Y)), occupied(B, cell(R, C)), cell(O is X-1, P is Y+1), cell(R, C) == cell(O, P), cell(U is X-2, V is Y+2), free(cell(U, V)).
valid_capture(A, B) :- (kinged(A); x(A)), adjacent_to(A, B), occupied(A, cell(X, Y)), occupied(B, cell(R, C)), cell(O is X+1, P is Y+1), cell(R, C) == cell(O, P), cell(U is X+2, V is Y+2), free(cell(U, V)).
valid_capture(A, B) :- (kinged(A); x(A)), adjacent_to(A, B), occupied(A, cell(X, Y)), occupied(B, cell(R, C)), cell(O is X+1, P is Y-1), cell(R, C) == cell(O, P), cell(U is X+2, V is Y-2), free(cell(U, V)).
```

###### Explanation

In order to answer this question, I wrote 2 different horn clauses which both define a valid version of `valid_capture(A, B)` with slight variations in the arithmetic. The first expression of each rule states that A and B must be adjacent to each other (otherwise `A` wouldn't be able to capture `B`). The next two expressions in the horn clause validate that each cell containing the two pieces is occupied by the two pieces. This is done mainly to get access to the coordinates of `A` as `(X, Y)` and `B` as `(R, C)`. The next premise verifies the existence of the cell up one and to the left one in the first horn clause and the cell up one and to the right one in the second cell. The coordinates of this cell are represented as `(O, P)`. The premise validates that the cell at `(O, P)` is equal to the cell occupied by B (I did this in order to validate the direction of the final landing spot for the attacking piece). After this, the potential landing cell for the attacking game piece is validated to exist (up two and to the left two spaces if the piece being attacked is up and to the left, up two and to the right two spaces if the piece being attacked is up and to the right) and its coordinates are represented as `(U, V)`. Finally, in the last expression, it is validated that the cell at `(U, V)` is a free cell. If all these conditions are met, then `B` can be validly captured by `A`. In `question 12` I added `kinged`, `x` and `o` expressions to the horn clauses. You can see the explanation under that explanation section.

### Question 7

Consider a piece `o` at `cell(1,2)` or `x` at `cell(8,1)`. They have made it to the other side of the board, and are now considered "kinged". Write a horn clause (or clauses) to give the "kinged" property to pieces that make this journey. These pieces may travel back toward the player. 

```
kinged(A) :- o(A), occupied(A, cell(1, Y)).
kinged(A) :- x(A), occupied(A, cell(8, Y)).
```

###### Explanation

In order to solve this problem, I wrote 2 different horn clauses defining a valid version of the predicate `kinged(A)` where `A` is either an x or o game piece. The first horn clause accounts for when `A` is an o piece. The first expression in the horn clause checks to see if `A` is defined as an `o` piece in the database. The next premise validates that the cell containing `A` is occupied (this is mainly done to represent the coordinates as `(1, Y)`). Because an o piece is kinged if it reaches the top row (X == 1) of the board. The final premise validates whether there exists a cell occupied by `A` where `X = 1` because by definition, that is when an `o` piece becomes `kinged`. If these conditions are satisfied for `A`, then `kinged(A)` is valid. The first expression in the horn clause checks to see if `A` is defined as an `o` piece in the database. The next premise validates that the cell containing `A` is occupied (this is mainly done to represent the coordinates as `(8, Y)`). Because an x piece is kinged if it reaches the bottom row (X == 8) of the board. The final premise validates whether there exists a cell occupied by `A` where `X = 8` because by definition, that is when an `x` piece becomes `kinged`. If either of these are true for `A`, `kinged(A)` is added to the database. 

### Question 8

Update your answer to Questions 4 and 6 to take into account kinged pieces.

###### Explanation

For question 4, I added 2 more rules defining `valid_move(A, B)`. In both of these horn clauses, they begin with the initial condition needing to be satisfied as `kinged(A)`. Because of this, these moves potential valid moves are only valid for the game pieces `A` which have the fact `kinged(A)` in the database. The remainder of the horn-clauses are the same, each reflecting the two previous horn-clauses, except for the fact that instead of subtracting 1 from the `X` coordinate of the current location of `A` (when checking the existence of the potential move), 1 is added to the `X` coordinate because `kinged` pieces can travel backwards on the board (which would be represented by the incrementing of the `X` value of the current cell). The other horn clauses do not need to be changed because even if `A` is `kinged` they would still correctly be valid. As for question 6, I duplicated the initial 2 horn clauses that I had created for question 6. I added the first expression that must be satisfied for the 2 new horn clauses to be valid as `kinged(A)` for the input game piece `A`. The rest of each of the two horn clauses are mostly the same as the original 2 horn clauses except for the fact that instead of subtracting 1 from the `X` coordinate of the current location of `A` (when checking the existence of the cell occupied by `B`), 1 is added to the `X` coordinate and instead of subtracting 2 from the `X` coordinate of the current location of `A` (when checking the existence of the cell `A` will potentially move to after the capture), 2 is added to the `X` coordinate. These two things are valid because a `kinged` piece can move in the opposite direction of the board which can be represented by changing the sign of the change in the `X` coordinate. The other horn clauses do not need to be changed because even if `A` is `kinged` they would still correctly be valid.

### Question 9

Consider the following game state:

```
   1  2  3  4  5  6  7  8  
1     x     x     x     _  
2  x     x     x     x     
3     _     x     _     x  
4  _     x     x     _     
5     o     o     _     _  
6  _     _     o     o     
7     o     o     o     o  
8  o     o     o     o    
```

The piece `o2` at `cell(5,4)` has two valid capture moves, but one is better: the double capture of x11 and x7. Write a horn clause (or clauses) that can enumate all valid double jumps with the predicate `double_jump(A,B,C)`, where A is the player's piece, B is the opponent's first piece, and C is the opponent's second piece: e.g. `double_jump(o2,x11,x7)`.

```
get_direction(A, B, DX, DY) :- occupied(A, cell(X, Y)), occupied(B, cell(U is X+1, P is Y+1)), DX == 2, DY == 2.
get_direction(A, B, DX, DY) :- occupied(A, cell(X, Y)), occupied(B, cell(U is X+1, P is Y-1)), DX == 2, DY == -2.
get_direction(A, B, DX, DY) :- kinged(A), occupied(A, cell(X, Y)), occupied(B, cell(U is X-1, P is Y-1)), DX == -2, DY == -2.
get_direction(A, B, DX, DY) :- kinged(A), occupied(A, cell(X, Y)), occupied(B, cell(U is X-1, P is Y+1)), DX == -2, DY == 2.
double_jump(A, B, C) :- valid_capture(A, B), get_direction(A, B, DX, DY), retract(occupied(A, cell(X, Y))), assert(occupied(A, cell(U is X+DX, Y is Y+DY))), valid_capture(A, C).
```

###### Explanation

In order to solve this problem, I wrote 5 different horn clauses. The first four horn clauses define a predicate called `get_direction(A, B, DX, DY)` which is valid if and only if a vector defined by `DX` (X direction) and `DY` (Y Direction) is the same as the direction from the piece `A` to the piece `B`. There are 4 possible scenarios (2 of which are only valid if `A` is a `kinged` piece because they involve a capture going backwards). The general definition verifying the cell that `A` and `B` occipies is occupied by `A`, and `B` and represents the coordinates of the two cells as `(X, Y)` and `(U, P)` where the values of `(U, P)` are relative to `(X, Y)` based off of the direction `B` is in relation to `A`. Because of this, only one of the four clauses will ever be valid at a time, and the `DX` and `DY` values must be specific values which are covered by the last 2 premises (`DX` and `DY` are the magnitude of the difference between the resulting location after the capture in the direction of the attack). Because of this, we can use this direction information to update the location of `A` after the first capture. The way the horn clause defining `double_jump(A, B, C)` works is by first validating that `A` can validly capture `B`. It then uses the `get_direction` predicate where the direction is represented by the variables `DX` and `DY` and can be used later. The location of `A` is then retracted from the database, and its new location is asserted by asserting `occupied(A, cell(U is X+DX, Y is Y+DY))` where `U` is the new X coordinate and `P` is the new Y coordinate calculated using the results of the `get_direction` predicate). Finally, if `valid_capture(A, C)` is valid, then the double jump itself is valid.

### Question 10

Write a horn clause (or clauses) which will select the next move player `o` will make. Call it `next_move(A)`, where A is the predicate relating to the move the player will make. Use Prolog's execution model to implement a move selection strategy. The strategy you should use is:

1. Prefer any move which captures the most opposiing pieces.
2. Prefer any move which leads to a kinged piece.
3. Prefer any valid move which does not lead to your opponent taking one or more of your pieces (you can call such moves "threatened" moves).
4. Prefer any valid move.

A full solution will consider all of these strategies. You may consider a subset of these for partial credit.

```
move(A, B) :- retract(occupied(A, _)), assert(occupied(A, B)).
threatened(A, B) :- x(C), retract(occupied(A, cell(X, Y))), assert(occupied(A, B)), valid_capture(C, A), retract((occupied(A, B))), assert(occupied(A, cell(X, Y))).
next(A, B, C) :- (valid_capture(B, C), get_direction(B, C, DX, DY), occupied(B, cell(X, Y)), next(move(B, cell(U is X+DX, P is Y+DY), B, C))); A == A
next_move(A) :- (valid_capture(B, C), get_direction(B, C, DX, DY), occupied(B, cell(X, Y)), next(move(B, cell(U is X+DX, P is Y+DY), B, C)), A == A); (valid_move(D, E), move(D, E), kinged(D), A == move(D, E)); (valid_move(F, G), \+ threatened(F, G), A == move(F, G)); (valid_move(H, I), A == move(H, I)).
```

###### Explanation

In order to solve this problem, I wrote 4 separate horn clauses. The first horn clause defines the predicate `move(A, B)` which retracts the current location of a given game piece `A` from the database and asserts the location as the given cell represented by the vartiable `B`. The next horn clause defines `threatened(A, B)`, which validates a game piece represented by the variable `C` as an x piece, retracts the current cell that `A` occupies and asserts a new location represented by `B`. Next, the `valid_capture` predicate is called on `C` to `A` and then the location is retracted and the old location is asserted again (this is so that the horn clause doesn't actually edit the database because it undoes the changes it needs to make to check if a piece will be threatened. NExt, I write the horn clause defining the predicate `next_move(A)`. I wrote this horn clause so that it is a series of `or` definitions in the order defined by the question because of the order which prolog execute4s. Because of this, the earlier `or` statements will take precedence over the ones defined later. Because of this, there are 4 different `or` conditions that if are valid cause `next_move(A)` to evaluate to true and be added to the database. The first uses `valid_capture`, `get_direction`, and `occupied` predicates to get values and then calls `next` which will continue to recursively call itself until its first expression is not satisfied. The `next` predicate essentially continues to move the piece to the correct spot after making a capture, and then recursively calls itself, making as many captures as possible. The next portion of `next_move` verifies that moving an arbitrary piece to a space, moves the piece to that space, and then validates that the piuece would now be `kinged` if moved to that space. It is then only valid if `A` is equal to that specific move. The next `or` statement involves calling `valid_move` on 2 arbitrary variables `F` and `G` and then evaluates to true only if moving `F` to `G` would not result in `F` being threatened. It does this using the `threatened` predicate I defined earlier. The final `or` statement delas with just finding a valid move. It does this by executing the `valid_move` predicate from arbitrary piece variable `H` to arbitrary cell variable `I`. It is valid if `A` equals the move of piece `H` to cell `I`. If none of these `or` expressions evaluate, then there is no valid move `A`. 

### Question 11

Write a query which will tell you if the `o` player is winning the game (has more tokens than `x`).

```
?- findall(A, o(A), L1), findall(B, x(B), L2), length(L1, LenO), length(L2, LenX), LenO > LenX.
```

###### Explanation

This query will return `false` if the `o` player has less than or equal to the number of pieces than the `x` opponent has (i.e. `o` is not winning). The query will return `true` as well as the values of `L1`, `L2`, `LenO`, and `LenX` if the `o` player has more pieces than the `x` opponent (i.e. `o` is winning). The way that this query works is by using prolog's built-in `findall` predicate for the the first 2 expresions. In the first expression, `findall` is called with the variable `A` as the target, `o(A)` as the template, and `L1` as the list that all of the elements found will be "stored" in. This essentially just finds every fact in the database represented by `o(A)` where `A` can be anything (in this case it will always be representative of a game piece). These facts are all within the list `L1`. In the second expression, `findall` is called with the variable `B` as the target, `x(B)` as the template, and `L2` as the list that all of the elements found will be "stored" in. This essentially just finds every fact in the database represented by `x(B)` where `B` can be anything (in this case it will always be representative of a game piece). These facts are all within the list `L2`. The next 2 expressions use prolog's built-in predicate `length`. The third expression uses the `length` predicate on the list variable `L1` (which has already been defined by the first expression in the query) and the variable `LenO`. Because `length` only evaluates to true if the length of `L1` is equal to `LenO` and because `LenO` hasn't been defined, it essentially just represents the length of `L1` which is the number of `o` pieces on the game board. The fourth expression uses the `length` predicate on the list variable `L2` (which has already been defined by the second expression in the query) and the variable `LenX`. Because `length` only evaluates to true if the length of `L2` is equal to `LenX` and because `LenX` hasn't been defined, `LenX` essentially just represents the length of `L2` which is the number of `x` pieces on the game board. Finally, it is evaluated whether or not `LenO` is greater than `LenX`. If it is, the query returns `true` because all conditions have been satisfied, meaning the `o` player has more pieces than the `x` oppenent and therefore the `o` player is winning. Otherwise it returns `false` because `LenO > lenX` will not evaluate to `true` because the `o` player either has less than or equal to the number of pieces that the `x` opponent has, and thus the `o` player is not winning. 

### Question 12

Write a query which will tell you whether or not the game is won by any player.

```
?- \+ (findall(A, o(A), L1), findall(B, x(B), L2), length(L1, LenO), length(L2, LenX), LenO > 0, LenX > 0, x(C), (valid_move(C, _); valid_capture(C, _)), o(F), (valid_move(F, _); valid_capture(F, _))).
```

###### Explanation

This query will return `true` if one player has no game pieces left, or if one of the players can no longer make any moves, which by the prompt would cause the game to be won by a player. The way that this query works is by using the query from question 11 except instead of comparing the length of the two lists to each other, comparing them both to 0 because if both lists have a length greater than 0, both teams have at least 1 piece left and can therefore still have not lost. The next thing that the query does is evaluate the existence of a variable on the team x which is represented by `C`. Next, the `valid_move` or `valid_capture` predicate is called with `C` as the game piece and an irrelevant target cell. I edited the `valid_move` and `valid_capture` predicates in questions 4 and 6 to allow moving "backwards" across the board (in relation to the player) to be valid for kinged o pieces or x pieces. I also edited `valid_move` and `valid_capture` going "forwards" across the board to be valid for o pieces or kinged x pieces. If one of these is valid, it validates that the x team has a move that they can make. Finally, the existence of a piece on team o is validated and represented as the variable `F`, and the `valid_move` or `valid_capture` predicate is called with `F` as the game piece and an irrelevant target cell. If one of these is valid, it validates that the o team has a move that they can make. This covers all the conditions that are required in order for the game to continue. Because of the `\+` operator at the beginning of the expression, if the whole expression evaluates true, the actual outcome of the query is `false` and therefore no team has won. Otherwise, the result of the query is `true` and there is a winner based on the current state.