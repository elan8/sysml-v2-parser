# META
~~~sexpr
(snapshot (type semantic) (description "Full ref declarations with the use-case feature-kind keyword inside use case def bodies (ref use case self : UseCase :>> Case::self;) parse into typed RefDecl nodes, while the bare ref :>> shorthand keeps its dedicated node (spec42 Gap 34)."))
~~~
# SOURCE
~~~sysml
package UseCaseRefDeclarations {
    use case def UC {
        ref use case self : UseCase :>> Case::self;
        ref use case start : UseCase :>> start {
            doc /* d */
        }
        ref :>> timeslices {
            doc /* shorthand keeps its node */
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "use_case_ref_declarations.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package UseCaseRefDeclarations {
    use case def UC {
        ref use case self : UseCase :>> Case::self;
        ref use case start : UseCase :>> start {
            doc
            /* d */
        }
        ref :>> timeslices {
            doc
            /* shorthand keeps its node */
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 206) (line 7) (column 17) (len 10)) (segments (segment 0 (token "timeslices") (name "timeslices") (separator none) (span (offset 206) (line 7) (column 17) (len 10)))))
  )
  (root (package (name "UseCaseRefDeclarations") (body (use-case-def (name "UC") (body (ref) (ref) (ref-redefinition (target (ref r0)) (body-span (span (offset 217) (line 7) (column 28) (len 58))) (body (doc))))))))
)
~~~
