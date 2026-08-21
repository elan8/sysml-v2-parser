# META
~~~sexpr
(snapshot (type semantic) (description "Empty member (bare semicolon) at file level after package. The trailing `in newX : Real;` is a legal Feature -- BasicFeaturePrefix's direction slot then FeatureDeclaration (KerML BNF 577/601) -- so it parses at namespace level rather than adding a second recovery, which is what the shared FeaturePrefix seam gives every scope that already accepted the undirected spelling."))
~~~
# SOURCE
~~~sysml
package MyPkg { }; in newX : Real;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "empty_member_after_package.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package MyPkg {
}

;

in newX : Real;
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "MyPkg") (body brace)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (kerml-feature (name "newX") (relationships) (value none) (body semicolon)))
)
~~~
