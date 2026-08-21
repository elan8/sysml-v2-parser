# META
~~~sexpr
(snapshot (type semantic) (description "KerML TypeBody routes directed keyword-less Feature declarations through FeaturePrefix rather than the SysML action-parameter node. The fixture retains all three FeatureDirection spellings, typing, multiplicity modifiers, a value, and anonymous redefinition while the companion calculation body continues to use its ActionBody-owned directed parameter form."))
~~~
# SOURCE
~~~sysml
package DirectedKeywordlessFeatures {
    behavior TypeBodyOwner {
        in input : Input[1];
        out output [0..*] nonunique;
        inout shared = source;
        in :>> inherited;
    }
    calc def SysMLCalculationOwner {
        in calculationParameter : Input;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "directed_keywordless_features.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package DirectedKeywordlessFeatures {
    behavior TypeBodyOwner {
        in input : Input[1];
        out output[0..*] nonunique;
        inout shared = source;
        in :>> inherited;
    }
    calc def SysMLCalculationOwner {
        in calculationParameter : Input;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 86) (line 3) (column 20) (len 5)) (segments (segment 0 (token "Input") (name "Input") (separator none) (span (offset 86) (line 3) (column 20) (len 5)))))
    (reference r1 (scope relative) (span (offset 156) (line 5) (column 24) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 156) (line 5) (column 24) (len 6)))))
    (reference r2 (scope relative) (span (offset 179) (line 6) (column 16) (len 9)) (segments (segment 0 (token "inherited") (name "inherited") (separator none) (span (offset 179) (line 6) (column 16) (len 9)))))
    (reference r3 (scope relative) (span (offset 267) (line 9) (column 35) (len 5)) (segments (segment 0 (token "Input") (name "Input") (separator none) (span (offset 267) (line 9) (column 35) (len 5)))))
  )
  (root (package (name "DirectedKeywordlessFeatures") (body brace (kerml-classifier (keyword behavior) (abstract false) (name "TypeBodyOwner") (specializes none) (body brace (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "input") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity (lower (expression (span (offset 92) (line 3) (column 26) (len 1)) (integer 1))) (upper (expression (span (offset 92) (line 3) (column 26) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction out) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "output") (typing none) (multiplicity (lower (expression (span (offset 116) (line 4) (column 21) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction inout) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "shared") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 156) (line 5) (column 24) (len 6)) (ref r1))))) (body semicolon)) (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (references none) (crosses none) (relationships) (value none) (body semicolon)))) (calc-def (name "SysMLCalculationOwner") (modifiers) (body brace (in-out (direction in) (reference false) (declaration "calculationParameter") (subsets none) (type (ref r3)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 241) (line 9) (column 9) (len 32))))))))
)
~~~
