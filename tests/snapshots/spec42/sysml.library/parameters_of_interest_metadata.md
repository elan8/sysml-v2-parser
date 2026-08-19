# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Metadata/ParametersOfInterestMetadata"))
~~~
# SOURCE
~~~sysml
standard library package ParametersOfInterestMetadata {
	doc
	/*
	 * This package contains definitions of metadata to identify key parameters of interest,
	 * including measures of effectiveness (MOE) and other key measures of performance (MOP).
	 */
	 
	 private import Metaobjects::SemanticMetadata;
	 
	 attribute measuresOfEffectiveness[*] nonunique {
	 	doc /* Base feature for attributes that are measures of effectiveness. */
	 }
	 
	 attribute measuresOfPerformance[*] nonunique {
	 	doc /* Base feature for attributes that are measures of performance. */
	 }
	 
	 metadata def <moe> MeasureOfEffectiveness :> SemanticMetadata {
	 	doc 
	 	/*
	 	 * MeasureOfEffectiveness is semantic metadata for identifying an attribute as a
	 	 * measure of effectiveness.
	 	 */
	 	
	 	:>> annotatedElement : SysML::Usage;
	 	:>> baseType = measuresOfEffectiveness meta SysML::Usage;
	 }
	 
	 metadata def <mop> MeasureOfPerformance :> SemanticMetadata {
	 	doc 
	 	/*
	 	 * MeasureOfPerformance is semantic metadata for identifying an attribute as a
	 	 * measure of performance.
	 	 */
	 	
	 	:>> annotatedElement : SysML::Usage;
	 	:>> baseType = measuresOfPerformance meta SysML::Usage;
	 }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "parameters_of_interest_metadata.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package ParametersOfInterestMetadata {
    doc
    /*
	 * This package contains definitions of metadata to identify key parameters of interest,
	 * including measures of effectiveness (MOE) and other key measures of performance (MOP).
	 */
    private import Metaobjects::SemanticMetadata;
    attribute def measuresOfEffectiveness[*] nonunique {
        doc
        /* Base feature for attributes that are measures of effectiveness. */
    }
    attribute def measuresOfPerformance[*] nonunique {
        doc
        /* Base feature for attributes that are measures of performance. */
    }
    metadata def <moe> MeasureOfEffectiveness :> SemanticMetadata {
        doc
        /*
	 	 * MeasureOfEffectiveness is semantic metadata for identifying an attribute as a
	 	 * measure of effectiveness.
	 	 */
        attribute :>> annotatedElement : SysML::Usage;
        attribute :>> baseType = measuresOfEffectiveness meta SysML::Usage;
    }
    metadata def <mop> MeasureOfPerformance :> SemanticMetadata {
        doc
        /*
	 	 * MeasureOfPerformance is semantic metadata for identifying an attribute as a
	 	 * measure of performance.
	 	 */
        attribute :>> annotatedElement : SysML::Usage;
        attribute :>> baseType = measuresOfPerformance meta SysML::Usage;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 271) (line 8) (column 18) (len 29)) (segments (segment 0 (token "Metaobjects") (name "Metaobjects") (separator none) (span (offset 271) (line 8) (column 18) (len 11))) (segment 1 (token "SemanticMetadata") (name "SemanticMetadata") (separator colon-colon) (span (offset 284) (line 8) (column 31) (len 16)))))
    (reference r1 (scope relative) (span (offset 618) (line 18) (column 48) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 618) (line 18) (column 48) (len 16)))))
    (reference r2 (scope relative) (span (offset 804) (line 25) (column 27) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 804) (line 25) (column 27) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 811) (line 25) (column 34) (len 5)))))
    (reference r3 (scope relative) (span (offset 785) (line 25) (column 8) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 785) (line 25) (column 8) (len 16)))))
    (reference r4 (scope relative) (span (offset 825) (line 26) (column 8) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 825) (line 26) (column 8) (len 8)))))
    (reference r5 (scope relative) (span (offset 836) (line 26) (column 19) (len 23)) (segments (segment 0 (token "measuresOfEffectiveness") (name "measuresOfEffectiveness") (separator none) (span (offset 836) (line 26) (column 19) (len 23)))))
    (reference r6 (scope relative) (span (offset 865) (line 26) (column 48) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 865) (line 26) (column 48) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 872) (line 26) (column 55) (len 5)))))
    (reference r7 (scope relative) (span (offset 931) (line 29) (column 46) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 931) (line 29) (column 46) (len 16)))))
    (reference r8 (scope relative) (span (offset 1113) (line 36) (column 27) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 1113) (line 36) (column 27) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 1120) (line 36) (column 34) (len 5)))))
    (reference r9 (scope relative) (span (offset 1094) (line 36) (column 8) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 1094) (line 36) (column 8) (len 16)))))
    (reference r10 (scope relative) (span (offset 1134) (line 37) (column 8) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 1134) (line 37) (column 8) (len 8)))))
    (reference r11 (scope relative) (span (offset 1145) (line 37) (column 19) (len 21)) (segments (segment 0 (token "measuresOfPerformance") (name "measuresOfPerformance") (separator none) (span (offset 1145) (line 37) (column 19) (len 21)))))
    (reference r12 (scope relative) (span (offset 1172) (line 37) (column 46) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 1172) (line 37) (column 46) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 1179) (line 37) (column 53) (len 5)))))
  )
  (root (library-package (name "ParametersOfInterestMetadata") (standard true) (body brace (doc) (import (target (span (span (offset 271) (line 8) (column 18) (len 29))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (attribute-def (declaration-name "measuresOfEffectiveness") (short-name none) (typing none) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body brace (doc))) (attribute-def (declaration-name "measuresOfPerformance") (short-name none) (typing none) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body brace (doc))) (metadata-def (name "MeasureOfEffectiveness") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1)))) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 836) (line 26) (column 19) (len 41)) (meta-cast (base (expression (span (offset 836) (line 26) (column 19) (len 23)) (ref r5))) (metaclass (ref r6))))))) (body semicolon)))) (metadata-def (name "MeasureOfPerformance") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r7)))) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r9)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r10)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1145) (line 37) (column 19) (len 39)) (meta-cast (base (expression (span (offset 1145) (line 37) (column 19) (len 21)) (ref r11))) (metaclass (ref r12))))))) (body semicolon)))))))
)
~~~
