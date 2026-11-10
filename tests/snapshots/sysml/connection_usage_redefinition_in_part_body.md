# META
~~~sexpr
(snapshot (type semantic) (description "A connection usage inside a part usage body may redefine an inherited connection with a multiplicity and a body, anonymously or named."))
~~~
# SOURCE
~~~sysml
package ConnectionUsageRedefinition {
    part def Bus;
    connection def Line;
    part def Grid {
        part buses : Bus[0..*];
        connection lines : Line[0..*];
    }
    part g : Grid {
        connection :>> lines[3] {
            attribute :>> aIndex = 1;
        }
        connection named :>> lines[2];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "connection_usage_redefinition_in_part_body.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ConnectionUsageRedefinition {
    part def Bus;
    connection def Line;
    part def Grid {
        part buses : Bus[0..*];
        connection lines : Line[0..*];
    }
    part g : Grid {
        connection[3] :>> lines {
            attribute :>> aIndex = 1;
        }
        connection named[2] :>> lines;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 122) (line 5) (column 22) (len 3)) (segments (segment 0 (token "Bus") (name "Bus") (separator none) (span (offset 122) (line 5) (column 22) (len 3)))))
    (reference r1 (scope relative) (span (offset 160) (line 6) (column 28) (len 4)) (segments (segment 0 (token "Line") (name "Line") (separator none) (span (offset 160) (line 6) (column 28) (len 4)))))
    (reference r2 (scope relative) (span (offset 191) (line 8) (column 14) (len 4)) (segments (segment 0 (token "Grid") (name "Grid") (separator none) (span (offset 191) (line 8) (column 14) (len 4)))))
    (reference r3 (scope relative) (span (offset 221) (line 9) (column 24) (len 5)) (segments (segment 0 (token "lines") (name "lines") (separator none) (span (offset 221) (line 9) (column 24) (len 5)))))
    (reference r4 (scope relative) (span (offset 309) (line 12) (column 30) (len 5)) (segments (segment 0 (token "lines") (name "lines") (separator none) (span (offset 309) (line 12) (column 30) (len 5)))))
  )
  (root (package (visibility none) (name "ConnectionUsageRedefinition") (body brace (part-def (name "Bus") (modifiers) (body semicolon)) (connection-def (name "Line") (modifiers) (extensions) (specializes none) (body semicolon)) (part-def (name "Grid") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "buses") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity (lower (expression (span (offset 126) (line 5) (column 26) (len 1)) (integer 0))) (upper (expression (span (offset 129) (line 5) (column 29) (len 1)) (infinity)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "lines") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity (lower (expression (span (offset 165) (line 6) (column 33) (len 1)) (integer 0))) (upper (expression (span (offset 168) (line 6) (column 36) (len 1)) (infinity)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "g") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower none) (upper (expression (span (offset 227) (line 9) (column 30) (len 1)) (integer 3)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (references none) (crosses none) (intersects none) (value none) (connect) (body brace (attribute-usage))) (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "named") (short-name none) (typing none) (multiplicity (lower none) (upper (expression (span (offset 315) (line 12) (column 36) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value none) (connect) (body semicolon)))))))
)
~~~
