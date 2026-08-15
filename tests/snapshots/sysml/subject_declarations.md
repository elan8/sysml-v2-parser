# META
~~~sexpr
(snapshot (type semantic) (description "A subject declaration accepts a multiplicity between its type and a trailing :>> clause, and a connection end may be declared by name alone (end ref source;). Subject declarations cover the :>> redefinition spelling (named, and anonymous type-less with a value), the default-keyword value clause, and the plain typed forms (spec42 Gap 35)."))
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
    requirement def R4 {
        subject subj : View[1] :>> RequirementCheck::subj;
    }
    connection def C {
        end ref source;
        end target : T;
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
    requirement def R4 {
        subject subj : View[1] :>> RequirementCheck::subj;
    }
    connection def C {
        end source;
        end target : T;
    }
}
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
    (reference r6 (scope relative) (span (offset 392) (line 15) (column 24) (len 4)) (segments (segment 0 (token "View") (name "View") (separator none) (span (offset 392) (line 15) (column 24) (len 4)))))
    (reference r7 (scope relative) (span (offset 404) (line 15) (column 36) (len 22)) (segments (segment 0 (token "RequirementCheck") (name "RequirementCheck") (separator none) (span (offset 404) (line 15) (column 36) (len 16))) (segment 1 (token "subj") (name "subj") (separator colon-colon) (span (offset 422) (line 15) (column 54) (len 4)))))
    (reference r8 (scope relative) (span (offset 502) (line 19) (column 22) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 502) (line 19) (column 22) (len 1)))))
  )
  (root (package (name "SubjectDeclarations") (body (use-case-def (name "U") (body (subject))) (requirement-def (name "R1") (body (subject (name "") (type none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 149) (line 6) (column 31) (len 13)) (ref r1)))))))) (requirement-def (name "R2") (body (subject (name "generateTorque") (type none) (redefines none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 234) (line 9) (column 40) (len 22)) (member-access (base (expression (span (offset 234) (line 9) (column 40) (len 7)) (ref r2))) (separator dot) (member (ref r3)))))))))) (requirement-def (name "R3") (body (subject (name "vehicle") (type (ref r4)) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 325) (line 12) (column 37) (len 11)) (ref r5)))))))) (requirement-def (name "R4") (body (subject (name "subj") (type (ref r6)) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (value none)))) (connection-def (name "C") (role ordinary) (specializes none) (body (end (identity (declaration (name "source") (span (offset 473) (line 18) (column 17) (len 6)))) (typing none) (references none) (redefines none) (crosses none)) (end (identity (declaration (name "target") (span (offset 493) (line 19) (column 13) (len 6)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (references none) (redefines none) (crosses none)))))))
)
~~~
