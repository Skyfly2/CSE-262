(in-ns 'homework-5.core)

; --------------------------------
; Question 3
; --------------------------------
; fn main() {
;    let x = 20;
;    let question_3 = if x == 10 {
;      1
;    } else if x > 10 {
;      2
;    } else {
;      3
;    };
; }

(def x 20)
(def question-3 
    (if (= x 10) 1 (if (> x 10) 2 3))
)