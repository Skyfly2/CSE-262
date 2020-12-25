(in-ns 'homework-5.core)

; --------------------------------
; Question 9
; --------------------------------
;
; struct point {
;   x: u32,
;   y: u32,
; }
;
; fn main() {
;  let p = Point {x: 10, y: 20};
;  let question_9 = p.x;
; }

(defstruct point :x :y)
(def p (struct point 10 10))
(def question-9 
    (:x p)
)