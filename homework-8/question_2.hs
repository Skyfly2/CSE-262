question_2 :: Ord a => [a] -> [a]
question_2 [] = []
question_2 [x] = []
question_2 (x:xs)
  | x > maximum xs = [x] ++ question_2 xs
  | otherwise = question_2 xs
  
