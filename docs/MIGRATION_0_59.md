# Migrating to v0.59.0

This guide covers upgrading from v0.58.0. The crate version is `0.59.0`, the minimum Rust version
stays **1.88**, and `PARSE_AST_VERSION` moves from **284 to 289**. The intermediate AST numbers in
commit trailers describe development changes within this release; 289 is the released schema to
target. Reparse source documents to rebuild caches. Serialized v0.58.0 envelopes are rejected
rather than upgraded by deserialization.

## Model changes

Text that used to parse is now an error, because the pinned grammar does not allow it:

| Before | After |
| --- | --- |
| `private /* c */ part def A;` | Use a note: `private //* c */ part def A;` |
| `case def C { 1 + 2; }` | Remove the `;` from the result expression |
| `calc def C { 1 + 2 attribute a; }` | Move the result expression to the end of the body |
| `library standard package P;` | `standard library package P;` |

A `/* */` comment at a member position stays a `Comment` member.

## Consumer changes

- `Body::Brace` replaces `close_span: Span` with `close: BodyClose`. Use `close.span()` for the
  close position or `close.authored_span()` for the `}` token. `Visitor::visit_body_braces` receives
  `&BodyClose`.
- `Import.membership` is now `Import.visibility: ImportVisibility`, and `Import.body_elements` is now
  `Import.body: Body<RelationshipBodyElement>`.
- `Package` and `LibraryPackage` gain `membership: Membership`. `CalcDefBodyPackageMember` and
  `CalcDefBodyLibraryPackageMember` no longer carry a separate `membership`; read
  `package.value.membership`.
- `InOutDecl.body` is now `ActionDefBody`, and `KermlRelationshipDecl.body` is now
  `Body<RelationshipBodyElement>`, replacing `Option<Vec<_>>`. Use `is_semicolon()` and
  `braced_elements()`.
- `IncludeUseCase.short_name`, `name` and `typing` are replaced by
  `declaration: Option<Node<UsageDeclaration>>`, and `multiplicity` applies to the reference form
  only.
- `AssignStmt` and `TerminateStmt` gain a `prefix: ActionNodePrefix` field, and `TerminateStmt` gains
  `is_then`.
- `UseCaseDefBodyElement` gains `ActionMember`. A bare `name;` in a case, calculation or constraint
  body is now a `DefaultReferenceUsage`, not an expression.
- Diagnostic lists can contain non-fatal warnings. Use `ParseResult::is_ok` or
  `ParseError::prevents_strict_parsing` to decide whether parsing succeeded.
- `emit_sysml` refuses an unterminated body and a bare import. Use `emit_recovered_sysml` to reprint
  recovered editor documents.
