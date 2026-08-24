# META
~~~sexpr
(snapshot (type semantic) (description "KerML function declarations parse as structured classifier nodes with abstract prefixes, quoted operator names, specializes clauses, typed parameters with multiplicities, kinded expr/bool/feature parameters, and return results with multiplicity properties and default values."))
~~~
# SOURCE
~~~sysml
standard library package RealFunctions {
    function re :> ComplexFunctions::re { in x: Real[1];
        return : Real[1] = x;
    }
    abstract function '+' specializes DataFunctions::'+' { in x: Real[1]; in y: Real[0..1]; return : Real[1]; }
    function sum specializes ComplexFunctions::sum { in collection: Real[0..*];
        return : Real default NumericalFunctions::sum0(collection, 0.0);
    }
    function includes { in seq: Anything[0..*] ordered nonunique;
        in expr test[0..*] { in argument: Anything[1]; return : Boolean[1]; }
        return : Anything[0..*] ordered nonunique;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "function_declarations.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package RealFunctions {
    function re :> ComplexFunctions::re {
        in x : Real[1];
        return : Real[1] = x;
    }
    abstract function '+' specializes DataFunctions::'+' {
        in x : Real[1];
        in y : Real[0..1];
        return : Real[1];
    }
    function sum specializes ComplexFunctions::sum {
        in collection : Real[0..*];
        return : Real default NumericalFunctions::sum0(collection, 0.0);
    }
    function includes {
        in seq : Anything[0..*] ordered nonunique;
        in expr test[0..*] {
            in argument : Anything[1];
            return : Boolean[1];
        }
        return : Anything[0..*] ordered nonunique;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 60) (line 2) (column 20) (len 20)) (segments (segment 0 (token "ComplexFunctions") (name "ComplexFunctions") (separator none) (span (offset 60) (line 2) (column 20) (len 16))) (segment 1 (token "re") (name "re") (separator colon-colon) (span (offset 78) (line 2) (column 38) (len 2)))))
    (reference r1 (scope relative) (span (offset 89) (line 2) (column 49) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 89) (line 2) (column 49) (len 4)))))
    (reference r2 (scope relative) (span (offset 172) (line 5) (column 39) (len 18)) (segments (segment 0 (token "DataFunctions") (name "DataFunctions") (separator none) (span (offset 172) (line 5) (column 39) (len 13))) (segment 1 (token "'+'") (name "+") (separator colon-colon) (span (offset 187) (line 5) (column 54) (len 3)))))
    (reference r3 (scope relative) (span (offset 199) (line 5) (column 66) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 199) (line 5) (column 66) (len 4)))))
    (reference r4 (scope relative) (span (offset 214) (line 5) (column 81) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 214) (line 5) (column 81) (len 4)))))
    (reference r5 (scope relative) (span (offset 275) (line 6) (column 30) (len 21)) (segments (segment 0 (token "ComplexFunctions") (name "ComplexFunctions") (separator none) (span (offset 275) (line 6) (column 30) (len 16))) (segment 1 (token "sum") (name "sum") (separator colon-colon) (span (offset 293) (line 6) (column 48) (len 3)))))
    (reference r6 (scope relative) (span (offset 314) (line 6) (column 69) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 314) (line 6) (column 69) (len 4)))))
    (reference r7 (scope relative) (span (offset 437) (line 9) (column 33) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 437) (line 9) (column 33) (len 8)))))
    (reference r8 (scope relative) (span (offset 513) (line 10) (column 43) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 513) (line 10) (column 43) (len 8)))))
  )
  (root (library-package (name "RealFunctions") (standard true) (body brace (kerml-classifier (keyword function) (abstract false) (name "re") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "x") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity (lower (expression (span (offset 94) (line 2) (column 54) (len 1)) (integer 1))) (upper (expression (span (offset 94) (line 2) (column 54) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (return-declaration (name none) (short-name none)))) (kerml-classifier (keyword function) (abstract true) (name "+") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r2)))) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "x") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity (lower (expression (span (offset 204) (line 5) (column 71) (len 1)) (integer 1))) (upper (expression (span (offset 204) (line 5) (column 71) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "y") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity (lower (expression (span (offset 219) (line 5) (column 86) (len 1)) (integer 0))) (upper (expression (span (offset 222) (line 5) (column 89) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (return-declaration (name none) (short-name none)))) (kerml-classifier (keyword function) (abstract false) (name "sum") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r5)))) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "collection") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity (lower (expression (span (offset 319) (line 6) (column 74) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (return-declaration (name none) (short-name none)))) (kerml-classifier (keyword function) (abstract false) (name "includes") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "seq") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity (lower (expression (span (offset 446) (line 9) (column 42) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering ordered) (uniqueness nonunique)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind expr) (member false) (all false) (name "test") (typing none) (multiplicity (lower (expression (span (offset 492) (line 10) (column 22) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body brace (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "argument") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity (lower (expression (span (offset 522) (line 10) (column 52) (len 1)) (integer 1))) (upper (expression (span (offset 522) (line 10) (column 52) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (return-declaration (name none) (short-name none)))) (return-declaration (name none) (short-name none)))))))
)
~~~
