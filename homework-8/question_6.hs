question_6 :: (Eq a, Num a) => [a] -> a
question_6 [] = -1
question_6 [x] = 0
question_6 (x:xs) = findEquilibrium x 1 xs

findEquilibrium total index (x:xs)
  | total == sum xs = index
  | length xs == 0 = -1
  | otherwise = findEquilibrium (total + x) (index + 1) xs