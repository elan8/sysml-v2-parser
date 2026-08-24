# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Simple Tests): MetadataTest. Its `#Security enum secret : ClassificationLevel = 2;` is the Pilot-only metadata-prefixed EnumeratedValue extension (Pilot SysML.xtext 765-771) absent from the 2026-04 pin's EnumeratedValue production (SysML-textual-bnf.kebnf 528-535), so it is retained as exact recovery."))
~~~
# SOURCE
~~~sysml
package MetadataTest {
	private import 'User Defined Extensions'::*;
	
	library package 'User Defined Extensions' {
		
		#Security enum def ClassificationLevel :> ScalarValues::Natural {
			uncl : ClassificationLevel = 0;
			conf : ClassificationLevel = 1;
			#Security enum secret : ClassificationLevel = 2;
		}
		
		metadata def Classified {
			ref :>> annotatedElement : SysML::Usage;
			ref classificationLevel : ClassificationLevel;
		}
		
		metadata def Security;
	}
	
	ref x {
		metadata Classified {
			classificationLevel = ClassificationLevel::conf;
		}
	}
	
	ref y {
		@Classified {
			classificationLevel = ClassificationLevel::conf;
		}
		@Security;
	}
	
	private ref #Classified #Security z1;
	abstract #Classified z2;
	
	ref z {
	    #Security #Classified metadata Classified {
	        classificationLevel = ClassificationLevel::secret;
	    }
	}	
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "metadata_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package MetadataTest {
    private import 'User Defined Extensions'::*;
    library package 'User Defined Extensions' {
        #Security
        enum def ClassificationLevel :> ScalarValues::Natural {
            uncl : ClassificationLevel = 0;
            conf : ClassificationLevel = 1;
            #Security enum secret : ClassificationLevel = 2;
        }
        metadata def Classified {
            attribute :>> annotatedElement : SysML::Usage;
            attribute classificationLevel : ClassificationLevel;
        }
        metadata def Security;
    }
    ref x {
        metadata Classified {
            classificationLevel = ClassificationLevel::conf;
        }
    }
    ref y {
        @Classified {
            classificationLevel = ClassificationLevel::conf;
        }
        @Security;
    }
    private ref #Classified #Security z1;
    abstract #Classified z2;
    ref z {
        #Security
        #Classified
        metadata Classified {
            classificationLevel = ClassificationLevel::secret;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 39) (line 2) (column 17) (len 25)) (segments (segment 0 (token "'User Defined Extensions'") (name "User Defined Extensions") (separator none) (span (offset 39) (line 2) (column 17) (len 25)))))
    (reference r1 (scope relative) (span (offset 122) (line 6) (column 4) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 122) (line 6) (column 4) (len 8)))))
    (reference r2 (scope relative) (span (offset 197) (line 7) (column 11) (len 19)) (segments (segment 0 (token "ClassificationLevel") (name "ClassificationLevel") (separator none) (span (offset 197) (line 7) (column 11) (len 19)))))
    (reference r3 (scope relative) (span (offset 232) (line 8) (column 11) (len 19)) (segments (segment 0 (token "ClassificationLevel") (name "ClassificationLevel") (separator none) (span (offset 232) (line 8) (column 11) (len 19)))))
    (reference r4 (scope relative) (span (offset 261) (line 9) (column 5) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 261) (line 9) (column 5) (len 8)))))
    (reference r5 (scope relative) (span (offset 284) (line 9) (column 28) (len 19)) (segments (segment 0 (token "ClassificationLevel") (name "ClassificationLevel") (separator none) (span (offset 284) (line 9) (column 28) (len 19)))))
    (reference r6 (scope relative) (span (offset 374) (line 13) (column 31) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 374) (line 13) (column 31) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 381) (line 13) (column 38) (len 5)))))
    (reference r7 (scope relative) (span (offset 355) (line 13) (column 12) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 355) (line 13) (column 12) (len 16)))))
    (reference r8 (scope relative) (span (offset 417) (line 14) (column 30) (len 19)) (segments (segment 0 (token "ClassificationLevel") (name "ClassificationLevel") (separator none) (span (offset 417) (line 14) (column 30) (len 19)))))
    (reference r9 (scope relative) (span (offset 511) (line 22) (column 4) (len 19)) (segments (segment 0 (token "classificationLevel") (name "classificationLevel") (separator none) (span (offset 511) (line 22) (column 4) (len 19)))))
    (reference r10 (scope relative) (span (offset 533) (line 22) (column 26) (len 25)) (segments (segment 0 (token "ClassificationLevel") (name "ClassificationLevel") (separator none) (span (offset 533) (line 22) (column 26) (len 19))) (segment 1 (token "conf") (name "conf") (separator colon-colon) (span (offset 554) (line 22) (column 47) (len 4)))))
    (reference r11 (scope relative) (span (offset 581) (line 27) (column 4) (len 10)) (segments (segment 0 (token "Classified") (name "Classified") (separator none) (span (offset 581) (line 27) (column 4) (len 10)))))
    (reference r12 (scope relative) (span (offset 597) (line 28) (column 4) (len 19)) (segments (segment 0 (token "classificationLevel") (name "classificationLevel") (separator none) (span (offset 597) (line 28) (column 4) (len 19)))))
    (reference r13 (scope relative) (span (offset 619) (line 28) (column 26) (len 25)) (segments (segment 0 (token "ClassificationLevel") (name "ClassificationLevel") (separator none) (span (offset 619) (line 28) (column 26) (len 19))) (segment 1 (token "conf") (name "conf") (separator colon-colon) (span (offset 640) (line 28) (column 47) (len 4)))))
    (reference r14 (scope relative) (span (offset 653) (line 30) (column 4) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 653) (line 30) (column 4) (len 8)))))
    (reference r15 (scope relative) (span (offset 682) (line 33) (column 15) (len 10)) (segments (segment 0 (token "Classified") (name "Classified") (separator none) (span (offset 682) (line 33) (column 15) (len 10)))))
    (reference r16 (scope relative) (span (offset 694) (line 33) (column 27) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 694) (line 33) (column 27) (len 8)))))
    (reference r17 (scope relative) (span (offset 718) (line 34) (column 12) (len 10)) (segments (segment 0 (token "Classified") (name "Classified") (separator none) (span (offset 718) (line 34) (column 12) (len 10)))))
    (reference r18 (scope relative) (span (offset 750) (line 37) (column 7) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 750) (line 37) (column 7) (len 8)))))
    (reference r19 (scope relative) (span (offset 760) (line 37) (column 17) (len 10)) (segments (segment 0 (token "Classified") (name "Classified") (separator none) (span (offset 760) (line 37) (column 17) (len 10)))))
    (reference r20 (scope relative) (span (offset 802) (line 38) (column 10) (len 19)) (segments (segment 0 (token "classificationLevel") (name "classificationLevel") (separator none) (span (offset 802) (line 38) (column 10) (len 19)))))
    (reference r21 (scope relative) (span (offset 824) (line 38) (column 32) (len 27)) (segments (segment 0 (token "ClassificationLevel") (name "ClassificationLevel") (separator none) (span (offset 824) (line 38) (column 32) (len 19))) (segment 1 (token "secret") (name "secret") (separator colon-colon) (span (offset 845) (line 38) (column 53) (len 6)))))
  )
  (root (package (name "MetadataTest") (body brace (import (target (span (span (offset 39) (line 2) (column 17) (len 28))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 64) (line 2) (column 42) (len 3))) (separator (span (offset 64) (line 2) (column 42) (len 2))) (marker (span (offset 66) (line 2) (column 44) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (library-package (name "User Defined Extensions") (standard false) (body brace (metadata-keyword-usage (type (ref r1)) (body none)) (enum-def (name "ClassificationLevel") (body brace (enum-value (extensions) (enum-keyword none) (visibility none) (name "uncl") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 219) (line 7) (column 33) (len 1)) (integer 0))))) (body semicolon) (span (offset 190) (line 7) (column 4) (len 31))) (enum-value (extensions) (enum-keyword none) (visibility none) (name "conf") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 254) (line 8) (column 33) (len 1)) (integer 1))))) (body semicolon) (span (offset 225) (line 8) (column 4) (len 31))) (enum-value (extensions (ref r4)) (enum-keyword present) (visibility none) (name "secret") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 306) (line 9) (column 50) (len 1)) (integer 2))))) (body semicolon) (span (offset 260) (line 9) (column 4) (len 48))))) (metadata-def (name "Classified") (abstract false) (specializes none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "classificationLevel") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (metadata-def (name "Security") (abstract false) (specializes none) (body semicolon)))) (ref (name "x") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body brace (metadata-usage (declaration-name "Classified") (type none) (about) (body brace (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r9)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 533) (line 22) (column 26) (len 25)) (ref r10))))) (body semicolon)))))) (ref (name "y") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r11)) (about) (body brace (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r12)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 619) (line 28) (column 26) (len 25)) (ref r13))))) (body semicolon)))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r14)) (about) (body semicolon)))) (ref (name "z1") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions (ref r15) (ref r16)) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (extended-def (prefix-keywords ((ref r17))) (definition-prefix abstract) (def false) (name "z2") (specializes none) (body semicolon)) (ref (name "z") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body brace (metadata-keyword-usage (type (ref r18)) (body none)) (metadata-keyword-usage (type (ref r19)) (body none)) (metadata-usage (declaration-name "Classified") (type none) (about) (body brace (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r20)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 824) (line 38) (column 32) (len 27)) (ref r21))))) (body semicolon)))))))))
)
~~~
