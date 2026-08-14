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
    (reference r0 (scope relative) (span (offset 83) (line 3) (column 29) (len 7)) (segments (segment 0 (token "UseCase") (name "UseCase") (separator none) (span (offset 83) (line 3) (column 29) (len 7)))))
    (reference r1 (scope relative) (span (offset 95) (line 3) (column 41) (len 10)) (segments (segment 0 (token "Case") (name "Case") (separator none) (span (offset 95) (line 3) (column 41) (len 4))) (segment 1 (token "self") (name "self") (separator colon-colon) (span (offset 101) (line 3) (column 47) (len 4)))))
    (reference r2 (scope relative) (span (offset 136) (line 4) (column 30) (len 7)) (segments (segment 0 (token "UseCase") (name "UseCase") (separator none) (span (offset 136) (line 4) (column 30) (len 7)))))
    (reference r3 (scope relative) (span (offset 148) (line 4) (column 42) (len 5)) (segments (segment 0 (token "start") (name "start") (separator none) (span (offset 148) (line 4) (column 42) (len 5)))))
    (reference r4 (scope relative) (span (offset 206) (line 7) (column 17) (len 10)) (segments (segment 0 (token "timeslices") (name "timeslices") (separator none) (span (offset 206) (line 7) (column 17) (len 10)))))
  )
  (root (package (name "UseCaseRefDeclarations") (body (use-case-def (name "UC") (body (ref (name "self") (kind use case) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (subsets none) (body semicolon)) (ref (name "start") (kind use case) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (subsets none) (body (doc))) (ref-redefinition (target (ref r4)) (body-span (span (offset 217) (line 7) (column 28) (len 58))) (body (doc))))))))
)
~~~
