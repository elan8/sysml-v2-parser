# META
~~~sexpr
(snapshot (type semantic) (description "ActionBodyItem admits NonBehaviorBodyItem, including MetadataUsage (SysML textual BNF 899-915 and 1666-1673; pinned Pilot SysML 1368-1372 and 140-147), and ActionNode, including TerminateNode (SysML 956-964 and 1116-1121; Pilot 1641-1647). The semicolon forms represented by the current typed AST render in both action definition and action usage bodies; each MetadataUsage owns source-backed MetadataBodyUsage reference members and each terminate target remains an expression through FORMAT reparse/idempotence."))
~~~
# SOURCE
~~~sysml
package ActionBodyMetadataAndTerminateEmit {
    action def Workflow {
        metadata ToolExecution {
            toolName = "definition";
        }
        terminate definitionTarget;
    }
    action workflow {
        metadata ToolExecution {
            toolName = "usage";
        }
        terminate usageTarget;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "action_body_metadata_and_terminate_emit.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sexpr
(stable-idempotent)
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 116) (line 4) (column 13) (len 8)) (segments (segment 0 (token "toolName") (name "toolName") (separator none) (span (offset 116) (line 4) (column 13) (len 8)))))
    (reference r1 (scope relative) (span (offset 169) (line 6) (column 19) (len 16)) (segments (segment 0 (token "definitionTarget") (name "definitionTarget") (separator none) (span (offset 169) (line 6) (column 19) (len 16)))))
    (reference r2 (scope relative) (span (offset 260) (line 10) (column 13) (len 8)) (segments (segment 0 (token "toolName") (name "toolName") (separator none) (span (offset 260) (line 10) (column 13) (len 8)))))
    (reference r3 (scope relative) (span (offset 308) (line 12) (column 19) (len 11)) (segments (segment 0 (token "usageTarget") (name "usageTarget") (separator none) (span (offset 308) (line 12) (column 19) (len 11)))))
  )
  (root (package (name "ActionBodyMetadataAndTerminateEmit") (body brace (action-def (name "Workflow") (modifiers) (specializes none) (body brace (metadata-usage (declaration-name "ToolExecution") (type none) (about) (body brace (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r0)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 127) (line 4) (column 24) (len 12)) (string "definition"))))) (body semicolon)))) (terminate (target (expression (span (offset 169) (line 6) (column 19) (len 16)) (ref r1)))))) (action-usage (keyword action) (name "workflow") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (metadata-usage (declaration-name "ToolExecution") (type none) (about) (body brace (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r2)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 271) (line 10) (column 24) (len 7)) (string "usage"))))) (body semicolon)))) (terminate (target (expression (span (offset 308) (line 12) (column 19) (len 11)) (ref r3)))))))))
)
~~~
