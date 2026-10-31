# META
~~~sexpr
(snapshot (type malformed) (description "Issue 200 cases 4 and 9: feature short names are retained in classifier and package bodies; do is a KerML NAME, if is reserved unless quoted.") (name-language kerml))
~~~
# SOURCE
~~~sysml
classifier Vehicle {
    feature <ms> mass;
    feature <do> : SpatialFrame;
    feature <'if'> : SpatialFrame;
}
package Root {
    feature <f> standalone;
    feature <if> : SpatialFrame;
    feature <'if'> : SpatialFrame;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "issue_200_feature_short_names.md"
    (diagnostics
      (diagnostic (code "reserved_declaration_name") (severity error) (category parseerror) (span (offset 170) (line 8) (column 14) (len 2)) (message "a reserved keyword cannot be an unquoted declaration name"))
    )
  )
)
~~~
# FORMAT
~~~sysml
classifier Vehicle {
    feature <ms> mass;
    feature <do> : SpatialFrame;
    feature <'if'> : SpatialFrame;
}

package Root {
    feature <f> standalone;
    feature <if> : SpatialFrame;
    feature <'if'> : SpatialFrame;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language kerml)
  (references
    (reference r0 (scope relative) (span (offset 63) (line 3) (column 20) (len 12)) (segments (segment 0 (token "SpatialFrame") (name "SpatialFrame") (separator none) (span (offset 63) (line 3) (column 20) (len 12)))))
    (reference r1 (scope relative) (span (offset 98) (line 4) (column 22) (len 12)) (segments (segment 0 (token "SpatialFrame") (name "SpatialFrame") (separator none) (span (offset 98) (line 4) (column 22) (len 12)))))
    (reference r2 (scope relative) (span (offset 176) (line 8) (column 20) (len 12)) (segments (segment 0 (token "SpatialFrame") (name "SpatialFrame") (separator none) (span (offset 176) (line 8) (column 20) (len 12)))))
    (reference r3 (scope relative) (span (offset 211) (line 9) (column 22) (len 12)) (segments (segment 0 (token "SpatialFrame") (name "SpatialFrame") (separator none) (span (offset 211) (line 9) (column 22) (len 12)))))
  )
  (root (kerml-classifier (keyword classifier) (abstract false) (name "Vehicle") (multiplicity none) (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "mass") (short-name "ms") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name none) (short-name "do") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name none) (short-name "if") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))) (package (visibility none) (name "Root") (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "standalone") (short-name "f") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name none) (short-name (invalid-name (reason reserved-keyword) (token "if") (span (offset 170) (line 8) (column 14) (len 2)))) (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name none) (short-name "if") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))))
)
~~~
