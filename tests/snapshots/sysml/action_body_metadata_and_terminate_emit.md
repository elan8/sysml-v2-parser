# META
~~~sexpr
(snapshot (type semantic) (description "ActionBodyItem admits NonBehaviorBodyItem, including MetadataUsage (SysML textual BNF 899-915 and 1666-1673; pinned Pilot SysML 1368-1372 and 140-147), and ActionNode, including TerminateNode (SysML 956-964 and 1116-1121; Pilot 1641-1647). The semicolon forms represented by the current typed AST render in both action definition and action usage bodies; each MetadataUsage keeps its typed attribute body and each terminate target remains an expression through FORMAT reparse/idempotence."))
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
~~~sysml
package ActionBodyMetadataAndTerminateEmit {
    action def Workflow {
        metadata ToolExecution {
            attribute toolName = "definition";
        }
        terminate definitionTarget;
    }
    action workflow {
        metadata ToolExecution {
            attribute toolName = "usage";
        }
        terminate usageTarget;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 169) (line 6) (column 19) (len 16)) (segments (segment 0 (token "definitionTarget") (name "definitionTarget") (separator none) (span (offset 169) (line 6) (column 19) (len 16)))))
    (reference r1 (scope relative) (span (offset 308) (line 12) (column 19) (len 11)) (segments (segment 0 (token "usageTarget") (name "usageTarget") (separator none) (span (offset 308) (line 12) (column 19) (len 11)))))
  )
  (root (package (name "ActionBodyMetadataAndTerminateEmit") (body brace (action-def (name "Workflow") (modifiers) (specializes none) (body brace (metadata-usage (declaration-name "ToolExecution") (type none) (about) (body brace (attribute-usage (declaration-name "toolName") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 127) (line 4) (column 24) (len 12)) (string "definition"))))) (body semicolon)))) (terminate (target (expression (span (offset 169) (line 6) (column 19) (len 16)) (ref r0)))))) (action-usage (name "workflow") (short-name none) (body brace (metadata-usage (declaration-name "ToolExecution") (type none) (about) (body brace (attribute-usage (declaration-name "toolName") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 271) (line 10) (column 24) (len 7)) (string "usage"))))) (body semicolon)))) (terminate (target (expression (span (offset 308) (line 12) (column 19) (len 11)) (ref r1)))))))))
)
~~~
