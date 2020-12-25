(in-ns 'homework-5.core)

; --------------------------------
; Question 12
; --------------------------------
; You must do this question with reduce
; fn main() {
;   let x = vec![1, 2, 3];
;   let y: i32 = x.iter().fold(0, |acc, i| acc * i );
; }

(def x [1 2 3])
(def question-12 
    (reduce * x)
)