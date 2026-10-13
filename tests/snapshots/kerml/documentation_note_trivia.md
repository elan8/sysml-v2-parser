# META
~~~sexpr
(snapshot (type semantic) (description "Documentation accepts line and block notes between identification halves, around locale and before its body, without consuming the documentation body or following members; short names and adjacent documentation remain distinct.") (name-language kerml))
~~~
# SOURCE
~~~sysml
class C {
    doc <a> // between names
        b /* text */
    feature afterLine;
    doc <short> //* between names */ 'declared name' /* block note */
    doc <only> // before body
        /* short name only */
    doc <localized> //* before locale */ locale // before string
        "en_US" // before body
        /* localized text */
    doc // before identification
        named // before locale
        locale "nl_NL" //* before body */ /* named text */
    doc //* before body */ /* plain text */
    doc locale //* before string */ "de_DE" /* locale only */
    doc <last> /* own body */
    doc locale "fr_FR" /* next body */
    feature finalFeature;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "documentation_note_trivia.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
class C {
    doc <a> b
    /* text */
    feature afterLine;
    doc <short> 'declared name'
    /* block note */
    doc <only>
    /* short name only */
    doc <localized> locale "en_US"
    /* localized text */
    doc named locale "nl_NL"
    /* named text */
    doc
    /* plain text */
    doc locale "de_DE"
    /* locale only */
    doc <last>
    /* own body */
    doc locale "fr_FR"
    /* next body */
    feature finalFeature;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language kerml)
  (references
  )
  (root (kerml-classifier (keyword class) (abstract false) (name "C") (multiplicity none) (specializes none) (conjugates none) (body brace (doc (name "b") (short-name "a") (locale none) (body (span (offset 51) (line 3) (column 13) (len 6)) (normalized "text "))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "afterLine") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (doc (name "declared name") (short-name "short") (locale none) (body (span (offset 138) (line 5) (column 56) (len 12)) (normalized "block note "))) (doc (name none) (short-name "only") (locale none) (body (span (offset 193) (line 7) (column 11) (len 17)) (normalized "short name only "))) (doc (name none) (short-name "localized") (locale "en_US") (body (span (offset 319) (line 10) (column 11) (len 16)) (normalized "localized text "))) (doc (name "named") (short-name none) (locale "nl_NL") (body (span (offset 446) (line 13) (column 45) (len 12)) (normalized "named text "))) (doc (name none) (short-name none) (locale none) (body (span (offset 490) (line 14) (column 30) (len 12)) (normalized "plain text "))) (doc (name none) (short-name none) (locale "de_DE") (body (span (offset 551) (line 15) (column 47) (len 13)) (normalized "locale only "))) (doc (name none) (short-name "last") (locale none) (body (span (offset 584) (line 16) (column 18) (len 10)) (normalized "own body "))) (doc (name none) (short-name none) (locale "fr_FR") (body (span (offset 622) (line 17) (column 26) (len 11)) (normalized "next body "))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "finalFeature") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))))
)
~~~
