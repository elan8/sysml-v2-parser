# META
~~~sexpr
(snapshot (type malformed) (description "An intersects clause is a KerML feature specialization that SysML's ViewUsage declaration does not admit: the member is refused with a diagnostic instead of being retained, and the valid view usages after it survive at package level and in a view body."))
~~~
# SOURCE
~~~sysml
package ViewDeclarationMalformed {
    view w;
    view intersecting intersects w;
    view retainedInPackage;
    view outer {
        view intersecting intersects w;
        view retainedInView;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "view_usage_declaration_malformed.md"
    (diagnostics
      (diagnostic (code "missing_body_or_semicolon") (severity error) (category parseerror) (span (offset 51) (line 3) (column 5) (len 36)) (message "expected ';' or '{' after view declaration header"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 51) (line 3) (column 5) (len 36)) (message "suppressed 1 cascading missing_body_or_semicolon diagnostic after earlier recovery errors"))
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
  )
  (root (package (name "ViewDeclarationMalformed") (body brace (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "w") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)) (malformed (code "missing_body_or_semicolon") (found "view intersecting intersects w;") (span (offset 51) (line 3) (column 5) (len 36))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "retainedInPackage") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "outer") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body brace (malformed (code "missing_body_or_semicolon") (found "view intersecting intersects w;") (span (offset 136) (line 6) (column 9) (len 40))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "retainedInView") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)))))))
)
~~~
