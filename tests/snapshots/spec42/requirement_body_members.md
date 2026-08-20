# META
~~~sexpr
(snapshot (type semantic) (description "Every requirement-definition body member family reaches its own typed node in source order: directed ref and calc parameters, a port usage, an allocation, a nested requirement definition with a short name, a bare anonymous `requirement;`, and both frame spellings (spec42 Gap 42)."))
~~~
# SOURCE
~~~sysml
package RequirementBodyMembers {
    requirement def R {
        in ref part suppliedPart : Part;
        in calc margin {
            1
        }
        port controlPort : ControlPort;
        allocate source to target;
        requirement def <'1'> Nested {
            doc
            /* a requirement definition nested in a requirement definition */
        }
        requirement;
        frame concern vs : VehicleSafety;
        frame named {
            doc
            /* the frame spelling that owns a body */
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "requirement_body_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package RequirementBodyMembers {
    requirement def R {
        in ref part suppliedPart : Part;
        in calc margin {
            1;
        }
        port controlPort : ControlPort;
        allocate source to target;
        requirement def <'1'> Nested {
            doc
            /* a requirement definition nested in a requirement definition */
        }
        requirement ;
        frame concern vs : VehicleSafety;
        frame named {
            doc
            /* the frame spelling that owns a body */
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 92) (line 3) (column 36) (len 4)) (segments (segment 0 (token "Part") (name "Part") (separator none) (span (offset 92) (line 3) (column 36) (len 4)))))
    (reference r1 (scope relative) (span (offset 174) (line 7) (column 28) (len 11)) (segments (segment 0 (token "ControlPort") (name "ControlPort") (separator none) (span (offset 174) (line 7) (column 28) (len 11)))))
    (reference r2 (scope relative) (span (offset 413) (line 14) (column 28) (len 13)) (segments (segment 0 (token "VehicleSafety") (name "VehicleSafety") (separator none) (span (offset 413) (line 14) (column 28) (len 13)))))
  )
  (root (package (name "RequirementBodyMembers") (body brace (requirement-def (name "R") (modifiers) (body brace (ref (name "suppliedPart") (short-name none) (prefix (direction in) (derived false) (usage-prefix none) (constant false)) (kind part) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (redefines none) (subsets none) (body semicolon)) (calc-usage (name "margin") (multiplicity none)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "controlPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (allocation-usage) (requirement-def (name "Nested") (modifiers) (body brace (doc (name none) (locale none) (body (span (offset 291) (line 11) (column 15) (len 61)) (normalized "a requirement definition nested in a requirement definition "))))) (requirement-usage (name none) (multiplicity none)) (frame (concern-keyword true) (name "vs") (short-name none) (type (ref r2)) (body semicolon)) (frame (concern-keyword false) (name "named") (short-name none) (type none) (body brace (doc (name none) (locale none) (body (span (offset 480) (line 17) (column 15) (len 37)) (normalized "the frame spelling that owns a body "))))))))))
)
~~~
