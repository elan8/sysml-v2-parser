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
            toolName = "ModelCenter";
            uri = "aserv://localhost/Vehicle/Equation1";
        }
        in dt : ISQ::TimeValue {
            @ToolVariable {
                name = "deltaT";
            }
        }
        in a : ISQ::AccelerationValue {
            @ToolVariable {
                name = "mass";
            }
        }
        in v_in : ISQ::SpeedValue {
            @ToolVariable {
                name = "v0";
            }
        }
        in x_in : ISQ::LengthValue {
            @ToolVariable {
                name = "x0";
            }
        }
        out v_out : ISQ::SpeedValue {
            @ToolVariable {
                name = "v";
            }
        }
        out x_out : ISQ::LengthValue {
            @ToolVariable {
                name = "x";
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
    (reference r1 (scope relative) (span (offset 129) (line 7) (column 4) (len 8)) (segments (segment 0 (token "toolName") (name "toolName") (separator none) (span (offset 129) (line 7) (column 4) (len 8)))))
    (reference r2 (scope relative) (span (offset 158) (line 8) (column 4) (len 3)) (segments (segment 0 (token "uri") (name "uri") (separator none) (span (offset 158) (line 8) (column 4) (len 3)))))
  )
  (root (package (name "Metadata Example-2") (body brace (action-usage (name "computeDynamics") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (import (target (span (span (offset 76) (line 4) (column 18) (len 18))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 91) (line 4) (column 33) (len 3))) (separator (span (offset 91) (line 4) (column 33) (len 2))) (marker (span (offset 93) (line 4) (column 35) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (metadata-usage (declaration-name "ToolExecution") (type none) (about) (body brace (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r1)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 140) (line 7) (column 15) (len 13)) (string "ModelCenter"))))) (body semicolon)) (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r2)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 164) (line 8) (column 10) (len 37)) (string "aserv://localhost/Vehicle/Equation1"))))) (body semicolon)))) (in-out-declaration) (in-out-declaration) (in-out-declaration) (in-out-declaration) (in-out-declaration) (in-out-declaration))))))
)
~~~
