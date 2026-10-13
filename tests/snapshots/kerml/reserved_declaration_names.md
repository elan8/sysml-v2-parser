# META
~~~sexpr
(snapshot (type malformed) (description "KerML lexical NAME rules reserve from/to while SysML-only words such as port, item, merge and decide remain valid KerML identifiers; quoted names are unrestricted.") (name-language kerml))
~~~
# SOURCE
~~~sysml
package KerMLNames {
    feature port;
    feature item;
    step merge;
    step decide;
    feature from;
    feature to;
    class <from> ValidClass;
    feature 'from';
    feature 'to';
    feature retained;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "reserved_declaration_names.md"
    (diagnostics
      (diagnostic (code "reserved_declaration_name") (severity error) (category parseerror) (span (offset 102) (line 6) (column 13) (len 4)) (message "a reserved keyword cannot be an unquoted declaration name"))
      (diagnostic (code "reserved_declaration_name") (severity error) (category parseerror) (span (offset 120) (line 7) (column 13) (len 2)) (message "a reserved keyword cannot be an unquoted declaration name"))
      (diagnostic (code "reserved_declaration_name") (severity error) (category parseerror) (span (offset 135) (line 8) (column 12) (len 4)) (message "a reserved keyword cannot be an unquoted declaration name"))
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
  (name-language kerml)
  (references
  )
  (root (package (name "KerMLNames") (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "port") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "item") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind step) (member false) (all false) (name "merge") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind step) (member false) (all false) (name "decide") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name (invalid-name (reason reserved-keyword) (token "from") (span (offset 102) (line 6) (column 13) (len 4)))) (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name (invalid-name (reason reserved-keyword) (token "to") (span (offset 120) (line 7) (column 13) (len 2)))) (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-classifier (keyword class) (abstract false) (name "ValidClass") (multiplicity none) (specializes none) (conjugates none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "from") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "to") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "retained") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))))
)
~~~
