# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Requirement Derivation/RequirementDerivation"))
~~~
# SOURCE
~~~sysml
standard library package RequirementDerivation {
	doc /* This package provides language-extension metadata for modeling requirement derivation. */
	
	public import DerivationConnections::*;
	private import Metaobjects::SemanticMetadata;
	
	metadata def <original> OriginalRequirementMetadata :> SemanticMetadata {
		doc
		/*
		 * OriginalRequirementMetadata identifies a usage as an original requirement.
		 * It is intended to be used to tag the original requirement end of a Derivation.
		 */
		 
		:> annotatedElement : SysML::Usage;
		:>> baseType = originalRequirements meta SysML::Usage;
	}
	
	metadata def <derive> DerivedRequirementMetadata :> SemanticMetadata {
		doc
		/*
		 * DerivedRequirementMetadata identifies a usage as a derived requirement.
		 * It is intended to be used to tag the derived requirement ends of a Derivation.
		 */
		 
		:> annotatedElement : SysML::Usage;	
		:>> baseType = derivedRequirements meta SysML::Usage;
	}
	
	metadata def <derivation> DerivationMetadata :> SemanticMetadata {
		doc
		/*
		 * DerivationMetadata is SemanticMetadata for a Derivation connection.
		 */
		 
		:> annotatedElement : SysML::ConnectionDefinition;
		:> annotatedElement : SysML::ConnectionUsage;
		:>> baseType = derivations meta SysML::Usage;
	}	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "requirement_derivation.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package RequirementDerivation {
    doc
    /* This package provides language-extension metadata for modeling requirement derivation. */
    public import DerivationConnections::*;
    private import Metaobjects::SemanticMetadata;
    metadata def <original> OriginalRequirementMetadata :> SemanticMetadata {
        doc
        /*
		 * OriginalRequirementMetadata identifies a usage as an original requirement.
		 * It is intended to be used to tag the original requirement end of a Derivation.
		 */
        attribute :> annotatedElement : SysML::Usage;
        attribute :>> baseType = originalRequirements meta SysML::Usage;
    }
    metadata def <derive> DerivedRequirementMetadata :> SemanticMetadata {
        doc
        /*
		 * DerivedRequirementMetadata identifies a usage as a derived requirement.
		 * It is intended to be used to tag the derived requirement ends of a Derivation.
		 */
        attribute :> annotatedElement : SysML::Usage;
        attribute :>> baseType = derivedRequirements meta SysML::Usage;
    }
    metadata def <derivation> DerivationMetadata :> SemanticMetadata {
        doc
        /*
		 * DerivationMetadata is SemanticMetadata for a Derivation connection.
		 */
        attribute :> annotatedElement : SysML::ConnectionDefinition;
        attribute :> annotatedElement : SysML::ConnectionUsage;
        attribute :>> baseType = derivations meta SysML::Usage;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 164) (line 4) (column 16) (len 21)) (segments (segment 0 (token "DerivationConnections") (name "DerivationConnections") (separator none) (span (offset 164) (line 4) (column 16) (len 21)))))
    (reference r1 (scope relative) (span (offset 206) (line 5) (column 17) (len 29)) (segments (segment 0 (token "Metaobjects") (name "Metaobjects") (separator none) (span (offset 206) (line 5) (column 17) (len 11))) (segment 1 (token "SemanticMetadata") (name "SemanticMetadata") (separator colon-colon) (span (offset 219) (line 5) (column 30) (len 16)))))
    (reference r2 (scope relative) (span (offset 295) (line 7) (column 57) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 295) (line 7) (column 57) (len 16)))))
    (reference r3 (scope relative) (span (offset 523) (line 14) (column 25) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 523) (line 14) (column 25) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 530) (line 14) (column 32) (len 5)))))
    (reference r4 (scope relative) (span (offset 504) (line 14) (column 6) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 504) (line 14) (column 6) (len 16)))))
    (reference r5 (scope relative) (span (offset 543) (line 15) (column 7) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 543) (line 15) (column 7) (len 8)))))
    (reference r6 (scope relative) (span (offset 554) (line 15) (column 18) (len 20)) (segments (segment 0 (token "originalRequirements") (name "originalRequirements") (separator none) (span (offset 554) (line 15) (column 18) (len 20)))))
    (reference r7 (scope relative) (span (offset 580) (line 15) (column 44) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 580) (line 15) (column 44) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 587) (line 15) (column 51) (len 5)))))
    (reference r8 (scope relative) (span (offset 652) (line 18) (column 54) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 652) (line 18) (column 54) (len 16)))))
    (reference r9 (scope relative) (span (offset 877) (line 25) (column 25) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 877) (line 25) (column 25) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 884) (line 25) (column 32) (len 5)))))
    (reference r10 (scope relative) (span (offset 858) (line 25) (column 6) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 858) (line 25) (column 6) (len 16)))))
    (reference r11 (scope relative) (span (offset 898) (line 26) (column 7) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 898) (line 26) (column 7) (len 8)))))
    (reference r12 (scope relative) (span (offset 909) (line 26) (column 18) (len 19)) (segments (segment 0 (token "derivedRequirements") (name "derivedRequirements") (separator none) (span (offset 909) (line 26) (column 18) (len 19)))))
    (reference r13 (scope relative) (span (offset 934) (line 26) (column 43) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 934) (line 26) (column 43) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 941) (line 26) (column 50) (len 5)))))
    (reference r14 (scope relative) (span (offset 1002) (line 29) (column 50) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 1002) (line 29) (column 50) (len 16)))))
    (reference r15 (scope relative) (span (offset 1139) (line 35) (column 25) (len 27)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 1139) (line 35) (column 25) (len 5))) (segment 1 (token "ConnectionDefinition") (name "ConnectionDefinition") (separator colon-colon) (span (offset 1146) (line 35) (column 32) (len 20)))))
    (reference r16 (scope relative) (span (offset 1120) (line 35) (column 6) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 1120) (line 35) (column 6) (len 16)))))
    (reference r17 (scope relative) (span (offset 1192) (line 36) (column 25) (len 22)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 1192) (line 36) (column 25) (len 5))) (segment 1 (token "ConnectionUsage") (name "ConnectionUsage") (separator colon-colon) (span (offset 1199) (line 36) (column 32) (len 15)))))
    (reference r18 (scope relative) (span (offset 1173) (line 36) (column 6) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 1173) (line 36) (column 6) (len 16)))))
    (reference r19 (scope relative) (span (offset 1222) (line 37) (column 7) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 1222) (line 37) (column 7) (len 8)))))
    (reference r20 (scope relative) (span (offset 1233) (line 37) (column 18) (len 11)) (segments (segment 0 (token "derivations") (name "derivations") (separator none) (span (offset 1233) (line 37) (column 18) (len 11)))))
    (reference r21 (scope relative) (span (offset 1250) (line 37) (column 35) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 1250) (line 37) (column 35) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 1257) (line 37) (column 42) (len 5)))))
  )
  (root (library-package (name "RequirementDerivation") (standard true) (body brace (doc (name none) (locale none) (body (span (offset 56) (line 2) (column 8) (len 88)) (normalized "This package provides language-extension metadata for modeling requirement derivation. "))) (import (target (span (span (offset 164) (line 4) (column 16) (len 24))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 185) (line 4) (column 37) (len 3))) (separator (span (offset 185) (line 4) (column 37) (len 2))) (marker (span (offset 187) (line 4) (column 39) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 206) (line 5) (column 17) (len 29))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (metadata-def (name "OriginalRequirementMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r2)))) (body brace (doc (name none) (locale none) (body (span (offset 324) (line 9) (column 5) (len 168)) (normalized "OriginalRequirementMetadata identifies a usage as an original requirement.\nIt is intended to be used to tag the original requirement end of a Derivation.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r4)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 554) (line 15) (column 18) (len 38)) (meta-cast (base (expression (span (offset 554) (line 15) (column 18) (len 20)) (ref r6))) (metaclass (ref r7))))))) (body semicolon)))) (metadata-def (name "DerivedRequirementMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r8)))) (body brace (doc (name none) (locale none) (body (span (offset 681) (line 20) (column 5) (len 165)) (normalized "DerivedRequirementMetadata identifies a usage as a derived requirement.\nIt is intended to be used to tag the derived requirement ends of a Derivation.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r10)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 909) (line 26) (column 18) (len 37)) (meta-cast (base (expression (span (offset 909) (line 26) (column 18) (len 19)) (ref r12))) (metaclass (ref r13))))))) (body semicolon)))) (metadata-def (name "DerivationMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r14)))) (body brace (doc (name none) (locale none) (body (span (offset 1031) (line 31) (column 5) (len 77)) (normalized "DerivationMetadata is SemanticMetadata for a Derivation connection.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r16)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r18)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r19)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1233) (line 37) (column 18) (len 29)) (meta-cast (base (expression (span (offset 1233) (line 37) (column 18) (len 11)) (ref r20))) (metaclass (ref r21))))))) (body semicolon)))))))
)
~~~
