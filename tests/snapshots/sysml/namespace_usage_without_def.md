# META
~~~sexpr
(snapshot (type semantic) (description "A definition keyword is `<kind> def` (CalculationDefKeyword, InterfaceDefKeyword, ConnectionDefKeyword), so at namespace level `connection X;`, `interface X;` and `calc X;` without `def` are ConnectionUsage, InterfaceUsage and CalculationUsage, including the Systems Library usage forms with multiplicity, `nonunique` and subsetting; the `def` forms stay definitions."))
~~~
# SOURCE
~~~sysml
package NamespaceUsages {
    connection def CD;
    interface def ID;
    calc def KD;
    connection ConnectionUsage;
    interface InterfaceUsage;
    calc CalculationUsage;
    interface typed : ID;
    calc typedCalc : KD;
    abstract interface interfaces : ID[0..*] nonunique :> connections {
        doc /* library form */
    }
    abstract calc calculations : KD[0..*] nonunique :> actions, evaluations {
        doc /* library form */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "namespace_usage_without_def.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package NamespaceUsages {
    connection def CD;
    interface def ID;
    calc def KD;
    connection ConnectionUsage;
    interface InterfaceUsage;
    calc CalculationUsage;
    interface typed : ID;
    calc typedCalc : KD;
    abstract interface interfaces : ID[0..*] nonunique :> connections {
        doc
        /* library form */
    }
    abstract calc calculations : KD[0..*] nonunique :> actions, evaluations {
        doc
        /* library form */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 199) (line 8) (column 23) (len 2)) (segments (segment 0 (token "ID") (name "ID") (separator none) (span (offset 199) (line 8) (column 23) (len 2)))))
    (reference r1 (scope relative) (span (offset 224) (line 9) (column 22) (len 2)) (segments (segment 0 (token "KD") (name "KD") (separator none) (span (offset 224) (line 9) (column 22) (len 2)))))
    (reference r2 (scope relative) (span (offset 264) (line 10) (column 37) (len 2)) (segments (segment 0 (token "ID") (name "ID") (separator none) (span (offset 264) (line 10) (column 37) (len 2)))))
    (reference r3 (scope relative) (span (offset 370) (line 13) (column 34) (len 2)) (segments (segment 0 (token "KD") (name "KD") (separator none) (span (offset 370) (line 13) (column 34) (len 2)))))
    (reference r4 (scope relative) (span (offset 392) (line 13) (column 56) (len 7)) (segments (segment 0 (token "actions") (name "actions") (separator none) (span (offset 392) (line 13) (column 56) (len 7)))))
    (reference r5 (scope relative) (span (offset 401) (line 13) (column 65) (len 11)) (segments (segment 0 (token "evaluations") (name "evaluations") (separator none) (span (offset 401) (line 13) (column 65) (len 11)))))
  )
  (root (package (name "NamespaceUsages") (body brace (connection-def (name "CD") (modifiers) (extensions) (specializes none) (body semicolon)) (interface-def (name "ID") (modifiers) (specializes none) (body semicolon)) (calc-def (name "KD") (modifiers) (body semicolon)) (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "ConnectionUsage") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect) (body semicolon)) (interface-usage (form declaration) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "InterfaceUsage") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (part none) (body semicolon)) (calc-usage (name "CalculationUsage") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (interface-usage (form declaration) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "typed") (short-name none) (type (ref r0)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (part none) (body semicolon)) (calc-usage (name "typedCalc") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (interface-usage (form declaration) (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "interfaces") (short-name none) (type (ref r2)) (multiplicity (lower (expression (span (offset 267) (line 10) (column 40) (len 1)) (integer 0))) (upper (expression (span (offset 270) (line 10) (column 43) (len 1)) (infinity)))) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (part none) (body brace (doc (name none) (locale none) (body (span (offset 314) (line 11) (column 15) (len 14)) (normalized "library form "))))) (calc-usage (name "calculations") (short-name none) (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity (lower (expression (span (offset 373) (line 13) (column 37) (len 1)) (integer 0))) (upper (expression (span (offset 376) (line 13) (column 40) (len 1)) (infinity)))) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (subsets (relationship (kind subsets) (implied false) (targets (ref r4) (ref r5)))) (redefines none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 429) (line 14) (column 15) (len 14)) (normalized "library form "))))))))
)
~~~
