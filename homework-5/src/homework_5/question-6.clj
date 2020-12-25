(in-ns 'homework-5.core)

; --------------------------------
; Question 6
; --------------------------------
; fn main() {
;   let s = String::from("hello0123!");
;   let mut z = 0;
;   for c in s.chars() { 
;     match c {
;       'a'...'z' => z += 1,
;       '0'...'9' => z += 2,
;       _ => z += 3,
;     };
;   }
;   let question_6 = z;
; }



(def s "hello0123!")
(def z (atom 0))
(def characters #{"a" "b" "c" "d" "e" "f" "g" "h" "i" "j" "k" "l" "m" "n" "o" "p" "q" "r" "s" "t" "u" "v" "w" "x" "y" "z"})
(def nums #{"1" "2" "3" "4" "5" "6" "7" "8" "9" "0"})
(defn traverse []
    (loop [c 0]
        (when (< c (count s))
            (def curr (nth s c))
            (def co (atom 0))
            (println @z)
            (if (contains? characters (str curr)) 
                (swap! z inc) 
            (if (contains? nums (str curr))
                (while ( < @co 2)
                (do
                    (swap! z inc)
                    (swap! co inc)))
                (while ( < @co 3)
                (do
                    (swap! z inc)
                    (swap! co inc)))
                )
            )
            (recur (inc c))
        )
    )
)
(traverse)
(def question-6 @z)
