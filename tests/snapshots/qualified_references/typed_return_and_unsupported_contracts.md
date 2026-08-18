# META
~~~sexpr
(snapshot (type recovery) (description "Structured return-reference bodies, portion kinds, KerML feature/class declarations, and reference connection usages retain semantics, while unimplemented KerML semantic/feature-form declarations and malformed annotation syntax are explicitly unsupported/malformed."))
~~~
# SOURCE
~~~sysml
package TypedContracts {
    datatype DeferredType;
    feature deferredFeature : DeferredType;
    class DeferredClass;
    multiplicity exactlyOne [1..1];
    interaction DeferredInteraction;
    predicate deferredPredicate;

    verification def Verify {
        return ref result[0..*] {
            doc /* Result documentation. */
            return Results::accepted.item;
        }
        return ref empty;
    }

    part def Timeline {
        snapshot initial;
        then timeslice later;
        ref connection unresolved { } :> Ghost::subset;
    }

    interface def BrokenInterface {
        @@@ bogus ###;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "typed_return_and_unsupported_contracts.md"
    (diagnostics
      (diagnostic (code "malformed_annotation_head") (severity error) (category parseerror) (span (offset 609) (line 24) (column 9) (len 19)) (message "malformed metadata reference in interface definition body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package TypedContracts {
    datatype DeferredType;
    feature deferredFeature : DeferredType;
    class def DeferredClass;
    multiplicity exactlyOne[1];
    interaction DeferredInteraction;
    predicate deferredPredicate;
    verification def Verify {
        return ref result[0..*] {
            doc
            /* Result documentation. */
            return Results::accepted.'item';
        }
        return ref empty;
    }
    part def Timeline {
        snapshot initial;
        then timeslice later;
        ref connection unresolved :> Ghost::subset {
        }
    }
    interface def BrokenInterface {
        @@@ bogus ###;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 355) (line 12) (column 20) (len 17)) (segments (segment 0 (token "Results") (name "Results") (separator none) (span (offset 355) (line 12) (column 20) (len 7))) (segment 1 (token "accepted") (name "accepted") (separator colon-colon) (span (offset 364) (line 12) (column 29) (len 8)))))
    (reference r1 (scope relative) (span (offset 373) (line 12) (column 38) (len 4)) (segments (segment 0 (token "item") (name "item") (separator none) (span (offset 373) (line 12) (column 38) (len 4)))))
  )
  (root (package (name "TypedContracts") (body brace (kerml-classifier (keyword datatype) (abstract false) (name "DeferredType") (specializes none) (body semicolon)) (kerml-feature (name "deferredFeature") (body semicolon)) (class-def) (kerml-classifier (keyword multiplicity) (abstract false) (name "exactlyOne") (specializes none) (body semicolon)) (kerml-classifier (keyword interaction) (abstract false) (name "DeferredInteraction") (specializes none) (body semicolon)) (kerml-classifier (keyword predicate) (abstract false) (name "deferredPredicate") (specializes none) (body semicolon)) (verification-case-def (name "Verify") (body brace (return-ref (name "result") (body-span (span (offset 290) (line 10) (column 33) (len 98))) (body brace (doc) (result (expression (span (offset 355) (line 12) (column 20) (len 22)) (member-access (base (expression (span (offset 355) (line 12) (column 20) (len 17)) (ref r0))) (separator dot) (member (ref r1))))))) (return-ref (name "empty") (body-span (span (offset 413) (line 14) (column 25) (len 1))) (body semicolon)))) (part-def (name "Timeline") (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "initial") (short-name none) (target none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration "later") (short-name none) (target none) (body semicolon)) (connection))) (interface-def (name "BrokenInterface") (modifiers) (specializes none) (body brace (malformed (code "malformed_annotation_head") (found "@@@ bogus ###;") (span (offset 609) (line 24) (column 9) (len 19))))))))
)
~~~
