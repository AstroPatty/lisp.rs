(defmacro > (&rest body)
  `(< ,@(reverse body)))
