# META
~~~sexpr
(snapshot (type semantic) (description "Direct upstream KerML Variable Feature Examples/Enhancements/TimeVaryingSteps.kerml excerpt: nested member steps and features retain TypeFeaturingPart targets and bodies."))
~~~
# SOURCE
~~~sysml
package TimeVaryingSteps {
    behavior TakePicture {
        member step merge : ControlPerformances::MergePerformance [0..1] featured by TakePicture_snapshots {
            member feature TakePicture_snapshots :>> Occurrences::Occurrence::snapshots featured by TakePicture {
                public import merge;
            }
        }
        member step focus [0..1] featured by TakePicture_snapshots {
            member feature TakePicture_snapshots :>> Occurrences::Occurrence::snapshots featured by TakePicture {
                public import focus;
            }
        }
        member step shoot [0..1] featured by TakePicture_snapshots {
            member feature TakePicture_snapshots :>> Occurrences::Occurrence::snapshots featured by TakePicture {
                public import shoot;
            }
        }
        member step decide : ControlPerformances::DecisionPerformance [0..1] featured by TakePicture_snapshots {
            member feature TakePicture_snapshots :>> Occurrences::Occurrence::snapshots featured by TakePicture {
                public import decide;
            }
        }
    }
    struct Camera {
        member step takePic : TakePicture [1] featured by Camera_snapshots {
            member feature Camera_snapshots :>> Occurrences::Occurrence::snapshots featured by Camera;
        }
    }
    struct MultiCamera {
        member step takePics : TakePicture [0..*] featured by Camera_snapshots {
            member feature Camera_snapshots :>> Occurrences::Occurrence::snapshots featured by Camera;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "upstream_time_varying_steps_type_featuring.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package TimeVaryingSteps {
    behavior TakePicture {
        member step 'merge' : ControlPerformances::MergePerformance[0..1] featured by TakePicture_snapshots {
            member feature TakePicture_snapshots :>> Occurrences::Occurrence::snapshots featured by TakePicture {
                public import merge;
            }
        }
        member step focus[0..1] featured by TakePicture_snapshots {
            member feature TakePicture_snapshots :>> Occurrences::Occurrence::snapshots featured by TakePicture {
                public import focus;
            }
        }
        member step shoot[0..1] featured by TakePicture_snapshots {
            member feature TakePicture_snapshots :>> Occurrences::Occurrence::snapshots featured by TakePicture {
                public import shoot;
            }
        }
        member step 'decide' : ControlPerformances::DecisionPerformance[0..1] featured by TakePicture_snapshots {
            member feature TakePicture_snapshots :>> Occurrences::Occurrence::snapshots featured by TakePicture {
                public import decide;
            }
        }
    }
    struct Camera {
        member step takePic : TakePicture[1] featured by Camera_snapshots {
            member feature Camera_snapshots :>> Occurrences::Occurrence::snapshots featured by Camera;
        }
    }
    struct MultiCamera {
        member step takePics : TakePicture[0..*] featured by Camera_snapshots {
            member feature Camera_snapshots :>> Occurrences::Occurrence::snapshots featured by Camera;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 82) (line 3) (column 29) (len 37)) (segments (segment 0 (token "ControlPerformances") (name "ControlPerformances") (separator none) (span (offset 82) (line 3) (column 29) (len 19))) (segment 1 (token "MergePerformance") (name "MergePerformance") (separator colon-colon) (span (offset 103) (line 3) (column 50) (len 16)))))
    (reference r1 (scope relative) (span (offset 139) (line 3) (column 86) (len 21)) (segments (segment 0 (token "TakePicture_snapshots") (name "TakePicture_snapshots") (separator none) (span (offset 139) (line 3) (column 86) (len 21)))))
    (reference r2 (scope relative) (span (offset 216) (line 4) (column 54) (len 34)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 216) (line 4) (column 54) (len 11))) (segment 1 (token "Occurrence") (name "Occurrence") (separator colon-colon) (span (offset 229) (line 4) (column 67) (len 10))) (segment 2 (token "snapshots") (name "snapshots") (separator colon-colon) (span (offset 241) (line 4) (column 79) (len 9)))))
    (reference r3 (scope relative) (span (offset 263) (line 4) (column 101) (len 11)) (segments (segment 0 (token "TakePicture") (name "TakePicture") (separator none) (span (offset 263) (line 4) (column 101) (len 11)))))
    (reference r4 (scope relative) (span (offset 383) (line 8) (column 46) (len 21)) (segments (segment 0 (token "TakePicture_snapshots") (name "TakePicture_snapshots") (separator none) (span (offset 383) (line 8) (column 46) (len 21)))))
    (reference r5 (scope relative) (span (offset 460) (line 9) (column 54) (len 34)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 460) (line 9) (column 54) (len 11))) (segment 1 (token "Occurrence") (name "Occurrence") (separator colon-colon) (span (offset 473) (line 9) (column 67) (len 10))) (segment 2 (token "snapshots") (name "snapshots") (separator colon-colon) (span (offset 485) (line 9) (column 79) (len 9)))))
    (reference r6 (scope relative) (span (offset 507) (line 9) (column 101) (len 11)) (segments (segment 0 (token "TakePicture") (name "TakePicture") (separator none) (span (offset 507) (line 9) (column 101) (len 11)))))
    (reference r7 (scope relative) (span (offset 627) (line 13) (column 46) (len 21)) (segments (segment 0 (token "TakePicture_snapshots") (name "TakePicture_snapshots") (separator none) (span (offset 627) (line 13) (column 46) (len 21)))))
    (reference r8 (scope relative) (span (offset 704) (line 14) (column 54) (len 34)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 704) (line 14) (column 54) (len 11))) (segment 1 (token "Occurrence") (name "Occurrence") (separator colon-colon) (span (offset 717) (line 14) (column 67) (len 10))) (segment 2 (token "snapshots") (name "snapshots") (separator colon-colon) (span (offset 729) (line 14) (column 79) (len 9)))))
    (reference r9 (scope relative) (span (offset 751) (line 14) (column 101) (len 11)) (segments (segment 0 (token "TakePicture") (name "TakePicture") (separator none) (span (offset 751) (line 14) (column 101) (len 11)))))
    (reference r10 (scope relative) (span (offset 855) (line 18) (column 30) (len 40)) (segments (segment 0 (token "ControlPerformances") (name "ControlPerformances") (separator none) (span (offset 855) (line 18) (column 30) (len 19))) (segment 1 (token "DecisionPerformance") (name "DecisionPerformance") (separator colon-colon) (span (offset 876) (line 18) (column 51) (len 19)))))
    (reference r11 (scope relative) (span (offset 915) (line 18) (column 90) (len 21)) (segments (segment 0 (token "TakePicture_snapshots") (name "TakePicture_snapshots") (separator none) (span (offset 915) (line 18) (column 90) (len 21)))))
    (reference r12 (scope relative) (span (offset 992) (line 19) (column 54) (len 34)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 992) (line 19) (column 54) (len 11))) (segment 1 (token "Occurrence") (name "Occurrence") (separator colon-colon) (span (offset 1005) (line 19) (column 67) (len 10))) (segment 2 (token "snapshots") (name "snapshots") (separator colon-colon) (span (offset 1017) (line 19) (column 79) (len 9)))))
    (reference r13 (scope relative) (span (offset 1039) (line 19) (column 101) (len 11)) (segments (segment 0 (token "TakePicture") (name "TakePicture") (separator none) (span (offset 1039) (line 19) (column 101) (len 11)))))
    (reference r14 (scope relative) (span (offset 1171) (line 25) (column 31) (len 11)) (segments (segment 0 (token "TakePicture") (name "TakePicture") (separator none) (span (offset 1171) (line 25) (column 31) (len 11)))))
    (reference r15 (scope relative) (span (offset 1199) (line 25) (column 59) (len 16)) (segments (segment 0 (token "Camera_snapshots") (name "Camera_snapshots") (separator none) (span (offset 1199) (line 25) (column 59) (len 16)))))
    (reference r16 (scope relative) (span (offset 1266) (line 26) (column 49) (len 34)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 1266) (line 26) (column 49) (len 11))) (segment 1 (token "Occurrence") (name "Occurrence") (separator colon-colon) (span (offset 1279) (line 26) (column 62) (len 10))) (segment 2 (token "snapshots") (name "snapshots") (separator colon-colon) (span (offset 1291) (line 26) (column 74) (len 9)))))
    (reference r17 (scope relative) (span (offset 1313) (line 26) (column 96) (len 6)) (segments (segment 0 (token "Camera") (name "Camera") (separator none) (span (offset 1313) (line 26) (column 96) (len 6)))))
    (reference r18 (scope relative) (span (offset 1393) (line 30) (column 32) (len 11)) (segments (segment 0 (token "TakePicture") (name "TakePicture") (separator none) (span (offset 1393) (line 30) (column 32) (len 11)))))
    (reference r19 (scope relative) (span (offset 1424) (line 30) (column 63) (len 16)) (segments (segment 0 (token "Camera_snapshots") (name "Camera_snapshots") (separator none) (span (offset 1424) (line 30) (column 63) (len 16)))))
    (reference r20 (scope relative) (span (offset 1491) (line 31) (column 49) (len 34)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 1491) (line 31) (column 49) (len 11))) (segment 1 (token "Occurrence") (name "Occurrence") (separator colon-colon) (span (offset 1504) (line 31) (column 62) (len 10))) (segment 2 (token "snapshots") (name "snapshots") (separator colon-colon) (span (offset 1516) (line 31) (column 74) (len 9)))))
    (reference r21 (scope relative) (span (offset 1538) (line 31) (column 96) (len 6)) (segments (segment 0 (token "Camera") (name "Camera") (separator none) (span (offset 1538) (line 31) (column 96) (len 6)))))
  )
  (root (package (name "TimeVaryingSteps") (body brace (kerml-classifier (keyword behavior) (abstract false) (name "TakePicture") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind step) (member true) (all false) (name "merge") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity (lower (expression (span (offset 121) (line 3) (column 68) (len 1)) (integer 0))) (upper (expression (span (offset 124) (line 3) (column 71) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships (featured-by (ref r1))) (value none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member true) (all false) (name "TakePicture_snapshots") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (references none) (crosses none) (relationships (featured-by (ref r3))) (value none) (body brace (import))))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind step) (member true) (all false) (name "focus") (typing none) (multiplicity (lower (expression (span (offset 365) (line 8) (column 28) (len 1)) (integer 0))) (upper (expression (span (offset 368) (line 8) (column 31) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships (featured-by (ref r4))) (value none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member true) (all false) (name "TakePicture_snapshots") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (references none) (crosses none) (relationships (featured-by (ref r6))) (value none) (body brace (import))))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind step) (member true) (all false) (name "shoot") (typing none) (multiplicity (lower (expression (span (offset 609) (line 13) (column 28) (len 1)) (integer 0))) (upper (expression (span (offset 612) (line 13) (column 31) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships (featured-by (ref r7))) (value none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member true) (all false) (name "TakePicture_snapshots") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (references none) (crosses none) (relationships (featured-by (ref r9))) (value none) (body brace (import))))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind step) (member true) (all false) (name "decide") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity (lower (expression (span (offset 897) (line 18) (column 72) (len 1)) (integer 0))) (upper (expression (span (offset 900) (line 18) (column 75) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships (featured-by (ref r11))) (value none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member true) (all false) (name "TakePicture_snapshots") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (references none) (crosses none) (relationships (featured-by (ref r13))) (value none) (body brace (import))))))) (kerml-classifier (keyword struct) (abstract false) (name "Camera") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind step) (member true) (all false) (name "takePic") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity (lower (expression (span (offset 1184) (line 25) (column 44) (len 1)) (integer 1))) (upper (expression (span (offset 1184) (line 25) (column 44) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships (featured-by (ref r15))) (value none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member true) (all false) (name "Camera_snapshots") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r16)))) (references none) (crosses none) (relationships (featured-by (ref r17))) (value none) (body semicolon)))))) (kerml-classifier (keyword struct) (abstract false) (name "MultiCamera") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind step) (member true) (all false) (name "takePics") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity (lower (expression (span (offset 1406) (line 30) (column 45) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships (featured-by (ref r19))) (value none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member true) (all false) (name "Camera_snapshots") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r20)))) (references none) (crosses none) (relationships (featured-by (ref r21))) (value none) (body semicolon)))))))))
)
~~~
