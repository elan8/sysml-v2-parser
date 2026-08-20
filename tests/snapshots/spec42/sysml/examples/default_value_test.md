# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Simple Tests): DefaultValueTest"))
~~~
# SOURCE
~~~sysml
package DefaultValueTest {
	
	part def V {
		attribute m default = 10;
		attribute n = 20;
	}
	
	part v1 : V {
		attribute :>> m = 20;
	}
	
	part def W :> V {
		attribute :>> m default = n;
	}
	
	part v2 = new W();
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "default_value_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package DefaultValueTest {
    part def V {
        attribute m default = 10;
        attribute n = 20;
    }
    part v1 : V {
        attribute :>> m = 20;
    }
    part def W :> V {
        attribute :>> m default = n;
    }
    part v2 = new W();
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 107) (line 8) (column 12) (len 1)) (segments (segment 0 (token "V") (name "V") (separator none) (span (offset 107) (line 8) (column 12) (len 1)))))
    (reference r1 (scope relative) (span (offset 127) (line 9) (column 17) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 127) (line 9) (column 17) (len 1)))))
    (reference r2 (scope relative) (span (offset 175) (line 13) (column 17) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 175) (line 13) (column 17) (len 1)))))
    (reference r3 (scope relative) (span (offset 187) (line 13) (column 29) (len 1)) (segments (segment 0 (token "n") (name "n") (separator none) (span (offset 187) (line 13) (column 29) (len 1)))))
    (reference r4 (scope relative) (span (offset 210) (line 16) (column 16) (len 1)) (segments (segment 0 (token "W") (name "W") (separator none) (span (offset 210) (line 16) (column 16) (len 1)))))
  )
  (root (package (name "DefaultValueTest") (body brace (part-def (name "V") (modifiers) (body brace (attribute-usage (declaration-name "m") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 67) (line 4) (column 25) (len 2)) (integer 10))))) (body semicolon)) (attribute-usage (declaration-name "n") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 87) (line 5) (column 17) (len 2)) (integer 20))))) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "v1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 131) (line 9) (column 21) (len 2)) (integer 20))))) (body semicolon)))) (part-def (name "W") (modifiers) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 187) (line 13) (column 29) (len 1)) (ref r3))))) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "v2") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 206) (line 16) (column 12) (len 7)) (constructor (type (ref r4)) (arguments)))))) (body semicolon)))))
)
~~~
