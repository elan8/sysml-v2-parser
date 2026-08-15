# META
~~~sexpr
(snapshot (type semantic) (description "Assign node with sequence indexing operator #()"))
~~~
# SOURCE
~~~sysml
package AssignTest {
    action def A {
        assign x := seq#(i);
        assign 'var' := data#(idx);
        assign a.b := items#(0);
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "assign_seq_index.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package AssignTest {
    action def A {
        assign x := seq#(i);
        assign var := data#(idx);
        assign a.b := items#(0);
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "AssignTest") (body brace (action-def (name "A") (specializes none) (body brace (assign) (assign) (assign))))))
)
~~~
