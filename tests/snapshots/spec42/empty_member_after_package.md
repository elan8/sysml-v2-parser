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
      (diagnostic (code "expected_keyword") (severity error) (category parseerror) (span (offset 17) (line 1) (column 18) (len 17)) (message "expected a specific keyword or punctuation token"))
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
  (root (package (name "MyPkg") (body brace)) (malformed (code "expected_keyword") (found "; in newX : Real;") (span (offset 17) (line 1) (column 18) (len 1))) (kerml-feature (name "newX") (body semicolon)))
)
~~~
