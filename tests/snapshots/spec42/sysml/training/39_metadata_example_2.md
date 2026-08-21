# META
~~~sexpr
(snapshot (type semantic) (description "Upstream SysML training corpus: 39. Metadata/Metadata Example-2.sysml. Its action usage directly owns a private namespace import followed by a MetadataUsage and typed in/out parameters; ActionBodyItem admits Import through NonBehaviorBodyItem (SysML textual BNF 894-917; pinned Pilot SysML.xtext 1361-1381)."))
~~~
# SOURCE
~~~sysml
package 'Metadata Example-2' {
	
	action computeDynamics {
		private import AnalysisTooling::*;
		
		metadata ToolExecution {
			toolName = "ModelCenter";
			uri = "aserv://localhost/Vehicle/Equation1";
		}
			
		in dt : ISQ::TimeValue             { @ToolVariable { name = "deltaT"; } }
		in a : ISQ::AccelerationValue      { @ToolVariable { name = "mass"; } }
		in v_in : ISQ::SpeedValue          { @ToolVariable { name = "v0"; } }
		in x_in : ISQ::LengthValue         { @ToolVariable { name = "x0"; } }
		
		out v_out : ISQ::SpeedValue        { @ToolVariable { name = "v"; } }
		out x_out : ISQ::LengthValue       { @ToolVariable { name = "x"; } }			
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "39_metadata_example_2.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Metadata Example-2' {
    action computeDynamics {
        private import AnalysisTooling::*;
        metadata ToolExecution {
            attribute toolName = "ModelCenter";
            attribute uri = "aserv://localhost/Vehicle/Equation1";
        }
        in dt : ISQ::TimeValue {
            @ToolVariable {
                attribute name = "deltaT";
            }
        }
        in a : ISQ::AccelerationValue {
            @ToolVariable {
                attribute name = "mass";
            }
        }
        in v_in : ISQ::SpeedValue {
            @ToolVariable {
                attribute name = "v0";
            }
        }
        in x_in : ISQ::LengthValue {
            @ToolVariable {
                attribute name = "x0";
            }
        }
        out v_out : ISQ::SpeedValue {
            @ToolVariable {
                attribute name = "v";
            }
        }
        out x_out : ISQ::LengthValue {
            @ToolVariable {
                attribute name = "x";
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 76) (line 4) (column 18) (len 15)) (segments (segment 0 (token "AnalysisTooling") (name "AnalysisTooling") (separator none) (span (offset 76) (line 4) (column 18) (len 15)))))
  )
  (root (package (name "Metadata Example-2") (body brace (action-usage (name "computeDynamics") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (import (target (span (span (offset 76) (line 4) (column 18) (len 18))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 91) (line 4) (column 33) (len 3))) (separator (span (offset 91) (line 4) (column 33) (len 2))) (marker (span (offset 93) (line 4) (column 35) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (metadata-usage (declaration-name "ToolExecution") (type none) (about) (body brace (attribute-usage (declaration-name "toolName") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 140) (line 7) (column 15) (len 13)) (string "ModelCenter"))))) (body semicolon)) (attribute-usage (declaration-name "uri") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 164) (line 8) (column 10) (len 37)) (string "aserv://localhost/Vehicle/Equation1"))))) (body semicolon)))) (in-out-declaration) (in-out-declaration) (in-out-declaration) (in-out-declaration) (in-out-declaration) (in-out-declaration))))))
)
~~~
