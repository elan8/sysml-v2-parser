# META
~~~sexpr
(snapshot (type recovery) (description "The result-expression arm of calculation, constraint and KerML type bodies models only an OwnedExpression. A member opening, after its optional MemberPrefix, with a reserved keyword that no arm of the scope models is recovered as one node with a stable diagnostic instead of being shredded into stray result expressions; the valid result expression that follows still parses."))
~~~
# SOURCE
~~~sysml
package FallbackRecovery {
    constraint def C {
        objective o;
        private objective p;
        x < 10
    }
    calc def K {
        objective o;
        1
    }
    class Kc {
        objective o;
        1
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "result_expression_fallback_recovery.md"
    (diagnostics
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 58) (line 3) (column 9) (len 21)) (message "unexpected keyword `objective` in constraint body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 79) (line 4) (column 9) (len 29)) (message "unexpected keyword `private` in constraint body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 146) (line 8) (column 9) (len 21)) (message "unexpected keyword `objective` in calc body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 198) (line 12) (column 9) (len 21)) (message "unexpected keyword `objective` in calc body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package FallbackRecovery {
    constraint def C {
        objective o;
        private objective p;
        x < 10;
    }
    calc def K {
        objective o;
        1;
    }
    class Kc {
        objective o;
        1;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 108) (line 5) (column 9) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 108) (line 5) (column 9) (len 1)))))
  )
  (root (package (name "FallbackRecovery") (body brace (constraint-def (name "C") (modifiers) (specializes none) (body brace (malformed (code "unexpected_keyword_in_scope") (found "objective o;") (span (offset 58) (line 3) (column 9) (len 21))) (malformed (code "unexpected_keyword_in_scope") (found "private objective p;") (span (offset 79) (line 4) (column 9) (len 29))) (expression (span (offset 108) (line 5) (column 9) (len 6)) (binary (operator "<") (left (expression (span (offset 108) (line 5) (column 9) (len 1)) (ref r0))) (right (expression (span (offset 112) (line 5) (column 13) (len 2)) (integer 10))))))) (calc-def (name "K") (modifiers) (body brace (malformed (code "unexpected_keyword_in_scope") (found "objective o;") (span (offset 146) (line 8) (column 9) (len 21))) (expression (expression (span (offset 167) (line 9) (column 9) (len 1)) (integer 1))))) (kerml-classifier (keyword class) (abstract false) (name "Kc") (multiplicity none) (specializes none) (conjugates none) (body brace (malformed (code "unexpected_keyword_in_scope") (found "objective o;") (span (offset 198) (line 12) (column 9) (len 21))) (expression (expression (span (offset 219) (line 13) (column 9) (len 1)) (integer 1))))))))
)
~~~
