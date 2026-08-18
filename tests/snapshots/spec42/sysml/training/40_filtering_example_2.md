# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 40 (Filtering): Filtering Example-2"))
~~~
# SOURCE
~~~sysml
package 'Filtering Example-2' {
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
		public import vehicle::**[@Safety];
	}
	
	package 'Mandatory Safety Features' {
		/* Parts that contribute to safety AND are mandatory. */
		public import vehicle::**[@Safety and Safety::isMandatory];
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "40_filtering_example_2.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Filtering Example-2' {
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
        public import vehicle::** [@Safety];
    }
    package 'Mandatory Safety Features' {
        public import vehicle::** [@Safety && Safety::isMandatory];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 48) (line 2) (column 17) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 48) (line 2) (column 17) (len 12))) (segment 1 (token "Boolean") (name "Boolean") (separator colon-colon) (span (offset 62) (line 2) (column 31) (len 7)))))
    (reference r1 (scope relative) (span (offset 122) (line 5) (column 27) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 122) (line 5) (column 27) (len 7)))))
    (reference r2 (scope relative) (span (offset 207) (line 11) (column 23) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 207) (line 11) (column 23) (len 6)))))
    (reference r3 (scope relative) (span (offset 281) (line 13) (column 24) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 281) (line 13) (column 24) (len 6)))))
    (reference r4 (scope relative) (span (offset 364) (line 17) (column 18) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 364) (line 17) (column 18) (len 6)))))
    (reference r5 (scope relative) (span (offset 484) (line 22) (column 29) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 484) (line 22) (column 29) (len 6)))))
    (reference r6 (scope relative) (span (offset 611) (line 28) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 611) (line 28) (column 17) (len 7)))))
    (reference r7 (scope relative) (span (offset 624) (line 28) (column 30) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 624) (line 28) (column 30) (len 6)))))
    (reference r8 (scope relative) (span (offset 752) (line 33) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 752) (line 33) (column 17) (len 7)))))
    (reference r9 (scope relative) (span (offset 765) (line 33) (column 30) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 765) (line 33) (column 30) (len 6)))))
    (reference r10 (scope relative) (span (offset 776) (line 33) (column 41) (len 19)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 776) (line 33) (column 41) (len 6))) (segment 1 (token "isMandatory") (name "isMandatory") (separator colon-colon) (span (offset 784) (line 33) (column 49) (len 11)))))
  )
  (root (package (name "Filtering Example-2") (body brace (import (target (span (span (offset 48) (line 2) (column 17) (len 21))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (metadata-def (name "Safety") (abstract false) (specializes none) (body brace (attribute-usage (declaration-name "isMandatory") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "interior") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "alarm") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "seatBelt") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 202) (line 11) (column 18) (len 1)) (integer 2))) (upper (expression (span (offset 202) (line 11) (column 18) (len 1)) (integer 2)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r2)) (about) (body brace (attribute-usage (declaration-name "isMandatory") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 228) (line 11) (column 44) (len 4)) (boolean true))))) (body semicolon)))))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontSeat") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 254) (line 12) (column 19) (len 1)) (integer 2))) (upper (expression (span (offset 254) (line 12) (column 19) (len 1)) (integer 2)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "driverAirBag") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r3)) (about) (body brace (attribute-usage (declaration-name "isMandatory") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 302) (line 13) (column 45) (len 5)) (boolean false))))) (body semicolon)))))))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bodyAssy") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "body") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bumper") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r4)) (about) (body brace (attribute-usage (declaration-name "isMandatory") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 385) (line 17) (column 39) (len 4)) (boolean true))))) (body semicolon)))))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "keylessEntry") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheelAssy") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheel") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 452) (line 21) (column 15) (len 1)) (integer 2))) (upper (expression (span (offset 452) (line 21) (column 15) (len 1)) (integer 2)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "antilockBrakes") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 479) (line 22) (column 24) (len 1)) (integer 2))) (upper (expression (span (offset 479) (line 22) (column 24) (len 1)) (integer 2)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r5)) (about) (body brace (attribute-usage (declaration-name "isMandatory") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 505) (line 22) (column 50) (len 5)) (boolean false))))) (body semicolon)))))))))) (package (name "Safety Features") (body brace (import (target (span (span (offset 611) (line 28) (column 17) (len 20))) (all none) (ref r6) (shape (filter (recursive-suffix (span (span (offset 618) (line 28) (column 24) (len 4))) (separator (span (offset 618) (line 28) (column 24) (len 2))) (marker (span (offset 620) (line 28) (column 26) (len 2)))) (members (filter-member (span (span (offset 622) (line 28) (column 28) (len 9))) (open (span (offset 622) (line 28) (column 28) (len 1))) (expression (expression (span (offset 623) (line 28) (column 29) (len 7)) (classification (metaclass (ref r7))))) (close (span (offset 630) (line 28) (column 36) (len 1))))))))))) (package (name "Mandatory Safety Features") (body brace (import (target (span (span (offset 752) (line 33) (column 17) (len 44))) (all none) (ref r8) (shape (filter (recursive-suffix (span (span (offset 759) (line 33) (column 24) (len 4))) (separator (span (offset 759) (line 33) (column 24) (len 2))) (marker (span (offset 761) (line 33) (column 26) (len 2)))) (members (filter-member (span (span (offset 763) (line 33) (column 28) (len 33))) (open (span (offset 763) (line 33) (column 28) (len 1))) (expression (expression (span (offset 764) (line 33) (column 29) (len 31)) (binary (operator "&&") (left (expression (span (offset 764) (line 33) (column 29) (len 7)) (classification (metaclass (ref r9))))) (right (expression (span (offset 776) (line 33) (column 41) (len 19)) (ref r10)))))) (close (span (offset 795) (line 33) (column 60) (len 1))))))))))))))
)
~~~
