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
    abstract function '+' :> DataFunctions::'+' {
        in x : Real[1];
        in y : Real[0..1];
        return : Real[1];
    }
    function sum :> ComplexFunctions::sum {
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
    (reference r1 (scope relative) (span (offset 172) (line 5) (column 39) (len 18)) (segments (segment 0 (token "DataFunctions") (name "DataFunctions") (separator none) (span (offset 172) (line 5) (column 39) (len 13))) (segment 1 (token "'+'") (name "+") (separator colon-colon) (span (offset 187) (line 5) (column 54) (len 3)))))
    (reference r2 (scope relative) (span (offset 275) (line 6) (column 30) (len 21)) (segments (segment 0 (token "ComplexFunctions") (name "ComplexFunctions") (separator none) (span (offset 275) (line 6) (column 30) (len 16))) (segment 1 (token "sum") (name "sum") (separator colon-colon) (span (offset 293) (line 6) (column 48) (len 3)))))
  )
  (root (library-package (name "RealFunctions") (standard true) (body (kerml-classifier (keyword function) (abstract false) (name "re") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0))))) (kerml-classifier (keyword function) (abstract true) (name "+") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1))))) (kerml-classifier (keyword function) (abstract false) (name "sum") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r2))))) (kerml-classifier (keyword function) (abstract false) (name "includes") (specializes none)))))
)
~~~
