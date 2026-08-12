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
        attribute  :>> m = 20;
    }
    part def W :> V {
        attribute  :>> m default = n;
    }
    part v2 = new W();
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 175) (line 13) (column 17) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 175) (line 13) (column 17) (len 1)))))
    (reference r1 (scope relative) (span (offset 187) (line 13) (column 29) (len 1)) (segments (segment 0 (token "n") (name "n") (separator none) (span (offset 187) (line 13) (column 29) (len 1)))))
  )
  (root (package (name "DefaultValueTest") (body (part-def (name "V") (body (attribute-usage (declaration-name "m") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 67) (line 4) (column 25) (len 2)) (integer 10))))) (body semicolon)) (attribute-usage (declaration-name "n") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 87) (line 5) (column 17) (len 2)) (integer 20))))) (body semicolon)))) (part-usage) (part-def (name "W") (body (attribute-usage (declaration-name none) (direction none) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 187) (line 13) (column 29) (len 1)) (ref r1))))) (body semicolon)))) (part-usage))))
)
~~~
