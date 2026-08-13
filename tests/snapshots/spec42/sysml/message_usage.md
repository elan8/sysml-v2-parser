# META
~~~sexpr
(snapshot (type semantic) (description "SysML message usage (isAbstract flow) in a package body, named and anonymous forms"))
~~~
# SOURCE
~~~sysml
package MessageUsageExample {
    message msg1 of Payload from a to b;
    message msg2 from a to b;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "message_usage.md"
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
  )
  (root (package (name "MessageUsageExample") (body (flow-usage) (flow-usage))))
)
~~~
