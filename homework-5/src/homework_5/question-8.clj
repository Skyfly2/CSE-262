(in-ns 'homework-5.core)

; --------------------------------
; Question 8
; --------------------------------
; fn main() {
;   let h = |i: i32| -> i32 { i + 1 }; // An anonymous function
;   let question_8 = double_result(h);
; }
;
; // This function takes a function that takes an i32 and returns an i32, then passes 
; // it the number 1 and doubles it
; fn double_result<F>(y: F) -> i32
;    where F: Fn(i32) -> i32 {
;    2 * y(1)
; }

(def h 
    (fn [i] (+ i 1))
)
(defn double_result [y]
    (* 2 (y 1))
)
(def question-8 
    (double_result h)
)