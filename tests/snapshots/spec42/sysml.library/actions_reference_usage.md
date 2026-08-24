# META
~~~sexpr
(snapshot (type semantic) (description "Direct retained context from Systems Library/Actions.sysml: SendAction and AcceptMessageAction use the pinned ReferenceUsage ordering `ref name :>> redefinedFeature: Type, Type`. Both action-body declarations and their directed reference-body members remain typed and format canonically."))
~~~
# SOURCE
~~~sysml
standard library package Actions {
    action def SendAction :> Action, SendPerformance {
        in :>> payload [0..*];
        ref sentMessage :>> sentTransfer: MessageTransfer, MessageAction {
            in :>> MessageTransfer::payload, MessageAction::payload;
        }
    }
    action def AcceptMessageAction :> Action, AcceptPerformance {
        inout :>> payload;
        ref acceptedMessage :>> acceptedTransfer: MessageTransfer, MessageAction {
            in :>> MessageTransfer::payload, MessageAction::payload;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "actions_reference_usage.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package Actions {
    action def SendAction :> Action, SendPerformance {
        in :>> payload[0..*];
        ref sentMessage : MessageTransfer, MessageAction :>> sentTransfer {
            in :>> MessageTransfer::payload, MessageAction::payload;
        }
    }
    action def AcceptMessageAction :> Action, AcceptPerformance {
        inout :>> payload;
        ref acceptedMessage : MessageTransfer, MessageAction :>> acceptedTransfer {
            in :>> MessageTransfer::payload, MessageAction::payload;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 64) (line 2) (column 30) (len 6)) (segments (segment 0 (token "Action") (name "Action") (separator none) (span (offset 64) (line 2) (column 30) (len 6)))))
    (reference r1 (scope relative) (span (offset 72) (line 2) (column 38) (len 15)) (segments (segment 0 (token "SendPerformance") (name "SendPerformance") (separator none) (span (offset 72) (line 2) (column 38) (len 15)))))
    (reference r2 (scope relative) (span (offset 105) (line 3) (column 16) (len 7)) (segments (segment 0 (token "payload") (name "payload") (separator none) (span (offset 105) (line 3) (column 16) (len 7)))))
    (reference r3 (scope relative) (span (offset 163) (line 4) (column 43) (len 15)) (segments (segment 0 (token "MessageTransfer") (name "MessageTransfer") (separator none) (span (offset 163) (line 4) (column 43) (len 15)))))
    (reference r4 (scope relative) (span (offset 180) (line 4) (column 60) (len 13)) (segments (segment 0 (token "MessageAction") (name "MessageAction") (separator none) (span (offset 180) (line 4) (column 60) (len 13)))))
    (reference r5 (scope relative) (span (offset 149) (line 4) (column 29) (len 12)) (segments (segment 0 (token "sentTransfer") (name "sentTransfer") (separator none) (span (offset 149) (line 4) (column 29) (len 12)))))
    (reference r6 (scope relative) (span (offset 319) (line 8) (column 39) (len 6)) (segments (segment 0 (token "Action") (name "Action") (separator none) (span (offset 319) (line 8) (column 39) (len 6)))))
    (reference r7 (scope relative) (span (offset 327) (line 8) (column 47) (len 17)) (segments (segment 0 (token "AcceptPerformance") (name "AcceptPerformance") (separator none) (span (offset 327) (line 8) (column 47) (len 17)))))
    (reference r8 (scope relative) (span (offset 365) (line 9) (column 19) (len 7)) (segments (segment 0 (token "payload") (name "payload") (separator none) (span (offset 365) (line 9) (column 19) (len 7)))))
    (reference r9 (scope relative) (span (offset 424) (line 10) (column 51) (len 15)) (segments (segment 0 (token "MessageTransfer") (name "MessageTransfer") (separator none) (span (offset 424) (line 10) (column 51) (len 15)))))
    (reference r10 (scope relative) (span (offset 441) (line 10) (column 68) (len 13)) (segments (segment 0 (token "MessageAction") (name "MessageAction") (separator none) (span (offset 441) (line 10) (column 68) (len 13)))))
    (reference r11 (scope relative) (span (offset 406) (line 10) (column 33) (len 16)) (segments (segment 0 (token "acceptedTransfer") (name "acceptedTransfer") (separator none) (span (offset 406) (line 10) (column 33) (len 16)))))
  )
  (root (library-package (name "Actions") (standard true) (body brace (action-def (name "SendAction") (modifiers) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0) (ref r1)))) (body brace (in-out (direction in) (reference false) (declaration none) (subsets none) (type none) (multiplicity (lower (expression (span (offset 114) (line 3) (column 25) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (value none) (span (offset 98) (line 3) (column 9) (len 22))) (ref (name "sentMessage") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3) (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (subsets none) (body brace (in-out-declaration))))) (action-def (name "AcceptMessageAction") (modifiers) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r6) (ref r7)))) (body brace (in-out (direction inout) (reference false) (declaration none) (subsets none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (value none) (span (offset 355) (line 9) (column 9) (len 18))) (ref (name "acceptedMessage") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9) (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (subsets none) (body brace (in-out-declaration))))))))
)
~~~
