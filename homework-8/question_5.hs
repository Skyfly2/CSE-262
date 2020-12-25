question_5 :: (Eq a, Num a) => [[a]] -> [[a]] -> [[a]]
question_5 m1 m2 = 
  if (length' (m1!!0)) /= (length' (m2))
    then [[]]
    else reformat (length' (m2!!0)) (multiply (length' (m1!!0)) (adjustMatrix1 m1) (adjustMatrix2 m2 0 0 (length' (m2!!0)) (length' m2)) (adjustMatrix2 m2 0 0 (length' (m2!!0)) (length' m2)))

adjustMatrix1 (x:xs)
  | length' xs == 0 = x
  | otherwise = x ++ adjustMatrix1 xs
 
adjustMatrix2 matrix row col lrow lcol
  | row < lcol = [matrix!!row!!col] ++ adjustMatrix2 matrix (row + 1) col lrow lcol
  | col < (lrow - 1) = adjustMatrix2 matrix 0 (col + 1) lrow lcol 
  | otherwise = []

multiply len rows cols ocols
  | take' len rows == [] = []
  | take' len cols == [] = multiply len (drop' len rows) ocols ocols
  | otherwise = [sum' (multiplyLists (take' len rows) (take' len cols))] ++ multiply len rows (drop' len cols) ocols

reformat len result
  | take' len result == [] = []
  | otherwise = [take' len result] ++ reformat len (drop' len result) 

length' [] = 0
length' [x] = 1
length' (x:xs) = 1 + (length' xs)

sum' [] = 0
sum' [x] = x
sum' (x:xs) = x + sum' xs

take' num [] = []
take' num (x:xs) = gather num 0 (x:xs) (x:xs)
gather num index [] [] = []
gather num index [] (y:ys) = []
gather num index (x:xs) [] = []
gather num index (x:xs) (y:ys)
  | (length' (y:ys)) == index = take' num []
  | index < num = [x] ++ gather num (index + 1) xs (y:ys)
  | otherwise = []

drop' num [] = []
drop' num (x:xs) = remove num 0 (x:xs) (x:xs)
remove num index [] [] = []
remove num index [] (y:ys) = []
remove num index (x:xs) [] = []
remove num index (x:xs) (y:ys)
  | (length' (y:ys)) == index = drop' num []
  | index < (num - 1) = remove num (index + 1) xs (y:ys)
  | otherwise = xs

multiplyLists [] [] = []
multiplyLists [] (y:ys) = []
multiplyLists (x:xs) [] = []
multiplyLists (x:xs) (y:ys) = [x * y] ++ multiplyLists xs ys