question_8 :: (Ord a) => [a] -> [a]
question_8 [] = []
question_8 [x] = [x]
question_8 x = merge (question_8 (fst (split x))) (question_8 (snd (split x)))

merge [] [] = []
merge x [] = x
merge [] y = y
merge (x:xs) (y:ys)
  | x < y = [x] ++ merge xs (y:ys)
  | otherwise = [y] ++ merge (x:xs) ys

split [] = ([], [])
split [x] = ([x], [])
split x = (take ((length x) `div` 2) x, drop ((length x) `div` 2) x)