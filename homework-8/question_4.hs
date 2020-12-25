question_4 [[]] = []
question_4 [[x]] = [x]
question_4 matrix = greedySolve matrix 0 0

greedySolve matrix currCol currRow =
  if (currCol == (length (matrix!!0)) - 1 && currRow == (length (matrix)) - 1)
    then [matrix!!currRow!!currCol]
    else if (currRow == 0)
            then if (currCol == 0)
                    then if (matrix!!currCol!!(currRow + 1) > matrix!!(currCol + 1)!!currRow)
                            then [matrix!!(currCol + 1)!!currRow] ++ greedySolve matrix (currCol + 1) currRow
                            else [matrix!!currCol!!(currRow + 1)] ++ greedySolve matrix currCol (currRow + 1)
                    else if (matrix!!currCol!!(currRow + 1) > matrix!!(currCol + 1)!!currRow)
                            then [matrix!!(currCol + 1)!!currRow] ++ greedySolve matrix (currCol + 1) currRow
                            else [matrix!!currCol!!(currRow + 1)] ++ greedySolve matrix currCol (currRow + 1)
            else if (currRow == (length (matrix)) - 1)
                    then [matrix!!(currCol + 1)!!currRow] ++ greedySolve matrix (currCol + 1) currRow
                    else if (currCol == (length (matrix!!0)) - 1)
                            then [matrix!!currCol!!(currRow + 1)] ++ greedySolve matrix currCol (currRow + 1)
                            else if (matrix!!currCol!!(currRow + 1) > matrix!!(currCol + 1)!!currRow)
                                then [matrix!!(currCol + 1)!!currRow] ++ greedySolve matrix (currCol + 1) currRow
                                else [matrix!!currCol!!(currRow + 1)] ++ greedySolve matrix currCol (currRow + 1)

