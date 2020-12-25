(in-ns 'homework-5.core)

; --------------------------------
; Question 2
; --------------------------------
; fn main() {
;    let x = 10;
;    let question_2 = if x == 10 {
;        1
;    } else {
;        2
;    };
; }

(def x 10)
(def question-2 
    (if (= x 10) 1 2)
)