(in-ns 'homework-5.core)

; --------------------------------
; Question 11
; --------------------------------
; You must do this queastion with map
; fn main() {
;   let x = vec![1, 2, 3];
;   let y: Vec<i32> = x.iter().map(|x| x * 2).collect();
;   let question_11 = y[1];
; }

(def x [1 2 3])
(def y
    (map (partial * 2) x)
)
(def question-11 
    (nth y 1)
)