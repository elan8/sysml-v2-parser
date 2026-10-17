# META
~~~sexpr
(snapshot (type semantic) (description "Rendering usages retain abstract, multiplicity, ordered/nonunique, subsets, redefines, and value clauses; the anonymous redefinition form nests inside a rendering usage body."))
~~~
# SOURCE
~~~sysml
package Views {
    abstract rendering renderings : Rendering[0..*] nonunique :> parts {
        doc /* renderings is the base rendering. */
    }
    rendering asElementTable : TabularRendering[1] :> renderings {
        view columnView[0..*] ordered {
            doc /* The Views to be rendered in the column cells. */
        }
        rendering :>> subrenderings[0..*] = columnView.viewRendering;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "rendering_usage_declarations.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Views {
    abstract rendering renderings : Rendering[0..*] nonunique :> parts {
        doc
        /* renderings is the base rendering. */
    }
    rendering asElementTable : TabularRendering[1] :> renderings {
        view columnView[0..*] ordered {
            doc
            /* The Views to be rendered in the column cells. */
        }
        rendering[0..*] :>> subrenderings = columnView.viewRendering;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 52) (line 2) (column 37) (len 9)) (segments (segment 0 (token "Rendering") (name "Rendering") (separator none) (span (offset 52) (line 2) (column 37) (len 9)))))
    (reference r1 (scope relative) (span (offset 81) (line 2) (column 66) (len 5)) (segments (segment 0 (token "parts") (name "parts") (separator none) (span (offset 81) (line 2) (column 66) (len 5)))))
    (reference r2 (scope relative) (span (offset 178) (line 5) (column 32) (len 16)) (segments (segment 0 (token "TabularRendering") (name "TabularRendering") (separator none) (span (offset 178) (line 5) (column 32) (len 16)))))
    (reference r3 (scope relative) (span (offset 201) (line 5) (column 55) (len 10)) (segments (segment 0 (token "renderings") (name "renderings") (separator none) (span (offset 201) (line 5) (column 55) (len 10)))))
    (reference r4 (scope relative) (span (offset 354) (line 9) (column 23) (len 13)) (segments (segment 0 (token "subrenderings") (name "subrenderings") (separator none) (span (offset 354) (line 9) (column 23) (len 13)))))
    (reference r5 (scope relative) (span (offset 376) (line 9) (column 45) (len 10)) (segments (segment 0 (token "columnView") (name "columnView") (separator none) (span (offset 376) (line 9) (column 45) (len 10)))))
    (reference r6 (scope relative) (span (offset 387) (line 9) (column 56) (len 13)) (segments (segment 0 (token "viewRendering") (name "viewRendering") (separator none) (span (offset 387) (line 9) (column 56) (len 13)))))
  )
  (root (package (name "Views") (body brace (rendering-usage (abstract true) (name "renderings") (short-name none) (type (ref r0)) (multiplicity (lower (expression (span (offset 62) (line 2) (column 47) (len 1)) (integer 0))) (upper (expression (span (offset 65) (line 2) (column 50) (len 1)) (infinity)))) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (value none) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 103) (line 3) (column 15) (len 35)) (normalized "renderings is the base rendering. "))))) (rendering-usage (abstract false) (name "asElementTable") (short-name none) (type (ref r2)) (multiplicity (lower none) (upper (expression (span (offset 195) (line 5) (column 49) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r3)))) (redefines none) (value none) (body brace (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "columnView") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 238) (line 6) (column 25) (len 1)) (integer 0))) (upper (expression (span (offset 241) (line 6) (column 28) (len 1)) (infinity)))) (multiplicity-modifiers (ordering ordered) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 272) (line 7) (column 19) (len 47)) (normalized "The Views to be rendered in the column cells. "))))) (rendering-usage (abstract false) (name none) (short-name none) (type none) (multiplicity (lower (expression (span (offset 368) (line 9) (column 37) (len 1)) (integer 0))) (upper (expression (span (offset 371) (line 9) (column 40) (len 1)) (infinity)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 376) (line 9) (column 45) (len 24)) (member-access (base (expression (span (offset 376) (line 9) (column 45) (len 10)) (ref r5))) (separator dot) (member (ref r6))))))) (body semicolon)))))))
)
~~~
