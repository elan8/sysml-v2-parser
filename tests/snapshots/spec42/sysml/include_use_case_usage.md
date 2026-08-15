# META
~~~sexpr
(snapshot (type semantic) (description "SysML IncludeUseCaseUsage reference form in a package body"))
~~~
# SOURCE
~~~sysml
package IncludeUseCaseUsageExample {
    include checkTires[1..*];
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "include_use_case_usage.md"
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
    (reference r0 (scope relative) (span (offset 49) (line 2) (column 13) (len 10)) (segments (segment 0 (token "checkTires") (name "checkTires") (separator none) (span (offset 49) (line 2) (column 13) (len 10)))))
  )
  (root (package (name "IncludeUseCaseUsageExample") (body brace (include (target (ref r0))))))
)
~~~
