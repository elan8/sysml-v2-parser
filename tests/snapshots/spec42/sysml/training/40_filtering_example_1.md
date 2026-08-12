# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 40 (Filtering): Filtering Example-1"))
~~~
# SOURCE
~~~sysml
package 'Filtering Example-1' {
	private import ScalarValues::Boolean;
	
	metadata def Safety {
		attribute isMandatory : Boolean;
	}
	
	part vehicle {
		part interior {
			part alarm;
			part seatBelt[2] {@Safety{isMandatory = true;}}
			part frontSeat[2];
			part driverAirBag {@Safety{isMandatory = false;}}
		}
		part bodyAssy {
			part body;
			part bumper {@Safety{isMandatory = true;}}
			part keylessEntry;
		}
		part wheelAssy {
			part wheel[2];
			part antilockBrakes[2] {@Safety{isMandatory = false;}}
		}
	}
	
	package 'Safety Features' {
		/* Parts that contribute to safety. */		
		public import vehicle::**;
		filter @Safety;
	}
	
	package 'Mandatory Safety Features' {
		/* Parts that contribute to safety AND are mandatory. */
		public import vehicle::**;
		filter @Safety and Safety::isMandatory;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "40_filtering_example_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Filtering Example-1' {
    private import ScalarValues::Boolean;
    metadata def Safety {
        attribute isMandatory : Boolean;
    }
    part vehicle {
        part interior {
            part alarm;
            part seatBelt[2] {
                @Safety {
                    attribute isMandatory = true;
                }
            }
            part frontSeat[2];
            part driverAirBag {
                @Safety {
                    attribute isMandatory = false;
                }
            }
        }
        part bodyAssy {
            part body;
            part bumper {
                @Safety {
                    attribute isMandatory = true;
                }
            }
            part keylessEntry;
        }
        part wheelAssy {
            part wheel[2];
            part antilockBrakes[2] {
                @Safety {
                    attribute isMandatory = false;
                }
            }
        }
    }
    package 'Safety Features' {
        public import vehicle::**;
        filter @Safety;
    }
    package 'Mandatory Safety Features' {
        public import vehicle::**;
        filter @Safety && Safety::isMandatory;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 48) (line 2) (column 17) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 48) (line 2) (column 17) (len 12))) (segment 1 (token "Boolean") (name "Boolean") (separator colon-colon) (span (offset 62) (line 2) (column 31) (len 7)))))
    (reference r1 (scope relative) (span (offset 611) (line 28) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 611) (line 28) (column 17) (len 7)))))
    (reference r2 (scope relative) (span (offset 761) (line 34) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 761) (line 34) (column 17) (len 7)))))
  )
  (root (package (name "Filtering Example-1") (body (import (target (span (span (offset 48) (line 2) (column 17) (len 21))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (metadata-def) (part-usage) (package (name "Safety Features") (body (import (target (span (span (offset 611) (line 28) (column 17) (len 11))) (all none) (ref r1) (shape (membership (recursive-suffix (span (span (offset 618) (line 28) (column 24) (len 4))) (separator (span (offset 618) (line 28) (column 24) (len 2))) (marker (span (offset 620) (line 28) (column 26) (len 2)))))))) (filter))) (package (name "Mandatory Safety Features") (body (import (target (span (span (offset 761) (line 34) (column 17) (len 11))) (all none) (ref r2) (shape (membership (recursive-suffix (span (span (offset 768) (line 34) (column 24) (len 4))) (separator (span (offset 768) (line 34) (column 24) (len 2))) (marker (span (offset 770) (line 34) (column 26) (len 2)))))))) (filter))))))
)
~~~
