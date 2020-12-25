(in-ns 'homework-5.core)

; --------------------------------
; Question 10
; --------------------------------
; fn main() {
;   let x = vec![1 2 3];
;   let y = x.len();
;   let question_10 = x[2] + y;
; }
(def x [1 2 3])
(def y 
    (count x)
)
(def question-10 
    (+ (nth x 2) y)
)