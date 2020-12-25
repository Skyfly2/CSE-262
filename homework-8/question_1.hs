question_1 :: (Eq a, Num a) => [a] -> a -> [(a, a)]
question_1 [] target = []
question_1 (x:xs) target = 
  let curr = [(x, y) | y <- xs, x + y == target]
  in curr ++ question_1 xs target
