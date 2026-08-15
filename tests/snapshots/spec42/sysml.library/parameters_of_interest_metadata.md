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
    attribute def measuresOfEffectiveness nonunique {
        doc
        /* Base feature for attributes that are measures of effectiveness. */
    }
    attribute def measuresOfPerformance nonunique {
        doc
        /* Base feature for attributes that are measures of performance. */
    }
    metadata def <moe> MeasureOfEffectiveness :> SemanticMetadata {
        doc
        /*
	 	 * MeasureOfEffectiveness is semantic metadata for identifying an attribute as a
	 	 * measure of effectiveness.
	 	 */
        attribute annotatedElement : SysML::Usage :>> annotatedElement;
        attribute baseType :>> baseType = measuresOfEffectiveness meta SysML::Usage;
    }
    metadata def <mop> MeasureOfPerformance :> SemanticMetadata {
        doc
        /*
	 	 * MeasureOfPerformance is semantic metadata for identifying an attribute as a
	 	 * measure of performance.
	 	 */
        attribute annotatedElement : SysML::Usage :>> annotatedElement;
        attribute baseType :>> baseType = measuresOfPerformance meta SysML::Usage;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 271) (line 8) (column 18) (len 29)) (segments (segment 0 (token "Metaobjects") (name "Metaobjects") (separator none) (span (offset 271) (line 8) (column 18) (len 11))) (segment 1 (token "SemanticMetadata") (name "SemanticMetadata") (separator colon-colon) (span (offset 284) (line 8) (column 31) (len 16)))))
  )
  (root (library-package (name "ParametersOfInterestMetadata") (standard true) (body brace (doc) (import (target (span (span (offset 271) (line 8) (column 18) (len 29))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (attribute-def) (attribute-def) (metadata-def) (metadata-def))))
)
~~~
