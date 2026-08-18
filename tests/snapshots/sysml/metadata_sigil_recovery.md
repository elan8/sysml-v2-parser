# META
~~~sexpr
(snapshot (type recovery) (description "Unsupported and malformed sigil syntax stay distinct states with distinct codes. A well-formed head the scope does not model is `unsupported_annotation_syntax` (a warning: legal SysML, not yet parsed here); a sigil with no qualified name behind it is `malformed_annotation_head` (an error: no production spells it). Both recover at their authored span and leave the valid siblings before and after them intact, including when the recovered region contains comments, quoted text and escaped delimiters."))
~~~
# SOURCE
~~~sysml
package MetadataSigilRecovery {
    part def Before;
    part def UnsupportedHeads {
        part beforeUnsupported;
        #tag : Foo::Bar weirdstuff;
        @tag : Foo::Bar weirdstuff;
        part afterUnsupported;
    }
    part def MalformedHeads {
        part beforeMalformed;
        #;
        @ ;
        #::Leading;
        part afterMalformed;
    }
    part def RecoveryRegionContents {
        part beforeRegion;
        #tag /* } not a real brace */ "a } in a string" '\}' weirdstuff;
        part afterRegion;
    }
    part def After;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "metadata_sigil_recovery.md"
    (diagnostics
      (diagnostic (code "unsupported_annotation_syntax") (severity warning) (category unsupportedgrammarform) (span (offset 125) (line 5) (column 9) (len 36)) (message "incomplete parser support for metadata syntax in part definition body"))
      (diagnostic (code "unsupported_annotation_syntax") (severity warning) (category unsupportedgrammarform) (span (offset 161) (line 6) (column 9) (len 36)) (message "incomplete parser support for metadata syntax in part definition body"))
      (diagnostic (code "malformed_annotation_head") (severity error) (category parseerror) (span (offset 294) (line 11) (column 9) (len 11)) (message "malformed metadata reference in part definition body"))
      (diagnostic (code "malformed_annotation_head") (severity error) (category parseerror) (span (offset 305) (line 12) (column 9) (len 12)) (message "malformed metadata reference in part definition body"))
      (diagnostic (code "malformed_annotation_head") (severity error) (category parseerror) (span (offset 317) (line 13) (column 9) (len 20)) (message "malformed metadata reference in part definition body"))
      (diagnostic (code "unsupported_annotation_syntax") (severity warning) (category unsupportedgrammarform) (span (offset 437) (line 18) (column 9) (len 73)) (message "incomplete parser support for metadata syntax in part definition body"))
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
  )
  (root (package (name "MetadataSigilRecovery") (body brace (part-def (name "Before") (body semicolon)) (part-def (name "UnsupportedHeads") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "beforeUnsupported") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "unsupported_annotation_syntax") (found "#tag : Foo::Bar weirdstuff;") (span (offset 125) (line 5) (column 9) (len 36))) (malformed (code "unsupported_annotation_syntax") (found "@tag : Foo::Bar weirdstuff;") (span (offset 161) (line 6) (column 9) (len 36))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterUnsupported") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "MalformedHeads") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "beforeMalformed") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "malformed_annotation_head") (found "#;") (span (offset 294) (line 11) (column 9) (len 11))) (malformed (code "malformed_annotation_head") (found "@ ;") (span (offset 305) (line 12) (column 9) (len 12))) (malformed (code "malformed_annotation_head") (found "#::Leading;") (span (offset 317) (line 13) (column 9) (len 20))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterMalformed") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "RecoveryRegionContents") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "beforeRegion") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "unsupported_annotation_syntax") (found "#tag /* } not a real brace */ \"a } in a string\" '\\}' weirdst") (span (offset 437) (line 18) (column 9) (len 73))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterRegion") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "After") (body semicolon)))))
)
~~~
