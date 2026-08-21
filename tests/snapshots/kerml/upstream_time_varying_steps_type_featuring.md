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
    (reference r0 (scope relative) (span (offset 139) (line 3) (column 86) (len 21)) (segments (segment 0 (token "TakePicture_snapshots") (name "TakePicture_snapshots") (separator none) (span (offset 139) (line 3) (column 86) (len 21)))))
    (reference r1 (scope relative) (span (offset 263) (line 4) (column 101) (len 11)) (segments (segment 0 (token "TakePicture") (name "TakePicture") (separator none) (span (offset 263) (line 4) (column 101) (len 11)))))
    (reference r2 (scope relative) (span (offset 383) (line 8) (column 46) (len 21)) (segments (segment 0 (token "TakePicture_snapshots") (name "TakePicture_snapshots") (separator none) (span (offset 383) (line 8) (column 46) (len 21)))))
    (reference r3 (scope relative) (span (offset 507) (line 9) (column 101) (len 11)) (segments (segment 0 (token "TakePicture") (name "TakePicture") (separator none) (span (offset 507) (line 9) (column 101) (len 11)))))
    (reference r4 (scope relative) (span (offset 627) (line 13) (column 46) (len 21)) (segments (segment 0 (token "TakePicture_snapshots") (name "TakePicture_snapshots") (separator none) (span (offset 627) (line 13) (column 46) (len 21)))))
    (reference r5 (scope relative) (span (offset 751) (line 14) (column 101) (len 11)) (segments (segment 0 (token "TakePicture") (name "TakePicture") (separator none) (span (offset 751) (line 14) (column 101) (len 11)))))
    (reference r6 (scope relative) (span (offset 915) (line 18) (column 90) (len 21)) (segments (segment 0 (token "TakePicture_snapshots") (name "TakePicture_snapshots") (separator none) (span (offset 915) (line 18) (column 90) (len 21)))))
    (reference r7 (scope relative) (span (offset 1039) (line 19) (column 101) (len 11)) (segments (segment 0 (token "TakePicture") (name "TakePicture") (separator none) (span (offset 1039) (line 19) (column 101) (len 11)))))
    (reference r8 (scope relative) (span (offset 1199) (line 25) (column 59) (len 16)) (segments (segment 0 (token "Camera_snapshots") (name "Camera_snapshots") (separator none) (span (offset 1199) (line 25) (column 59) (len 16)))))
    (reference r9 (scope relative) (span (offset 1313) (line 26) (column 96) (len 6)) (segments (segment 0 (token "Camera") (name "Camera") (separator none) (span (offset 1313) (line 26) (column 96) (len 6)))))
    (reference r10 (scope relative) (span (offset 1424) (line 30) (column 63) (len 16)) (segments (segment 0 (token "Camera_snapshots") (name "Camera_snapshots") (separator none) (span (offset 1424) (line 30) (column 63) (len 16)))))
    (reference r11 (scope relative) (span (offset 1538) (line 31) (column 96) (len 6)) (segments (segment 0 (token "Camera") (name "Camera") (separator none) (span (offset 1538) (line 31) (column 96) (len 6)))))
  )
  (root (package (name "TimeVaryingSteps") (body brace (kerml-classifier (keyword behavior) (abstract false) (name "TakePicture") (specializes none) (body brace (kerml-feature (name "merge") (relationships (featured-by (ref r0))) (value none) (body brace (kerml-feature (name "TakePicture_snapshots") (relationships (featured-by (ref r1))) (value none) (body brace (import))))) (kerml-feature (name "focus") (relationships (featured-by (ref r2))) (value none) (body brace (kerml-feature (name "TakePicture_snapshots") (relationships (featured-by (ref r3))) (value none) (body brace (import))))) (kerml-feature (name "shoot") (relationships (featured-by (ref r4))) (value none) (body brace (kerml-feature (name "TakePicture_snapshots") (relationships (featured-by (ref r5))) (value none) (body brace (import))))) (kerml-feature (name "decide") (relationships (featured-by (ref r6))) (value none) (body brace (kerml-feature (name "TakePicture_snapshots") (relationships (featured-by (ref r7))) (value none) (body brace (import))))))) (kerml-classifier (keyword struct) (abstract false) (name "Camera") (specializes none) (body brace (kerml-feature (name "takePic") (relationships (featured-by (ref r8))) (value none) (body brace (kerml-feature (name "Camera_snapshots") (relationships (featured-by (ref r9))) (value none) (body semicolon)))))) (kerml-classifier (keyword struct) (abstract false) (name "MultiCamera") (specializes none) (body brace (kerml-feature (name "takePics") (relationships (featured-by (ref r10))) (value none) (body brace (kerml-feature (name "Camera_snapshots") (relationships (featured-by (ref r11))) (value none) (body semicolon)))))))))
)
~~~
