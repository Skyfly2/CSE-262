(in-ns 'homework-5.core)

; --------------------------------
; Question 5
; --------------------------------
; fn main() {
;   let s = "hello";
;   let question_5 = if s.len() >= 5 && s.get(0..5).unwrap() == "hello" {
;     1 
;   } else {
;     2
;   };
; }

(def s "hello")
(def question-5
    (if (= (count s) 5) (if (= s "hello") 1 2) 2)
)