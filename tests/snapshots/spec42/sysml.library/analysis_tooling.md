# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Analysis/AnalysisTooling"))
~~~
# SOURCE
~~~sysml
standard library package AnalysisTooling {
	doc
	/*
	 * This package contains definitions for metadata annotations related
	 * to analysis tool integration.
	 */

	private import ScalarValues::*;
	
	metadata def ToolExecution {
		doc
		/*
		 * ToolExecution metadata identifies an external analysis tool to be
		 * used to implement the annotated action.
		 */
	
		attribute toolName : String;
		attribute uri : String;
	}
	
	metadata def ToolVariable {
		doc
		/*
		 * ToolVariable metadata is used in the context of an action that has
		 * been annotated with ToolExecution metadata. It is used to annotate
		 * a parameter or other feature of the action with the name of the
		 * variable in the tool that is to correspond to the annotated
		 * feature.
		 */
	
		attribute name : String;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "analysis_tooling.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package AnalysisTooling {
    doc
    /*
	 * This package contains definitions for metadata annotations related
	 * to analysis tool integration.
	 */
    private import ScalarValues::*;
    metadata def ToolExecution {
        doc
        /*
		 * ToolExecution metadata identifies an external analysis tool to be
		 * used to implement the annotated action.
		 */
        attribute toolName : String;
        attribute uri : String;
    }
    metadata def ToolVariable {
        doc
        /*
		 * ToolVariable metadata is used in the context of an action that has
		 * been annotated with ToolExecution metadata. It is used to annotate
		 * a parameter or other feature of the action with the name of the
		 * variable in the tool that is to correspond to the annotated
		 * feature.
		 */
        attribute name : String;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 179) (line 8) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 179) (line 8) (column 17) (len 12)))))
  )
  (root (library-package (name "AnalysisTooling") (standard true) (body (doc) (import (target (span (span (offset 179) (line 8) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 191) (line 8) (column 29) (len 3))) (separator (span (offset 191) (line 8) (column 29) (len 2))) (marker (span (offset 193) (line 8) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (metadata-def) (metadata-def))))
)
~~~
