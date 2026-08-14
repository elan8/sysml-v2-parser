# META
~~~sexpr
(snapshot (type semantic) (description "Subject declarations cover the :>> redefinition spelling (named, and anonymous type-less with a value), the default-keyword value clause, and the plain typed forms (spec42 Gap 35)."))
~~~
# SOURCE
~~~sysml
package SubjectDeclarations {
    use case def U {
        subject subj :>> Case::subj;
    }
    requirement def R1 {
        subject :>> vehicle = vehicle_large;
    }
    requirement def R2 {
        subject generateTorque default engine1.generateTorque;
    }
    requirement def R3 {
        subject vehicle : Vehicle = testVehicle;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "subject_declarations.md"
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
    (reference r0 (scope relative) (span (offset 139) (line 6) (column 21) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 139) (line 6) (column 21) (len 7)))))
    (reference r1 (scope relative) (span (offset 149) (line 6) (column 31) (len 13)) (segments (segment 0 (token "vehicle_large") (name "vehicle_large") (separator none) (span (offset 149) (line 6) (column 31) (len 13)))))
    (reference r2 (scope relative) (span (offset 234) (line 9) (column 40) (len 7)) (segments (segment 0 (token "engine1") (name "engine1") (separator none) (span (offset 234) (line 9) (column 40) (len 7)))))
    (reference r3 (scope relative) (span (offset 242) (line 9) (column 48) (len 14)) (segments (segment 0 (token "generateTorque") (name "generateTorque") (separator none) (span (offset 242) (line 9) (column 48) (len 14)))))
    (reference r4 (scope relative) (span (offset 315) (line 12) (column 27) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 315) (line 12) (column 27) (len 7)))))
    (reference r5 (scope relative) (span (offset 325) (line 12) (column 37) (len 11)) (segments (segment 0 (token "testVehicle") (name "testVehicle") (separator none) (span (offset 325) (line 12) (column 37) (len 11)))))
  )
  (root (package (name "SubjectDeclarations") (body (use-case-def (name "U") (body (subject))) (requirement-def (name "R1") (body (subject (name "") (type none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 149) (line 6) (column 31) (len 13)) (ref r1)))))))) (requirement-def (name "R2") (body (subject (name "generateTorque") (type none) (redefines none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 234) (line 9) (column 40) (len 22)) (member-access (base (expression (span (offset 234) (line 9) (column 40) (len 7)) (ref r2))) (separator dot) (member (ref r3)))))))))) (requirement-def (name "R3") (body (subject (name "vehicle") (type (ref r4)) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 325) (line 12) (column 37) (len 11)) (ref r5)))))))))))
)
~~~
