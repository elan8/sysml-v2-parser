# META
~~~sexpr
(snapshot (type recovery) (description "Part-definition recovery synchronizes at the full FIRST spellings of nested Package and LibraryPackage. A malformed member before a standard library package must retain that package and the later typed sibling (SysML textual BNF 180-207 and 234-248; pinned Pilot SysML agrees)."))
~~~
# SOURCE
~~~sysml
part def Container {
    nonsense ???;
    standard library package Recovered {
        part component : Component;
    }
    part later : Later;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "part_def_nested_packages_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 25) (line 2) (column 5) (len 18)) (message "unrecognized declaration `nonsense` in part definition body"))
    )
  )
)
~~~
# FORMAT
~~~sexpr
(stable-idempotent)
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 105) (line 4) (column 26) (len 9)) (segments (segment 0 (token "Component") (name "Component") (separator none) (span (offset 105) (line 4) (column 26) (len 9)))))
    (reference r1 (scope relative) (span (offset 139) (line 6) (column 18) (len 5)) (segments (segment 0 (token "Later") (name "Later") (separator none) (span (offset 139) (line 6) (column 18) (len 5)))))
  )
  (root (part-def (name "Container") (modifiers) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "nonsense ???;") (span (offset 25) (line 2) (column 5) (len 18))) (library-package (name "Recovered") (standard true) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "component") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "later") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))
)
~~~
