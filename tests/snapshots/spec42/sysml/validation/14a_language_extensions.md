# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (14-Language Extensions): 14a-Language Extensions"))
~~~
# SOURCE
~~~sysml
package '14a-Language Extensions' {
	private import 'User Defined Extensions'::*;
	
	package 'User Defined Extensions' {
		
		enum def ClassificationLevel {
			uncl;
			conf;
			secret;
		}
		
		metadata def Classified {
			ref :>> annotatedElement : SysML::PartUsage;
			attribute classificationLevel : ClassificationLevel[1];
		}
	}
	
	part part_X {
		metadata Classified {
			classificationLevel = ClassificationLevel::conf;
		}
	}
	
	// Alternative shorthand notation
	part part_Y {
		@Classified {
			classificationLevel = ClassificationLevel::conf;
		}
	}

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "14a_language_extensions.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '14a-Language Extensions' {
    private import 'User Defined Extensions'::*;
    package 'User Defined Extensions' {
        enum def ClassificationLevel {
            uncl;
            conf;
            secret;
        }
        metadata def Classified {
            attribute :>> annotatedElement : SysML::PartUsage;
            attribute classificationLevel : ClassificationLevel[1];
        }
    }
    part part_X {
        metadata Classified {
            classificationLevel = ClassificationLevel::conf;
        }
    }
    part part_Y {
        @Classified {
            classificationLevel = ClassificationLevel::conf;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 52) (line 2) (column 17) (len 25)) (segments (segment 0 (token "'User Defined Extensions'") (name "User Defined Extensions") (separator none) (span (offset 52) (line 2) (column 17) (len 25)))))
    (reference r1 (scope relative) (span (offset 251) (line 13) (column 31) (len 16)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 251) (line 13) (column 31) (len 5))) (segment 1 (token "PartUsage") (name "PartUsage") (separator colon-colon) (span (offset 258) (line 13) (column 38) (len 9)))))
    (reference r2 (scope relative) (span (offset 232) (line 13) (column 12) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 232) (line 13) (column 12) (len 16)))))
    (reference r3 (scope relative) (span (offset 304) (line 14) (column 36) (len 19)) (segments (segment 0 (token "ClassificationLevel") (name "ClassificationLevel") (separator none) (span (offset 304) (line 14) (column 36) (len 19)))))
    (reference r4 (scope relative) (span (offset 379) (line 20) (column 4) (len 19)) (segments (segment 0 (token "classificationLevel") (name "classificationLevel") (separator none) (span (offset 379) (line 20) (column 4) (len 19)))))
    (reference r5 (scope relative) (span (offset 401) (line 20) (column 26) (len 25)) (segments (segment 0 (token "ClassificationLevel") (name "ClassificationLevel") (separator none) (span (offset 401) (line 20) (column 26) (len 19))) (segment 1 (token "conf") (name "conf") (separator colon-colon) (span (offset 422) (line 20) (column 47) (len 4)))))
    (reference r6 (scope relative) (span (offset 490) (line 26) (column 4) (len 10)) (segments (segment 0 (token "Classified") (name "Classified") (separator none) (span (offset 490) (line 26) (column 4) (len 10)))))
    (reference r7 (scope relative) (span (offset 506) (line 27) (column 4) (len 19)) (segments (segment 0 (token "classificationLevel") (name "classificationLevel") (separator none) (span (offset 506) (line 27) (column 4) (len 19)))))
    (reference r8 (scope relative) (span (offset 528) (line 27) (column 26) (len 25)) (segments (segment 0 (token "ClassificationLevel") (name "ClassificationLevel") (separator none) (span (offset 528) (line 27) (column 26) (len 19))) (segment 1 (token "conf") (name "conf") (separator colon-colon) (span (offset 549) (line 27) (column 47) (len 4)))))
  )
  (root (package (name "14a-Language Extensions") (body brace (import (target (span (span (offset 52) (line 2) (column 17) (len 28))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 77) (line 2) (column 42) (len 3))) (separator (span (offset 77) (line 2) (column 42) (len 2))) (marker (span (offset 79) (line 2) (column 44) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "User Defined Extensions") (body brace (enum-def (name "ClassificationLevel") (body brace (enum-value (enum-keyword none) (visibility none) (name "uncl") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 160) (line 7) (column 4) (len 5))) (enum-value (enum-keyword none) (visibility none) (name "conf") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 169) (line 8) (column 4) (len 5))) (enum-value (enum-keyword none) (visibility none) (name "secret") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 178) (line 9) (column 4) (len 7))))) (metadata-def (name "Classified") (abstract false) (specializes none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "classificationLevel") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "part_X") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (metadata-usage (declaration-name "Classified") (type none) (about) (body brace (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r4)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 401) (line 20) (column 26) (len 25)) (ref r5))))) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "part_Y") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r6)) (about) (body brace (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r7)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 528) (line 27) (column 26) (len 25)) (ref r8))))) (body semicolon)))))))))
)
~~~
