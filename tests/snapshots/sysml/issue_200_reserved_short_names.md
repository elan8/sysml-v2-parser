# META
~~~sexpr
(snapshot (type malformed) (description "Issue 200 case 9: SysML do is reserved in short names and must be quoted; the malformed spelling is retained with its diagnostic."))
~~~
# SOURCE
~~~sysml
package ShortNames {
    attribute <do> bad;
    attribute <'do'> good;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "issue_200_reserved_short_names.md"
    (diagnostics
      (diagnostic (code "reserved_declaration_name") (severity error) (category parseerror) (span (offset 36) (line 2) (column 16) (len 2)) (message "a reserved keyword cannot be an unquoted declaration name"))
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
  (name-language sysml)
  (references
  )
  (root (package (name "ShortNames") (body brace (attribute-usage) (attribute-usage))))
)
~~~
