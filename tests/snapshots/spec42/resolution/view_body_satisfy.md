# META
~~~sexpr
(snapshot (type semantic) (description "View-body satisfy endpoint resolution coverage"))
~~~
# SOURCE
~~~sysml
package ViewCoverage {
    viewpoint def ArchitectureViewpoint;
    view def ArchitectureView;
    view architecture : ArchitectureView {
        satisfy ArchitectureViewpoint;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "view_body_satisfy.md"
    (diagnostics
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
    (reference r0 (scope relative) (span (offset 119) (line 4) (column 25) (len 16)) (segments (segment 0 (token "ArchitectureView") (name "ArchitectureView") (separator none) (span (offset 119) (line 4) (column 25) (len 16)))))
    (reference r1 (scope relative) (span (offset 154) (line 5) (column 17) (len 21)) (segments (segment 0 (token "ArchitectureViewpoint") (name "ArchitectureViewpoint") (separator none) (span (offset 154) (line 5) (column 17) (len 21)))))
  )
  (root (package (name "ViewCoverage") (body brace (viewpoint-def) (view-def) (view (name "architecture") (short-name none) (type (ref r0)) (body brace (satisfy (assert false) (negated false) (requirement (reference (ref r1))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)))))))
)
~~~
