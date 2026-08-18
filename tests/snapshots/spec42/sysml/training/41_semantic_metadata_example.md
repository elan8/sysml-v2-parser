# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 41 (Language Extension): Semantic Metadata Example"))
~~~
# SOURCE
~~~sysml
library package 'Semantic Metadata Example' {
	private import 'Model Library Example'::*;
	private import Metaobjects::SemanticMetadata;

	metadata def situation :> SemanticMetadata {
		:>> baseType = situations meta SysML::Usage;
	}
	
	metadata def cause :> SemanticMetadata {
		:>> baseType = causes meta SysML::Usage;
	}
	
	metadata def failure :> SemanticMetadata {
		:>> baseType = failures meta SysML::Usage;
	}
	
	metadata def causation :> SemanticMetadata {
		:>> baseType = causations meta SysML::Usage;
	}
	
	metadata def scenario :> SemanticMetadata {
		:>> baseType = scenarios meta SysML::Usage;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "41_semantic_metadata_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
library package 'Semantic Metadata Example' {
    private import 'Model Library Example'::*;
    private import Metaobjects::SemanticMetadata;
    metadata def situation :> SemanticMetadata {
        attribute :>> baseType = situations meta SysML::Usage;
    }
    metadata def cause :> SemanticMetadata {
        attribute :>> baseType = causes meta SysML::Usage;
    }
    metadata def failure :> SemanticMetadata {
        attribute :>> baseType = failures meta SysML::Usage;
    }
    metadata def causation :> SemanticMetadata {
        attribute :>> baseType = causations meta SysML::Usage;
    }
    metadata def scenario :> SemanticMetadata {
        attribute :>> baseType = scenarios meta SysML::Usage;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 62) (line 2) (column 17) (len 23)) (segments (segment 0 (token "'Model Library Example'") (name "Model Library Example") (separator none) (span (offset 62) (line 2) (column 17) (len 23)))))
    (reference r1 (scope relative) (span (offset 106) (line 3) (column 17) (len 29)) (segments (segment 0 (token "Metaobjects") (name "Metaobjects") (separator none) (span (offset 106) (line 3) (column 17) (len 11))) (segment 1 (token "SemanticMetadata") (name "SemanticMetadata") (separator colon-colon) (span (offset 119) (line 3) (column 30) (len 16)))))
    (reference r2 (scope relative) (span (offset 165) (line 5) (column 28) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 165) (line 5) (column 28) (len 16)))))
    (reference r3 (scope relative) (span (offset 190) (line 6) (column 7) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 190) (line 6) (column 7) (len 8)))))
    (reference r4 (scope relative) (span (offset 201) (line 6) (column 18) (len 10)) (segments (segment 0 (token "situations") (name "situations") (separator none) (span (offset 201) (line 6) (column 18) (len 10)))))
    (reference r5 (scope relative) (span (offset 217) (line 6) (column 34) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 217) (line 6) (column 34) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 224) (line 6) (column 41) (len 5)))))
    (reference r6 (scope relative) (span (offset 259) (line 9) (column 24) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 259) (line 9) (column 24) (len 16)))))
    (reference r7 (scope relative) (span (offset 284) (line 10) (column 7) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 284) (line 10) (column 7) (len 8)))))
    (reference r8 (scope relative) (span (offset 295) (line 10) (column 18) (len 6)) (segments (segment 0 (token "causes") (name "causes") (separator none) (span (offset 295) (line 10) (column 18) (len 6)))))
    (reference r9 (scope relative) (span (offset 307) (line 10) (column 30) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 307) (line 10) (column 30) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 314) (line 10) (column 37) (len 5)))))
    (reference r10 (scope relative) (span (offset 351) (line 13) (column 26) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 351) (line 13) (column 26) (len 16)))))
    (reference r11 (scope relative) (span (offset 376) (line 14) (column 7) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 376) (line 14) (column 7) (len 8)))))
    (reference r12 (scope relative) (span (offset 387) (line 14) (column 18) (len 8)) (segments (segment 0 (token "failures") (name "failures") (separator none) (span (offset 387) (line 14) (column 18) (len 8)))))
    (reference r13 (scope relative) (span (offset 401) (line 14) (column 32) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 401) (line 14) (column 32) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 408) (line 14) (column 39) (len 5)))))
    (reference r14 (scope relative) (span (offset 447) (line 17) (column 28) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 447) (line 17) (column 28) (len 16)))))
    (reference r15 (scope relative) (span (offset 472) (line 18) (column 7) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 472) (line 18) (column 7) (len 8)))))
    (reference r16 (scope relative) (span (offset 483) (line 18) (column 18) (len 10)) (segments (segment 0 (token "causations") (name "causations") (separator none) (span (offset 483) (line 18) (column 18) (len 10)))))
    (reference r17 (scope relative) (span (offset 499) (line 18) (column 34) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 499) (line 18) (column 34) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 506) (line 18) (column 41) (len 5)))))
    (reference r18 (scope relative) (span (offset 544) (line 21) (column 27) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 544) (line 21) (column 27) (len 16)))))
    (reference r19 (scope relative) (span (offset 569) (line 22) (column 7) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 569) (line 22) (column 7) (len 8)))))
    (reference r20 (scope relative) (span (offset 580) (line 22) (column 18) (len 9)) (segments (segment 0 (token "scenarios") (name "scenarios") (separator none) (span (offset 580) (line 22) (column 18) (len 9)))))
    (reference r21 (scope relative) (span (offset 595) (line 22) (column 33) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 595) (line 22) (column 33) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 602) (line 22) (column 40) (len 5)))))
  )
  (root (library-package (name "Semantic Metadata Example") (standard false) (body brace (import (target (span (span (offset 62) (line 2) (column 17) (len 26))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 85) (line 2) (column 40) (len 3))) (separator (span (offset 85) (line 2) (column 40) (len 2))) (marker (span (offset 87) (line 2) (column 42) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 106) (line 3) (column 17) (len 29))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (metadata-def (name "situation") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r2)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 201) (line 6) (column 18) (len 28)) (meta-cast (base (expression (span (offset 201) (line 6) (column 18) (len 10)) (ref r4))) (metaclass (ref r5))))))) (body semicolon)))) (metadata-def (name "cause") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r6)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 295) (line 10) (column 18) (len 24)) (meta-cast (base (expression (span (offset 295) (line 10) (column 18) (len 6)) (ref r8))) (metaclass (ref r9))))))) (body semicolon)))) (metadata-def (name "failure") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r10)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 387) (line 14) (column 18) (len 26)) (meta-cast (base (expression (span (offset 387) (line 14) (column 18) (len 8)) (ref r12))) (metaclass (ref r13))))))) (body semicolon)))) (metadata-def (name "causation") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r14)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 483) (line 18) (column 18) (len 28)) (meta-cast (base (expression (span (offset 483) (line 18) (column 18) (len 10)) (ref r16))) (metaclass (ref r17))))))) (body semicolon)))) (metadata-def (name "scenario") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r18)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r19)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 580) (line 22) (column 18) (len 27)) (meta-cast (base (expression (span (offset 580) (line 22) (column 18) (len 9)) (ref r20))) (metaclass (ref r21))))))) (body semicolon)))))))
)
~~~
