question_3 :: [Char] -> Int
question_3 str =
  let time = getVals str
      hourAngle = fixAngle (((time!!0 / 12.0) * 360) + ((time!!1 / 60.0) * 30))
      minuteAngle = fixAngle ((time!!1 / 60.0) * 360)
  in round (calcAngle hourAngle minuteAngle)::Int

getVals str =
  let res = [x | x <- str, x /= ':']
  in reader res

reader str =
  if length str == 4
    then [read (take 2 str)::Double, read (reverse (take 2 (reverse str)))::Double]
    else [read (take 1 str)::Double, read (reverse (take 2 (reverse str)))::Double]

calcAngle hourAngle minuteAngle =
  if hourAngle > minuteAngle
    then adjustAngle (hourAngle - minuteAngle)
    else adjustAngle (minuteAngle - hourAngle)

adjustAngle angle =
  if angle > 180
    then (360 - angle)
    else angle

fixAngle angle =
  if angle >= 360
    then (angle - 360)
    else angle