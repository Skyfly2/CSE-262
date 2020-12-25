question_7 :: [Char] -> [Char]
question_7 [] = ""
question_7 [x] = [x]
question_7 str
  | True == ((first (containsDuplicates str ' ' False 0 0 0)) == ' ') = str
  | otherwise = question_7 (remove (first (containsDuplicates str ' ' False 0 0 0)) str (second (containsDuplicates str ' ' False 0 0 0)) (third (containsDuplicates str ' ' False 0 0 0)) 0)


containsDuplicates str lastchar started start end count =
  if count == length str
    then (lastchar, start, count - 1)
  else if started == False
    then if str!!count == lastchar
          then containsDuplicates str (str!!count) True (count - 1) 0 (count + 1)
          else if count == (length str) - 1
                then (' ', 0,0)
                else containsDuplicates str (str!!count) False 0 0 (count + 1)
    else if str!!count == lastchar
          then containsDuplicates str lastchar True start 0 (count + 1)
          else if count == (length str) - 1
                then (lastchar, start, count - 1)
                else (lastchar, start, count - 1)

remove y [] start end count = []
remove y (x:xs) start end count
  | end == 0 = (x:xs)
  | count > end = (x:xs)
  | count < start = x : remove y xs start end (count + 1)
  | y == x = remove y xs start end (count + 1)
  | otherwise = x : remove y xs start end (count + 1)

first (a,_,_) = a
second (_,a,_) = a
third (_,_,a) = a