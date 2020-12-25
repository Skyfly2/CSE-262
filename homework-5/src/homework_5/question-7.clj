(in-ns 'homework-5.core)

; --------------------------------
; Question 7
; --------------------------------
; fn add_two(x: u16, y: u16) -> u16 {
;   x + y 
; }
; 
; fn main() {
;   let question_7 = add_two(4,5);
; }
(defn add_two [num1, num2]
    (+ num1 num2)
)
(def question-7 
    (add_two 4 5)
)