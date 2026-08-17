# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- **A calculation body owns its action members, and `ref calc` parses as one declaration.**
  `CalculationBodyItem = ActionBodyItem | ReturnParameterMember`, so a calculation body holds every
  action-body member. It held none of them, and the failure was silent rather than a diagnostic:
  the body's keyword-less `DefaultReferenceUsage` fallback read each action keyword as a feature
  name, so `first f;` parsed as two invented members -- `'first';` and `f;` -- and formatted back
  that way. The same fallback swallowed the `ref` of `BasicUsagePrefix = RefPrefix
  ( isReference ?= 'ref' )?`, so `ref calc self : Calculation :>> Action::self;` in the checked-in
  `Calculations.sysml` fixture emitted as `'ref';` on its own line followed by a `calc` usage that
  had lost its prefix. `CalcUsage::is_reference` retains the keyword and the calculation body now
  routes action members to the action dispatcher. Only a calculation reaches it: `calc_def_body`
  also serves KerML type bodies, whose `TypeBodyElement` has no action-node alternative.
  **AST version 160 -> 161.**

- **A part usage body takes the view family, and a part definition body can emit it.**
  `ViewUsage`, `ViewpointUsage` and `RenderingUsage` are usage-element alternatives and their
  three definitions are `DefinitionElement` alternatives, so `UsageBody = DefinitionBody` admits
  all six in a part usage body exactly as in a part definition body. The part usage scope modelled
  none of them -- `rendering r { ... }` inside `part p { ... }` reached recovery -- and the part
  definition scope, which parsed all six, had every one of them in its emitter's `unsupported`
  group, so a document with a nested view definition parsed and then could not be formatted.
  **AST version 159 -> 160.**

- **A declared `interface` usage with no typing parses.** `InterfaceUsageDeclaration`'s
  `UsageDeclaration` makes the `: Type` optional and its `( 'connect' InterfacePart )?` optional
  too, but the parser reached a name only through the typed spelling or the `connect` spelling.
  `interface i;` and `interface i { ... }` left the name unconsumed, the body parser then failed
  on it, and the whole member went to recovery -- while `interface i : I { ... }` parsed.
  **AST version 158 -> 159.**

- **A `flow def` can be formatted.** `FlowDefinition = OccurrenceDefinitionPrefix ( 'flow' |
  'message' ) 'def' Definition` is a `DefinitionElement`, so it is legal at package level and in a
  part definition or part usage body. It parsed into a complete typed node in all three, and all
  three emitters reported it as an unsupported construct -- a document containing one parsed
  cleanly and then could not be emitted at all. Its body is a `DefinitionBody`, the same shape
  `emit_allocation_def` already wrote.

### Added

- **Two more `examples/` files round-trip.** `Simple Tests/TextualRepresentationTest.sysml` and
  `Metadata Examples/VerificationMetadataExample.sysml` are promoted into
  `EXAMPLES_ROUNDTRIP_PASS`: the first needed a textual representation to be dispatched in action
  and KerML type bodies at all, the second needed `@` metadata annotations to emit from the scopes
  that own them and a comment to keep the elements its `about` clause names.

- **A comment's `about` clause is parsed instead of scanned past.**
  `Comment = ( 'comment' Identification ( 'about' Annotation ( ',' Annotation )* )? )? …`, and the
  clause was skipped with `take_until("/*")` -- a raw substring search with no bound. It ran past
  the comment's own end, past the enclosing `}`, and through however many later declarations it
  took to reach a block comment, discarding every one of them with no diagnostic:
  `comment about` with no target consumed the `attribute mass;` after it, the closing brace, and
  the whole next `part def`. The annotated elements were dropped too, and so was a `locale` that
  followed them. `CommentAnnotation::about_targets` holds them as the qualified references they
  are, emission reproduces them, and an incomplete clause is a recovery node with an exact span
  that later siblings survive. Three checked-in spec fixtures get their annotated elements back.
  **AST version 157 -> 158.**

- **An `enum def` body keeps its annotating members instead of silently dropping them.**
  `EnumerationBody` is the one production that names the membership directly -- `';' | '{'
  ( ownedRelationship += AnnotatingMember | ownedRelationship += EnumerationUsageMember )* '}'` --
  and the body parser recognised `doc` and `comment` only to discard them: no node, no span, no
  diagnostic. `rep` and `@` were not recognised at all, and on any member it could not parse it
  ran to the closing brace and dropped everything in between, still with no diagnostic. A body
  with four annotating members and three values parsed clean and kept two values. It now goes
  through the shared brace-member routine, so annotating members are retained in authored order
  beside the values and a malformed member becomes an `Error` node with an exact span that later
  values survive. Documentation reappears in the checked-in library fixtures that had it --
  `RiskMetadata`, `15.10-Primitive Data Types`, `documentation_in_bodies`. `EnumerationBody` is
  now `Body<EnumerationBodyElement>` rather than `Body<EnumeratedValue>`, and the semantic
  projection names each member instead of the definition as a whole. **AST version 156 -> 157.**

- **Package bodies use the shared annotating family.** No new syntax -- this scope already
  accepted all four -- but its four variants collapse into `Annotating(AnnotatingMember)`, so the
  emitter, the projection and the traversal reach the production through one path here too. The
  per-production FIRST-set guards are kept: each alternative still dispatches under its own
  `PackageProduction` tag, so scope drift is still detected per alternative.

- **Requirement, case, constraint, calculation, view and rendering bodies accept the whole
  annotating production, and a KerML type body stops shredding a `rep`.** `RequirementBodyItem`
  extends `DefinitionBodyItem`, `CaseBodyItem` and `CalculationBodyItem` extend `ActionBodyItem`,
  and `ViewDefinitionBodyItem`/`ViewBodyItem` extend `DefinitionBodyItem`, so all nine scopes own
  the production; between them they accepted eight of the thirty-six alternative-scope pairs. The
  worst case was the KerML type body, which shares `CalcDefBodyElement`: `rep x language "text"
  /* … */` was not dispatched at all, and the fallback member parser broke it into four invented
  members -- `'rep'; x; 'language'; "text";` -- with no diagnostic, so the document parsed clean
  and formatted back as something else. A constraint or calculation body containing an `@`
  metadata annotation parsed but could not be formatted at all. **AST version 155 -> 156.**

- **Action, action-usage, state and control-node bodies accept the whole annotating production,
  and can emit it.** `ActionBodyItem` and `StateBodyItem` both start at `NonBehaviorBodyItem`,
  which reaches `AnnotatingElement` through `DefinitionMember`, so `comment /* ... */` belongs in
  all of them and was a parse error in all of them. The other three alternatives parsed but could
  not be formatted: `emit_action_def_body`, `emit_action_usage_body` and `emit_state_def_body`
  each reported a metadata annotation or a textual representation as an unsupported construct, so
  a document was parseable and unformattable at the same time. All four now go through the shared
  annotating emitter. Control-node bodies (`first`, `merge`, `decide`, `join`, `fork`) inherit the
  change with the action-body member set they already share. **AST version 154 -> 155.**

- **Eleven structural body scopes accept the whole annotating production.** Part, attribute, port,
  connection, interface and occurrence definitions, port, interface and perform usages, KerML
  feature bodies, and metadata bodies each admitted documentation and nothing else -- or, for part
  definitions, everything but a textual representation -- while the grammar reaches
  `AnnotatingElement` in all of them through `DefinitionBodyItem -> DefinitionMember ->
  DefinitionElement` (`NonFeatureMember -> MemberElement` on the KerML side). They now carry
  `Annotating(AnnotatingMember)` and dispatch through the one parser, so `comment`, `rep` and the
  `@` metadata spelling parse, emit and traverse identically wherever the production is legal. The
  derived evidence is `planning/annotating-member-matrix.md`. **AST version 153 -> 154.**

- **A comment's keyword span is validated on deserialization.** `comment` is optional in the
  production and its presence is the only thing separating a member from a bare block comment,
  which reparses as trivia and disappears -- so emission reads the span, and a wire document could
  redirect it at another comment's keyword and silently change what the document says. It is now
  checked like a delimiter: it must slice to `comment`, and it must lie inside the comment that
  owns it rather than merely inside the enclosing declaration.

### Removed

- **`DefinitionBodyElement::Doc`, which the parser could not build.** Documentation in a flow,
  allocation or message body has always arrived as
  `OccurrenceMember(OccurrenceBodyElement::Doc)`; the sibling variant had no construction site
  anywhere, so it was an unreachable state in a public enum and a second representation of the
  same syntactic fact.

### Fixed

- **An anonymous connection definition no longer emits a doubled space.** The trailing space was
  written with the `connection def` keyword rather than with the identification, so
  `#derivation connection { ... }` formatted as `#derivation connection def  {`.

- **Two adjacent comment members no longer fuse into one.** A comment's optional `locale`
  lookahead skipped block comments as trivia, so it walked past the member's own body and found
  the *next* member's `locale`: `comment named /* two */` followed by `locale "en_US" /* three */`
  parsed as a single comment named `named`, in locale `en_US`, whose text was ` three `. The
  second member's text was discarded with no diagnostic. `doc_comment` had the identical
  lookahead and is fixed with it.

- **An action-body `ref` declaration keeps its kind keyword, multiplicity and `:>` clause.**
  `action_ref_decl_inner` parsed the `action` keyword and the multiplicity only to discard them,
  never parsed a subsetting clause at all, and ended with a skip-to-terminator that swallowed
  whatever was left -- so `derived ref action deferred : ActionUsage :> Metadata::metadataItems;`
  formatted back as `derived ref deferred : ActionUsage;`. **AST version 152 -> 153.**

### Changed

- **A calculation definition projects its body.** It was a bare `(calc-def)` marker, so no
  snapshot could show whether a calculation body member survived parsing -- which is how the
  shredding above stayed invisible. Action members reuse the exhaustive `ActionDefBodyElement`
  writer rather than restating it.

- **A package-level part usage projects its typing and its body.** It was a bare `(part-usage)`
  marker, so a snapshot could not show any member of `part p { ... }` written at package level,
  and the usage's own type reference never appeared in the projection's reference table.
  `PartUsageBody` and `RefBody` are both `Body<PartUsageBodyElement>`, so the exhaustive element
  match that ref bodies already used covers this scope unchanged -- nothing called it. 112 fixtures
  gain body detail; the reference tables renumber because the table is built from what the
  projection reaches, in the order it reaches it.

- **Ref and attribute usage projections name the `RefPrefix` chain.** The semantic projection
  recorded neither `derived`/`abstract`/`variation`/`constant` nor the direction, so a snapshot
  could not distinguish `derived abstract constant ref attribute x` from a bare `attribute x`,
  and the fields added for those keywords had no structural coverage at all.

- **Body expressions accept undirected parameters, leading documentation, and parameter bodies.**
  `vertices->exists{p2 : Point; ...}` (`sysml.library/Domain Libraries/Geometry/ShapeItems.sysml`)
  declares an undirected parameter, which the grammar allows and the parser required a direction
  for; `alternatives->minimize { doc /* ... */ ... }` opens with documentation; and
  `selectOne {in ref a { doc /* ... */ } ...}` terminates a parameter with its own documented
  body instead of `;` (both `TradeStudies.sysml`). `CollectionOperatorParameter::direction`
  became optional and its `semicolon_span` became a typed `terminator`.
- **`abstract calc` keeps its keyword and its `:>` clause.** `CalcUsage` had neither field, so
  `abstract calc subcalculations : Calculation :> calculations, subactions { ... }`
  (Systems Library `Calculations.sysml`) emitted as `calc subcalculations : Calculation`.
  **AST version 151 -> 152.**

- **Calc usages take the `RefPrefix` and are requirement-body members.** `in calc eval :
  EvaluationFunction { ... }` (`sysml.library/Domain Libraries/Analysis/TradeStudies.sysml:61`)
  had no arm in requirement bodies, and `calc_usage` parsed neither the direction nor `abstract`
  -- `CalcUsage::direction` existed but nothing ever populated it, and the `abstract` keyword was
  consumed and dropped. Both now come from the shared `ref_prefix`.
  **AST version 150 -> 151.**

- **Occurrence bodies accept directed occurrence usages and connection usages.**
  `directed_occurrence_usage` was dispatched only in action bodies, and it required the
  `occurrence` keyword immediately after the direction, so `in event occurrence sourceEvent [1]
  default that.sourceEvent;` (`sysml.library/Systems Library/Flows.sysml`) failed twice over.
  `connection :HappensDuring connect sourceEvent to [1] self;` had no arm in this scope either.
- **Concern usages are requirement-body members, and keep their `abstract` and multiplicity.**
  `abstract concern concerns[0..*] :> concernChecks { ... }` (`Requirements.sysml`) was only
  reachable at package level, and `ConcernUsage` had no field for either the keyword or the
  multiplicity, both of which the parser consumed and discarded. **AST version 149 -> 150.**

- **A brace-bodied `require` / `assume` member is a constraint-body member.** The constraint body's
  terminal arm falls through to `expression`, which read `require viewpointSatisfactions` as an
  expression and then could not account for the `{` that followed
  (`sysml.library/Systems Library/Views.sysml:43`). A typed arm now precedes it.
- **Part usage bodies accept bare end declarations.** `ref :>> outgoingTransfersFromSelf :> ...
  { end ref source; end ref target; }` (`Ports.sysml:37`) puts connector ends in a usage body;
  occurrence bodies already modelled the member, part usage bodies did not.
  **AST version 148 -> 149.**

- **View definition bodies accept viewpoint usages and `satisfy requirement ... by ...`.** Both
  are parsed by the same productions package and part bodies already dispatch; this scope had no
  arm for them (`sysml.library/Systems Library/Views.sysml`).
- **`abstract view def` and `abstract rendering def` keep the keyword.** `ViewDef` and
  `RenderingDef` had no `is_abstract` field, so the definition prefix parser produced the flag
  and emission dropped it. **AST version 147 -> 148.**
- **A subject declaration accepts a multiplicity before a trailing `:>>`.** `subject subj :
  View[1] :>> RequirementCheck::subj;` (`Views.sysml`) failed because the redefinition was
  parsed first and the `[1]` blocked it. Emission also placed the multiplicity after the
  redefinition target, where it would reparse as the *target's* multiplicity; it now precedes
  the clause, as authored.
- **A connection end may be declared by name alone.** `end ref source;` (`Ports.sysml`) has no
  typing, reference subsetting or nested usage, and every branch of `end_decl` required one of
  those.

- **Nested case usages are members of case-family bodies, and keep their declaration tail.**
  `abstract case subcases : Case[0..*] :> cases, subcalculations { ... }` (`sysml.library/Systems
  Library/Cases.sysml:56`) and its `use case` / `verification` siblings had no member arm in these
  bodies, so they were recovered text. Worse, the parsers that did handle them elsewhere ran
  `usage_header` and then skipped to the body with `take_until_terminator`, discarding the
  multiplicity and, for `UseCaseUsage`, the subsets targets. All three now parse a real feature
  usage header; `CaseUsage`, `VerificationCaseUsage` and `UseCaseUsage` gained the fields to
  hold it. `Simple Tests/VerificationTest.sysml` round-trips as a result.
  **AST version 146 -> 147.**

- **A `return` declaration keeps a `:>>` clause written after its type.** `return verdict :
  VerdictKind :>> result;` (`sysml.library/Systems Library/VerificationCases.sysml:22`) only had
  a path for the leading anonymous form (`return :>> result;`), where the target stands in for
  the declaration name; a trailing clause on a named declaration was recovered text.
  `CaseReturnDecl` gained a `redefines` field, and the semantic projection shows it, so a
  regression that dropped it would be visible in the AST rather than only in the emitted string.
  **AST version 145 -> 146.**

- **Two more members that had nowhere to go.** `RefDeclKind` gained `case` and `verification`
  (`ref case self : Case :>> Calculation::self;`, `sysml.library/Systems Library/Cases.sysml`),
  matched after the two-word `use case` keyword so that form still wins. Part usage bodies gained
  the `InOutDecl` member that port and action bodies already had, so the keyword-less `in :>>
  MessageTransfer::payload, MessageAction::payload;` inside a `ref` body
  (`Actions.sysml`) is structured rather than an unexpected keyword. **AST version 144 -> 145.**

- **The `in`/`out`/`inout` direction is part of the shared `RefPrefix`.** BNF
  `RefPrefix = FeatureDirection? 'derived'? ('abstract' | 'variation')? 'constant'?` puts the
  direction first, but it was parsed ad hoc by a few callers, so `in ref alternatives :
  Anything[1..*] { ... }` (`sysml.library/Domain Libraries/Analysis/TradeStudies.sysml`) had no
  path in the scopes that had not hand-rolled one. It is now a slot of the shared prefix, and
  `RefDecl` finally records it instead of hardcoding `None`. One consequence: `out attribute
  :>> a_out : T = v;` is now the `AttributeUsage` its `attribute` keyword says it is, carrying
  the direction on `AttributeUsage::direction`, rather than an `InOutDecl`; the keyword-less
  `in x : Real;` is still an `InOutDecl`.

- **Occurrence-style definition bodies no longer swallow members behind a visibility prefix.**
  `DefinitionBody` -- shared by `flow def`, `occurrence def`, `allocation def` and the rest --
  tried its opaque unsupported-member capture *before* the structured dispatch, the reverse of
  every other body. Any member starting with `private`, `ref`, `abstract`, `in` or `connection`
  was captured whole even when the parser directly below handled it, so `private attribute
  seBeforeNum : Natural[1] = ...;` (`sysml.library/Systems Library/Flows.sysml`) was
  unsupported grammar while the same line without `private` parsed. These bodies also now
  dispatch `ref` members. **AST version 143 -> 144.**

- **`ref` members are accepted in every body whose grammar allows them.** Five scopes never
  dispatched `connector::ref_decl` at all -- port definitions, requirement definitions, view
  definitions, rendering definitions and view usages -- so `ref self : Port :>> Object::self;`
  (`sysml.library/Systems Library/Ports.sysml`) and its siblings were captured as unsupported
  grammar even though the same member parsed one scope over. `RefDeclKind` also gained the
  `concern`, `viewpoint`, `rendering`, `view` and `action` keywords, each from a library line
  that previously had nowhere to go. **AST version 142 -> 143.**

- **The `RefPrefix` modifier chain is accepted on every usage that allows it.** BNF `RefPrefix
  = 'derived'? ('abstract' | 'variation')? 'constant'?` may precede any usage keyword, but each
  parser had hand-rolled whichever subset it happened to need, so a legal prefix was a parse gap
  in the scopes that had not adopted it. `derived ref item receiverArgument : Expression[0..1]
  subsets Metadata::metadataItems;` (`sysml.library/Systems Library/SysML.sysml`) and every one
  of its 190 siblings fell through to unsupported-grammar capture. The chain is now parsed in
  one place (`parser::usage::ref_prefix`) and used by all four `RefDecl` parsers plus
  `item_usage` and `attribute_usage`. `RefDecl` and `ItemUsage` gained `is_derived`,
  `usage_prefix` and `is_constant` to hold it; `ItemUsage::is_abstract` became `usage_prefix`,
  which can also represent the `variation` alternative. **AST version 141 -> 142.**
- **A repeated specialization clause keeps every target.** Writing a subsetting-family clause
  kind twice in one header (`subsets parameter, usage subsets Metadata::metadataItems`) kept
  only the last clause, dropping the earlier targets with no diagnostic. Repeated clauses now
  accumulate into the one relationship they describe, and its span covers every authored
  fragment.
- **The `abstract` prefix on a `ref` declaration survives emission.** It was parsed and
  discarded because `RefDecl` had nowhere to put it, so `abstract ref :>> trailerHitch[1];`
  formatted as `ref :>> trailerHitch;`.
- **One emission order for the `RefPrefix` keywords.** The part, port and attribute emitters
  each spelled the chain inline and had drifted into three different orders, so `derived
  abstract x` could come back out as `abstract derived x`.

### Added

- **`ast::Body<E>`, one container for every declaration body.** Twenty-seven per-family body
  enums -- `PackageBody`, `PartDefBody`, `ActionDefBody`, and the rest -- were the same two
  alternatives written out again for each scope: `;` or `{ member* }`. They are now type
  aliases for one generic container, so the shape is stated once while the member set stays
  typed per scope: `Body<PartDefBodyElement>` and `Body<ActionDefBodyElement>` remain different
  types and a member still cannot appear in a scope whose grammar does not accept it. The
  container carries shared accessors (`is_semicolon`, `braced_elements`, `members`), and
  `braced_elements` returns `None` for a semicolon body so the `;`/`{}` distinction stays
  visible rather than flattening to an empty list.
- **One owning AST traversal boundary: `ast::visit`.** `ast::visit::Visitor` (borrowing) and
  `ast::visit::mutable::VisitorMut` (in-place transformation) cover every node reachable from
  `RootNamespace`. Both expand from a single inventory that destructures every struct without
  `..` and matches every enum without `_`, so a new field or variant is a compile error at the
  traversal until a deliberate decision is made about it -- and neither traversal direction can
  drift from the other. Consumers implement only the node kinds they have a rule for; the
  default methods walk children.

### Added

- **Body delimiters are retained.** `Body::Semicolon` keeps the `;` span and `Body::Brace` keeps
  both brace spans, captured where the parser consumes them rather than recomputed. The one
  hand-rolled reconstruction of those positions (arithmetic over consumed lengths, in the
  `first`/`merge` body) is gone, and the shared brace-member routine now returns the delimiters
  along with the members, so no scope has to consume `{` and `}` correctly on its own. There is
  Deserialization validates them against the tree, not just against themselves: a delimiter must
  slice to the token it claims, lie inside the declaration that owns it, and -- for a brace pair --
  wrap that body's own members in order. Pointing a body's delimiters at another well-formed
  `{ ... }` pair elsewhere in the document is therefore rejected. The traversal grew `enter_node`
  and `leave_node` hooks so a consumer can tell which declaration it is inside. Both alternatives require an authored token, so the container cannot represent a
  declaration with no body at all; the two places that accept one -- a `#Name` metadata keyword
  used as a prefix, and an action usage whose terminator is inferred -- hold `Option<Body<_>>`,
  which confines that state to them instead of offering it to every scope. There is deliberately
  no state for a missing closing brace: an unterminated body does not currently
  produce a body at all -- the enclosing declaration becomes a recovery node -- so a typed close
  outcome would be an unreachable variant until recovery can retain the members it read.
- **`ast::AnnotatingMember`, the grammar's annotating production as one type.** `AnnotatingElement
  = Comment | Documentation | TextualRepresentation | MetadataFeature` is a single production in
  both the KerML and SysML grammars, so the scopes that accept all of it -- relationship bodies
  and `ref` bodies -- now hold one `Annotating(AnnotatingMember)` variant instead of four parallel
  ones. One parser dispatches the production, one emitter renders it, and a `ref` body reuses both
  directly rather than translating relationship-body members into its own. Scopes that accept only
  part of the production keep their own variants until the parser supports the rest, so the type
  never claims coverage the parser lacks. `#Name` prefix metadata stays separate: it is
  `PrefixMetadataMember`, a prefix on a declaration rather than a body member.

### Changed

- **`ref` declarations project their structure in semantic snapshots.** Every scope rendered one
  as a bare `(ref)` marker, so the snapshot could not show which members a `ref` body held, in what
  order, or with what typing -- the invariant that one `ref` body parser exists to guarantee. One
  projection now serves every owner, recursing into nested `ref` declarations, and reference
  labels appear for the identities inside them, which were previously absent from the snapshot's
  reference list entirely.

### Fixed

- **AST equality compares authored syntax that emission depends on.** Three hand-written
  `PartialEq` impls excluded fields that are not provenance: `AttributeUsage` ignored the `ref` and
  `abstract` prefixes, `RefDecl` ignored its direction, and `TypingRelationship` ignored whether the
  author wrote `:>` or `specializes`. All three drive emitted output, so a formatter that dropped or
  swapped one would have passed every whole-AST comparison in the suite -- including the round-trip
  tests whose purpose is to catch exactly that. Span exclusion is unchanged: position is still not
  identity.
- **A constraint body accepts a feature declaration.** `constraint c { mass : Real; }` parsed
  `mass` as a bare expression and left `: Real;` for recovery, which the opaque capture then hid.
  A constraint definition body is a `DefinitionBody`, so it owns usages as well as the constraint
  expression. Two more release examples now round-trip: `Analysis Examples/Dynamics.sysml` and
  `Simple Tests/ConstraintTest.sysml`.
- **A redundant `;` between body members is separator punctuation.** It reached the member parser
  and was reported as unrecognized content; it is now consumed where members are collected.
- **An import with a braced body could not be serialized.** Its target span ran past the reference
  to wherever suffix parsing stopped, swallowing the whitespace before `{`, so the document failed
  the crate's own provenance validation -- `15_10_primitive_data_types` in the snapshot corpus was
  one. The target now ends at its last authored token. A new corpus test serializes and
  round-trips every snapshot document, so provenance is checked against every construct the parser
  handles rather than a handful of fixtures.
- **Emission no longer invents a `;` for a body that was never written.** A `#Name` prefix on a
  declaration and an action usage whose terminator the parser infers both stored their absent body
  as the semicolon form, so formatting wrote a `;` the author never typed -- splitting
  `#situation x : T;` into two members and `action a accept M via v;` into a member plus a stray
  clause. `MetadataKeywordUsage::body` and `ActionUsage::body` are now `Option`, keeping "no body
  was written" distinct from "a semicolon was written" without adding that state to every scope.
- **The brace-less `if` branch keeps its authored spelling.** `if x then y;` was stored as a
  one-member brace body and re-emitted as `if x  { then y; }` -- braces the author never wrote,
  plus a doubled space. `ActionBranchBody` distinguishes the two spellings the grammar offers, so
  each is emitted as authored.
- **A `ref` body no longer depends on which declaration owns it.** `UsageBody = DefinitionBody`
  (SysML 8.2.2.6.2), so a `ref` body holds the general usage-member set wherever it appears -- but
  five parsers built one, each accepting its own member grammar and wrapping the result in a
  per-owner variant, so `RefBodyElement` recorded which parser had run rather than what the grammar
  allows. There is now one parser. A connection-owned `ref` body gains the usage members it was
  rejecting (`assert constraint { ... }` in `john_individual_example` parses instead of reporting
  `unexpected keyword 'assert' in ref usage body`), and part-, action-, and state-owned `ref`
  bodies can be emitted at all -- their members previously failed emission as unsupported
  constructs. Recovery diagnostics still name the `ref` body scope.
- **Comment and textual-representation members now parse in usage bodies.** The general
  usage-member scope accepted only two of the four annotating alternatives, so `comment /* ... */`
  and `rep x language "..." /* ... */` were rejected inside a `part p { ... }` body even though the
  grammar admits the whole `AnnotatingElement` production wherever a definition body is legal. It
  now uses [`AnnotatingMember`], which brings all four. `#Name` prefix metadata stays a separate
  member, as the grammar has it.
- **An anonymous `comment` member survives format and reparse.** `CommentAnnotation` did not record
  whether the optional `comment` keyword was authored, so emission omitted it whenever the member
  had no name or locale -- and a bare `/* ... */` reparses as trivia, losing the member. The
  keyword's span is now retained and reproduced. The keyword-less spelling
  (`locale "en" /* ... */`) is no longer given a keyword it never had, either.
- **A `rep` member now emits from every scope that accepts one.** Three copies of the same match
  handled annotating members in relationship bodies and disagreed: a textual representation
  emitted from an import body but failed as an unsupported construct from alias, dependency, and
  `connect` bodies, so whether a document could be formatted depended on which construct owned the
  body. One production now means one emitter.

### Removed

- **`Other(String)` is gone from every body scope.** Eleven scopes carried a member that held a
  copy of the source with no span, no structure, and -- deliberately -- no diagnostic. It fired
  along two different paths, and both were backwards: unrecognized text was swallowed silently
  while *recognized* keywords got a diagnostic, and one path decided by sniffing the raw text for
  `:>>` or a leading `ref`/`abstract`/`return`. Content the scope cannot parse is now a recovery
  node with its authored span and a report; a spec-valid member the scope does not model is an
  explicit `Unsupported` node carrying a warning. The two states stay distinct, and each exists
  only in the scopes that can produce it. `capture_opaque_member` and `OpacityKind::Other` are
  removed with the last producer.

- **Two opaque body-member fallbacks the parser cannot produce.** `PartDefBodyElement::Other`
  and `OccurrenceBodyElement::Other` retained unrecognized member text as a string, but no
  parser path constructed either one: recovery in those scopes already produces a malformed or
  unsupported node with its authored span. The variants only widened the type -- and its
  deserialized contract -- with a state that could not occur, so the emitter and opacity report
  carried policy for it, and two integration tests asserted members had not "degraded" into it.
  Removing them makes that guarantee structural. Token-level `BinaryOperator::Other` and
  `UnaryOperator::Other` are deliberately kept: they classify an authored operator spelling and
  back a total `from_token` constructor, which is narrowly scoped opaque syntax rather than an
  untyped scope member.

### Changed

- **`PARSE_AST_VERSION` is now 141.** `Body` carries its delimiters: `Semicolon` holds the `;`
  span, `Brace` holds both brace spans, and a new `Absent` variant covers a declaration that
  never had a body to write. `IfStmt`'s branches become `ActionBranchBody`. `RefBodyElement` is
  removed: `RefBody` is now
  `Body<PartUsageBodyElement>`, the usage-member set the grammar gives it. `CommentAnnotation`
  gains `keyword_span`, and
  `PartUsageBodyElement` replaces its `Doc` and `MetadataAnnotation` variants with `Annotating`.
  Relationship and `ref` body elements replace their `Doc`,
  `Comment`, `TextualRep`, and `MetadataAnnotation` variants with a single `Annotating` variant
  wrapping [`AnnotatingMember`]. `EnumerationBody`'s brace members move from `values` to
  `elements`, matching every other body. That is the only wire change from the body-container
  work: the shared container keeps the variant and field names the other twenty-six bodies
  already used, so their serialized shape is unchanged. Two duplicate container names collapse
  into the type they were always equal to -- the public `RequireConstraintBody` and a
  parser-internal `StructuredConstraintBody`, both `Body<ConstraintDefBodyElement>`, are now
  `ConstraintDefBody`, which also removes two conversion functions that existed only to move
  members between identical types.
- **Whole-tree walks now go through `ast::visit` instead of hand-maintained recursion.** Test
  span normalization (`RootNamespace::normalize_for_test_comparison`), recovery-diagnostic
  collection, deserialization provenance validation, and qualified-reference-identity
  validation were four independent traversals -- roughly 4,700 lines -- that each had to be
  edited when a member could appear somewhere new, and each of which could silently miss a
  scope. They are now four small policies over the shared traversal (under 300 lines in total), with
  identical diagnostics across the snapshot and fixture corpus. Reference-identity validation
  in particular no longer routes through a no-output `serde::Serializer`; it is a typed visit.
- **`RootNamespace::normalize_for_test_comparison` now erases every span in the tree** rather
  than the subset the hand-written copy happened to reach, and it preserves whether an optional
  span was authored at all. Whether a construct recorded a `language` clause or a declaration
  name is grammar and still compares; where it was authored is provenance and does not.

### Fixed

- **Recovered text is no longer dropped when formatting eight recovery forms.** Malformed
  package-body, state-body, and requirement-body members, and root-level recovery, wrapped
  their `ParseErrorNode` in a `Span::dummy()` sentinel while the enclosing member node held the
  real span, so the emitter had no slice to stream and silently omitted the authored text.
  Those nodes now carry their exact authored span, and formatting a recovered document emits
  the malformed slice at its tree position.

- **`PARSE_AST_VERSION` is now 135.** `StateDefBodyElement` gains `AttributeUsage`,
  `ActionUsage`, `SuccessionUsage`, and `AssertConstraint` variants: the Systems Library's
  `States.sysml` state-body members (`attribute :>> isTriggerDuring;`, `action :>> subactions
  :> middle { ... }`, `succession stateSequencing first [0..1] exclusiveStates then [0..1]
  exclusiveStates { ... }`, `assert constraint { ... }`) now dispatch to their existing typed
  productions instead of opaque recovery (spec42 gap 42, state-def half). `action_usage`
  additionally accepts a leading `:>`/`:>>` specialization clause standing in for the name
  (mirroring `attribute_usage`'s prefix heads), which also fixes the same form inside
  part/action bodies. Emitters no longer print a double space in anonymous `action {`/`action
  :>>`/`assert constraint {` spellings; five more release example files roundtrip.
- **`PARSE_AST_VERSION` is now 134.** `EntryAction`/`DoAction`/`ExitAction` gain
  `declared_name`/`type_name`/`redefines` for declaring a *new* nested action (`entry action
  entryAction :>> 'entry';`, `do action doAction : Action :>> 'do';`, Systems Library
  `States.sysml`) and an `effect: Option<TransitionEffect>` for `assign`/`send`/`accept`
  effects written directly under the keyword (`entry assign counter.count := 0;`), mirroring
  `Transition::effect` (spec42 gap 43). The reference form keeps `action_reference`.
- **`PARSE_AST_VERSION` is now 133.** `AttributeBodyElement` gains `Bind`, `Connection`,
  `CalcDef`, `CalcUsage`, and `ConstraintUsage` variants: named/multiplicity-qualified binding
  members, named/typed connection usages, nested calcs, and plain (non-`assert`) constraints
  inside attribute/item-shaped bodies now dispatch to their existing typed productions instead
  of opaque capture (Geometry `ShapeItems.sysml`, `Time.sysml`, `Items.sysml`; spec42 gap
  49a). `constraint_usage` additionally accepts the anonymous body-only form
  (`constraint { expr }`).
- **`PARSE_AST_VERSION` is now 132.** Use-case-family and variant member widening (spec42 gaps
  44/45/46): `VariantTypedUsage` gains a `Requirement` kind (`variant requirement r1;`);
  `UseCaseDefBodyElement` gains `InOutDecl` for the directed parameter shorthand (`in scenario
  = cityScenario;`); `ActorUsage.type_name` becomes optional for the bare untyped actor form
  (`actor environment;`, `actor passenger [0..4];`, OMG spec Annex A). `InOutDecl` additionally
  gains a typed `subsets` clause: the authored `:>` spelling on a directed parameter (`out
  voltage :> ISQ::electricPotential = ...;`) is retained as a subsetting relationship instead
  of being silently folded into `type_name`; `ConstraintDefBodyElement::InOutDecl` is boxed
  (wire-format neutral).
- **`PARSE_AST_VERSION` is now 131.** The `[unit]` annotation now applies to parenthesized
  tuples, invocation/constructor results, and feature references in expression position
  (`(0, shape.width/2, 0)[source]`, the Domain Geometry coordinate-frame idiom; spec42 gap
  49c) via the existing `Expression::LiteralWithUnit` shape. Previously such brackets either
  failed the whole statement (inside argument lists) or were silently captured as a bogus
  declaration multiplicity after the value.
- **`PARSE_AST_VERSION` is now 130.** The bare `:>>`/`:>` shorthand (no `attribute` keyword)
  and its metadata-body twin accept comma-separated multi-target lists like every other
  redefinition/subsets clause (`SI.kerml`'s `kelvin`; spec42 gap 49b), and `ref` declarations
  no longer consume the `redefines`/`subsets`/`references`/`crosses` keyword spelling of a
  relationship clause as their declared name (`Items.kerml`; spec42 gap 49d). Anonymous
  attribute/ref declarations also stop emitting a double space / a fabricated `''` name.
- **`PARSE_AST_VERSION` is now 129.** The canonical anonymous flow shorthand `flow from <a> to
  <b>;` (and its `succession flow` sibling) no longer silently misparses the `from` keyword as
  the flow's declared name: such statements now produce `FlowUsage { name: None, .. }`, so
  cached parses from earlier versions are invalidated even though the serialized shape is
  unchanged (spec42 gap 47). Anonymous flows also emit the `from`-keyword spelling instead of
  the double-spaced endpoint shorthand.
- **`PARSE_AST_VERSION` is now 128.** `ExtendedDefinition` gains `has_def_keyword`: the bare
  `#<keyword>+ <Name> { ... }` extended-usage shorthand (no `def` keyword at all, e.g.
  `#clouddd ArrowheadCore { ... }`) now parses into the same typed node as its `def`-suffixed
  sibling instead of dropping the rest of the package body into unrecovered error-token
  consumption (spec42 gap 39). Keyword-led `#tag <member>` prefix shapes stay on
  `metadata_keyword_prefix`.
- **`PARSE_AST_VERSION` is now 127.** `KermlFeatureMember` and `KermlEndMember` gain an
  `is_const` prefix flag: `const end [1] feature a;` / `const end feature b;` (KerML
  association bodies) now attach the `const` keyword to the end/feature member instead of
  misparsing `const` as a dangling bare feature reference followed by an unrelated end member
  (spec42 gap 36).
- **`PARSE_AST_VERSION` is now 126.** `UseCaseDefBodyElement` gains a
  `Ref(Box<Node<RefDecl>>)` variant and `RefDeclKind` gains `UseCase`, so full `ref use case
  <name> : <Type> :>> <target>;` declarations inside use-case bodies (pervasive in Systems
  Library `UseCases.sysml`) parse into typed `RefDecl` nodes instead of spraying per-token
  error recovery; the bare `ref :>> target { ... }` shorthand keeps its dedicated
  `RefRedefinition` node (spec42 gap 34).
- **`PARSE_AST_VERSION` is now 125.** `SubjectDecl` gains a `redefines:
  Option<Node<SubsettingRelationship>>` clause (`subject subj :>> Case::subj;`, and the
  anonymous type-less `subject :>> vehicle = vehicle_large;`) and its `value` widens from a
  bare `=`-only `Expression` to the shared `FeatureValue` clause, adding the `default`-keyword
  spelling (`subject generateTorque default engine1.generateTorque;`, OMG spec Annex A;
  spec42 gap 35). Subject emission no longer prints a double space for anonymous subjects.
- **`PARSE_AST_VERSION` is now 124.** `RelationshipBodyElement` gains a
  `KermlFeature(Box<Node<KermlFeatureMember>>)` variant: braced `RelationshipBody` forms
  (dependency/alias/relationship-statement bodies) now own feature members (`dependency z to
  x, y { feature e; }`) per the BNF's `ownedRelatedElement`, instead of dropping the whole
  bodied statement to error recovery (spec42 gap 37). Ref bodies keep the annotation-only
  subset.
- **`PARSE_AST_VERSION` is now 123.** `PartDefBodyElement`, `PartUsageBodyElement`, and
  `AttributeBodyElement` gain a `KermlClassifier(Box<Node<KermlClassifierDecl>>)` variant so
  KerML classifier-keyword declarations (`struct`, `classifier`, `datatype`, `assoc`,
  `behavior`, ...) nested inside part/attribute-shaped bodies parse into the same typed node
  they already get at package scope instead of falling to error recovery (spec42 gap 38;
  `class` keeps its dedicated `ClassDef` shape).
- **`PARSE_AST_VERSION` is now 122.** The opaque `ActionBodyDecl` node (an unparsed
  keyword + text blob for `attribute`/`calc`/`event` declarations and nested `action def`s in
  action bodies) is retired: `ActionDefBodyElement`/`ActionUsageBodyElement` lose their `Decl`
  variant and gain typed `AttributeUsage`, `CalcUsage`, and `ActionDef` variants, with `event`
  forms dispatching through the existing typed `OccurrenceUsage` (spec42 gap 33).
  `OpacityKind::ActionBodyDecl` is removed alongside. `OccurrenceUsage` additionally accepts a
  multiplicity authored between the name and the typing (`event occurrence
  zeroCrossingEvents[0..*] : ZeroCrossingEventDef`), per the BNF's declaration ordering.
- **`PARSE_AST_VERSION` is now 121.** `AllocationUsage` and `FlowUsage` gain
  `subsets`/`redefines` (previously parsed by the shared usage header and discarded) and typed
  connector ends: `source`/`target` and `from`/`to` are now
  `Option<Node<KermlConnectorEnd>>` (optional multiplicity + arena-backed feature chain +
  optional `::>`/`references` end-name split) instead of opaque `Expression` nodes, so the
  authored allocate end names (`allocate logical ::> torqueGenerator to physical ::>
  powerTrain`) are retained rather than discarded (spec42 gaps 27/28).
- **`PARSE_AST_VERSION` is now 120.** Typed-field gaps from the spec42 audit:
  `ThenTarget` gains a `Send(Box<Node<ActionUsage>>)` variant (`then send new S() to b;`,
  spec42 gap 30); `KermlFeatureMember` gains a `crosses` cross-subsetting clause (gap 32);
  `ViewpointUsage` gains `subsets`/`redefines` mirroring `ViewUsage` (gap 25); and
  `RequireConstraint` gains an arena-backed `target: Option<QualifiedReferenceId>` for the
  keyword-less reference shorthand `require <qualified.name>;` (`name` now only carries the
  `constraint`-keyword form's declared name; gap 29).
- **`PARSE_AST_VERSION` is now 119.** Package-level KerML declarations are unified onto their
  typed nodes (spec42 gaps 13/14/22/23):
  - New `KermlRelationshipDecl` (`PackageBodyElement::KermlRelationship`) models the KerML
    explicit relationship declarations (BNF §8.2.4): `subtype`/`subclassifier`/`typing`/
    `subset`/`redefinition` with an optional `specialization <ident>` (or doubled-keyword)
    prefix, `disjoining? disjoint a from b`, `inverting? inverse a of b`, and `featuring (I
    of)? a by b`, each with the annotation-only `RelationshipBody`.
  - `KermlClassifierKeyword` gains `Type` (`type UnionType unions A, B;`) and the spelled-out
    `Association`; `subclassifier` moved from the classifier keywords to the relationship
    family (it declares a relationship, not a classifier). `KermlTypeRelationshipKeyword`
    gains `Differences`.
  - Bare forward declarations of classifier keywords (`classifier X;`, `datatype D;`, ...)
    now parse as `KermlClassifierDecl` with a `;` body -- a resolvable named declaration --
    instead of the span-only `KermlBareDeclaration`.
  - Plain `feature`-keyword package members route through `KermlFeatureMember`; the
    superseded `DefaultReferenceUsage`-shaped `feature_usage_member` production and its
    `FeatureBodyElement::Expr`/`ExprMember`/`ExprMemberElement` machinery are removed
    (`expr s { ... }` members parse as feature members of kind `expr`).
  - Keyword-less implicit-feature package members parse (`causeA;`, `y = expr;`,
    `z : Type;`) as `DefaultReferenceUsage`; bare *reserved keywords* (`then;`) still get
    their targeted recovery diagnostic.
  - Expression grammar: KerML dot shorthands `x.{...}` (collect) and `x.?{...}` (select)
    parse as `CollectionOp` with a new authored-spelling `dot_shorthand` flag.
- **`PARSE_AST_VERSION` is now 118.** KerML type-body members reach attribute-shaped bodies
  (the body grammar `class_def`'s KerML `class`/`struct`/`datatype` definitions share with
  SysML attribute/item bodies): `AttributeBodyElement` gains `KermlFeature`, `Invariant`,
  `KermlConnector`, and nested `ClassDef` variants, so `feature x : Natural[1];`, `member
  feature ...`, `composite feature ... subsets ...`, `portion feature all ...`, `var x : T;`,
  `step`/`expr`/`bool` kinds, `inv name { ... }`, `connector a ::> a.x to b;`, and nested
  `class` definitions parse instead of `unrecognized_declaration_in_scope` (spec42 gaps
  15/16/17/18/19/21/24). Package bodies gain a `PackageBodyElement::KermlConnector` member
  (`connector a2 from x.s to y.t;`, spec42 gap 16); connector ends accept the `::>` operator
  spelling and an end-name-led binary shorthand; `InOutDecl` gains `is_var` (`out var y1;`,
  spec42 gap 18). Resolves the `connector all` fixture wholesale.
- **`PARSE_AST_VERSION` is now 117.** `TypingRelationship` gains a `spelling:
  TypingSpelling` field recording whether the author wrote the symbolic operator (`:`/`:>`) or
  the keyword form (`specializes`, `defined by`, `typed by`), and emission renders the authored
  spelling instead of canonicalizing everything to the operator — `function abs specializes
  ComplexFunctions::abs` now round-trips as written. Equality ignores the field (like spans):
  `specializes B` and `:> B` name the same relationship; the spelling is provenance.
- **`PARSE_AST_VERSION` is now 116.** Calc/type bodies no longer swallow unmodeled members as
  diagnostic-silent opaque `Other(...)` captures: `CalcDefBodyElement::Other` is removed, every
  unparseable member becomes an explicit recovery node with a `recovered_calc_body_element`
  diagnostic, and the ~90 library members the swallow was hiding are structurally implemented:
  - New type-body members: `KermlConnectorMember` (`connector :Type from [1] self to [1]
    this;`, named/`all`/`from`-less forms, `references` end chains), `KermlBindingMember`
    (`binding [1] a = [1] b;`, named `of` form, bodies), `KermlSuccessionMember` (`succession
    [1] a then [*] b;`, `all`, named `first` form), `KermlEndMember` (`end name? [mult]?
    subsets? feature ...` cross features), plus `Import`, `Comment`, `AttributeUsage`,
    `AssertConstraint`, nested `KermlClassifier`, and keyword-less `DefaultReferenceUsage`
    binding members (named `private x: T[1] = ...;` and anonymous `:>> x = ... { ... }` with
    nested binding bodies via new `FeatureBodyElement::Binding`/`Doc` variants).
  - `KermlFeatureMember` gains `chains`, type relationship clauses (`unions`/`intersects`/
    `disjoint from`), an optional kind keyword (`portion redefines ... = ...;`), and an
    optional name; `ReturnDecl` gains `attribute`/`feature` kind keywords, an optional type
    (`return result [1..1];`), and merged multi-clause redefinitions;
    `DefaultReferenceUsage` gains a multiplicity.
  - Expression grammar: the KerML null-coalescing `??` operator
    (`BinaryOperator::NullCoalesce`), parenthesis-free collection-operator function references
    (`->reduce RealFunctions::'+'`), and a Range-operator lexing fix -- `1..4` previously
    mis-lexed `1.` as a real literal, so `(1..size(x))` only "parsed" as a bogus member access
    and plain ranges failed outright.
  - `CalcDefBodyElement::ReturnDecl` boxes its node (clippy `large_enum_variant`).
  The full-library scan stays at zero diagnostics with the swallow removed; the two tests that
  asserted the old silent-`Other` behavior now assert the explicit recovery contract.
- **`PARSE_AST_VERSION` is now 115.** `ViewUsage` retains its multiplicity on the named path
  (the shared usage header already parsed it and the named constructor discarded it) and gains
  `ordered`/`nonunique` multiplicity properties, so `view columnView[0..*] ordered { ... }`
  (Systems Library `Views.sysml`) no longer loses `[0..*] ordered` on formatting. Emission
  orders the clauses the way each form's parser reparses them (multiplicity after the target
  for the anonymous `:>>` form, before the trailing subsets clause for the named form).
- **`PARSE_AST_VERSION` is now 114.** Feature values gain a typed standalone KerML
  `BodyExpression` form and authored-operator fidelity: new `Expression::BodyExpr` models
  `{ parameters* result? }` as a primary expression (sharing `CollectionOperatorBody`'s shape),
  so the pin initializers `in whileTest default {true} { ... }` (Systems Library
  `Actions.sysml`) parse as typed values instead of being consumed opaquely and discarded by
  `in_out_decl`; `feature_value_part` accepts `{ ... }` after any value operator. `FeatureValue`
  gains `has_operator`, so the bare `default expr` / `default {expr}` spellings no longer emit
  a fabricated `= ` (`return : Real default sum0(...)` now round-trips byte-for-byte).
- **`PARSE_AST_VERSION` is now 113.** The remaining KerML declaration grammar used by the
  pinned `sysml.library` is structurally implemented; the layered conformance scorecard's L2
  claim now **passes for both the Systems Library and the full library** (94 files, zero
  diagnostics, zero `ExtendedLibraryDecl`/`KermlSemanticDecl`/`KermlFeatureDecl` fallback
  nodes). In detail:
  - `KermlClassifierDecl` covers `datatype`/`metaclass`/`struct`/`assoc`/`assoc struct`/
    `behavior`/`interaction`/`predicate`/`multiplicity`/`subclassifier`/`classifier`/`class`
    (plus `function` from 112), with `all` sufficiency, a post-name multiplicity, `:` typing
    for feature forms, and typed `disjoint from`/`unions`/`intersects` clauses
    (`KermlTypeRelationship`).
  - New `KermlFeatureMember` (calc/type-body and package scope) models `member`/`derived`/
    `abstract`/`composite`/`portion`/`var`/`end` prefixes, the `feature`/`step`/`expr`/`bool`
    kind keywords, `all`, leading or trailing redefinitions, multi-target typing, multiplicity
    with `ordered`/`nonunique`, subsets/redefines/references clauses, `inverse of`, values, and
    nested type bodies. New `KermlInvariantMember` models `inv (not)? name? { ... }` at both
    scopes.
  - `TypedParameterMember` gains `abstract`, the `calc`/`step` kinds, post-redefinition typing
    and multiplicity; `InOutDecl` accepts the anonymous typed form (`in : T[1];`), the
    spelled-out `redefines` operator, and typing/multiplicity trailing a redefinition.
  - `ReturnDecl` gains a `{ ... }` result body (`CalcDefBody`); `RefDecl` retains its kind
    keyword (`ref item scene : Scene;` no longer drops `item` on formatting) and
    `RefBodyElement` gains an `AttributeUsage` variant; part usage bodies dispatch kinded
    `ref item :>> a, b;` members; `DefaultReferenceUsage` accepts the anonymous leading
    `:>> target = expr;` binding.
  - Formatting quotes declared names that spell reserved keywords (`'in'`, `'ref'`,
    `'about'`), which previously re-emitted bare and could not reparse.
  - From 112 (folded in): `ExprMemberElement::ReturnDecl` boxes its node to keep the enum size
    bounded (wire format unchanged).
- **`PARSE_AST_VERSION` is now 112.** KerML `function` declarations parse as a structured
  `PackageBodyElement::KermlClassifier` node (`KermlClassifierDecl`: `abstract` prefix, keyword
  enum, identification, multi-target `specializes`/`:>` clause, calc-style body) instead of the
  opaque `KermlSemanticDecl` fallback, covering the Kernel Function Library. Supporting grammar:
  `ReturnDecl` gains `multiplicity`, `ordered`/`nonunique`, and a full `FeatureValue` value
  clause (`return : Real[1] = x;`, `return : Anything[0..*] ordered nonunique;`, `return : Real
  default …;`); new `CalcDefBodyElement::TypedParameter` models KerML kinded parameters
  (`in expr fn[0..*] { … }`, `in bool test = expr;`, `in feature clock : Clock[1] default
  localClock { … }`) whose bodies follow the calc-body member grammar; and `in_out_decl` rejects
  the `expr`/`bool`/`feature` kind keywords so those forms reach the kinded-parameter arm.
  Full-library diagnostics: 552 -> 250.
- **`PARSE_AST_VERSION` is now 111.** `ItemUsage` gains `is_abstract` (BNF `RefPrefix`,
  accepted by the parser for the first time), `subsets` (`:> objects`, previously parsed by the
  shared usage header and discarded), and `ordered`/`nonunique` multiplicity properties
  (previously skipped). Covers the package-level `abstract item items : Item[0..*] nonunique
  :> objects { ... }` declarations (Systems Library `Items.sysml`/`Metadata.sysml`) that
  previously fell through to the `ExtendedLibraryDecl` fallback. The shared
  `feature_usage_header` now captures its post-typing multiplicity and `ordered`/`nonunique`
  flags on `UsageHeader` instead of discarding them. The L2 Systems Library scorecard layer now
  passes: zero diagnostics and zero fallback nodes across all 21 files.
- **`PARSE_AST_VERSION` is now 110.** `RenderingUsage` retains its full declaration surface
  instead of discarding everything but the name and type: `is_abstract`, `multiplicity`,
  `ordered`/`nonunique`, `:>` subsets, `:>>` redefines, and a `ValuePart` feature value. The
  declaration name is optional (anonymous `rendering :>> subrenderings[0..*] =
  columnView.viewRendering;`, Systems Library `Views.sysml`), and
  `RenderingUsageBodyElement` gains a `Rendering(Box<Node<RenderingUsage>>)` variant so
  rendering usages can nest inside rendering usage bodies (`asElementTable`).
- **`PARSE_AST_VERSION` is now 109.** Connection/interface `ref` declaration bodies are now a
  structured member scope: `RefBodyElement` gains a `Ref(Box<Node<RefDecl>>)` variant for nested
  keyword-less `ref` declarations, and `connector::ref_decl` accepts a `MemberPrefix` visibility
  prefix (captured on `RefDecl::membership`) plus `nonunique`/`ordered` directly after the
  post-typing multiplicity (before further specialization clauses). Covers Systems Library
  `Interfaces.sysml`'s `ref port :>> participant : Port [2..*] nonunique ordered { protected ref
  thisParticipant :>> self; ... }`. Unrecognized members recover as
  `recovered_ref_body_element` ("ref usage body") instead of
  `recovered_relationship_body_element`. `RefDecl` gains `multiplicity`/`ordered`/`nonunique`
  (previously parsed and discarded by `connector::ref_decl`, so `[2..*] nonunique ordered` was
  dropped on formatting), `emit_ref_decl` emits typing before multiplicity before the
  subsetting-family clauses (the one order every `RefDecl` parser reparses), and
  `StateDefBodyElement::Ref` boxes its node to keep the enum size bounded.
- **`PARSE_AST_VERSION` is now 108.** Direction-prefixed parameter declarations (`InOutDecl`)
  now cover the full BNF `FeatureSpecializationPart`/`ValuePart` surface the Systems Library
  uses: the multiplicity clause may precede the typing (`in transitionLinkSource[1]:
  StateAction :>> ...`), `ordered`/`nonunique` multiplicity properties are retained as typed
  flags, a `:>>` redefinition (including comma-separated multi-target form) may trail a named
  declaration, the value clause is a full `FeatureValue` (`= expr` / `:= expr` /
  `default (=|:=)? expr`) instead of a bare `= expr` expression, and a `{ ... }` terminator
  body is retained as typed action-body elements instead of being consumed and discarded.
  `OccurrenceUsage` gains `direction: Option<InOut>` (BNF `RefPrefix`, e.g. `in occurrence
  terminatedOccurrence[1] { ... }`, dispatched in action bodies like directed `in item`/`in
  part`) and `value: Option<Node<FeatureValue>>` (`in occurrence terminatedOccurrence default
  that as Occurrence { ... }`), both from Systems Library `Actions.sysml`. To keep enum sizes
  bounded, `ExprMemberElement::InOutDecl` and `CalcDefBodyElement::InOutDecl` now box their
  node (`Box<Node<InOutDecl>>`; serialized form unchanged).
- **`PARSE_AST_VERSION` is now 107.** `KermlBareDeclaration::keyword` is now an exhaustive
  `KermlBareDeclarationKeyword` enum instead of an owned `String` (a finite grammar set, with
  distinct variants for authored synonyms like `assoc`/`association`), and
  `KermlBareDeclaration`/`BindingConnectorUsage` keep only a `name_span: Option<Span>` for the
  declared name instead of also copying it into an owned `String` -- the name text is resolved
  through the document source when needed, matching the "authored spelling lives in source, not
  per-node strings" contract.
- **`PARSE_AST_VERSION` is now 106.** New `PackageBodyElement::ExtendedDefinition` node models
  SysML §8.2.2.27 `ExtendedDefinition`: one or more `#<name>` metadata-keyword tags standing in
  place of the usual classifier keyword before `def` (`#situation def Failure;`,
  `#SecurityRelated #situation def Vulnerability;`, `abstract #situation def AbstractFailure;`,
  `variation #situation def V;`, with optional `:>` specialization and a `{ ... }` body reusing
  `PackageBody`). Tried before `metadata_keyword_prefix` in package-body dispatch, so `def
  Failure;` no longer falls through to raw error recovery.
- **`PARSE_AST_VERSION` is now 105.** `AttributeBodyElement` gains a structured `ItemUsage`
  variant. A nested `item name : Type;` inside an `attribute def`/`attribute`/`item def`/`item`
  body now parses as a real item usage (reusing the same `item_usage` parser `part def`/`part`
  bodies already dispatch to) instead of being swallowed by the opaque-capture fallback into
  `AttributeBodyElement::Other`.
- **`PARSE_AST_VERSION` is now 104.** Bare, `;`-terminated `classifier` forward declarations
  (e.g. `classifier SpatialFrame;`) parse as a structured `KermlBareDeclaration` node with a real
  `name` field instead of falling through to the opaque `ClassifierDecl` raw-text fallback.
- **`PARSE_AST_VERSION` is now 103.** `ConcernUsage` (including `concern def`) retains its `:>`
  subsetting and `:>>` redefinition clauses (`ConcernUsage::subsets`/`ConcernUsage::redefines`)
  instead of discarding them after parsing, matching sibling usage kinds.
- **`PARSE_AST_VERSION` is now 102.** `ViewUsage` retains its `:>` subsetting clause
  (`ViewUsage::subsets`) instead of discarding it after parsing, matching sibling usage kinds such
  as `OccurrenceUsage`/`StateUsage`/`PortUsage`.
- **`PARSE_AST_VERSION` is now 101.** `individual item`/`individual occurrence`/`individual port`
  short usage forms parse correctly instead of being misclassified or falling into a recovery
  cascade: package-level `item def` now requires the `def` keyword so it no longer shadows
  `individual item x;`; `individual` occurrence usages accept an optional `occurrence` kind
  keyword (`OccurrenceUsage::has_occurrence_keyword` preserves whether it was authored); `port`
  usages accept an `individual` prefix (`PortUsage::is_individual`); and `state def`/`connection
  def` accept `individual` (`StateDef::is_individual`/`ConnectionDef::is_individual`).
- **`PARSE_AST_VERSION` is now 100.** `InterfaceUsage` (all three variants) retains its `:>`/`:>>`
  subsetting and redefinition clauses (`subsets`/`redefines`) instead of discarding them after
  parsing, matching sibling usage kinds such as `ConnectionUsageMember`.
- **`PARSE_AST_VERSION` is now 99.** `AnalysisCaseUsage` and `CaseUsage` retain their `:>`/`:>>`
  subsetting and redefinition clauses (`subsets`/`redefines`) instead of discarding them after
  parsing, matching sibling usage kinds such as `RequirementUsage`/`PortUsage`/`StateUsage`.
- **`PARSE_AST_VERSION` is now 97.** Package bodies gain `@ Name (: Type)? about target(,
  ...)?;` standalone metadata-annotation support, accept stacked `#Prefix #Prefix ... member`
  metadata tags before a member instead of at most one, and port definition bodies dispatch
  `#Prefix`-tagged nested port declarations through the same shared metadata-keyword parsing other
  definition bodies already use.
- **`PARSE_AST_VERSION` is now 96.** `ConstraintUsage` retains its `:>`/`:>>` subsetting and
  redefinition clauses (`ConstraintUsage::subsets`/`redefines`) instead of discarding them after
  parsing, matching sibling usage kinds such as `ConnectionUsageMember`.
- **Canonical document source ranges.** `SourceStorage::position_at`, `SourceStorage::range_of`,
  and `ParsedDocument::range` resolve byte-backed parser spans through one lazily built,
  document-owned newline index. Downstream diagnostics and navigation can retain a `Span` and ask
  the parsed document for its canonical multiline range without rescanning source text.
- **`PARSE_AST_VERSION` is now 87.** In/out parameters retain their `ref` feature prefix and
  typed multiplicity, and occurrence-body exhibit usages retain their arena-backed state path.
  Emission also preserves quoting for compound unit names such as `'N/mm²'`.
- **`PARSE_AST_VERSION` is now 86.** `FirstMergeBody` brace forms retain an aggregate source span,
  exact opening- and closing-brace spans, and ordered typed action-body members. Valid output pins
  and other recognized members remain semantic syntax; unsupported and malformed members are
  explicit nodes that preserve diagnostics and recovery continuation. Formatting `first`, `merge`,
  `decide`, `join`, and `fork` bodies consumes those typed members instead of fabricating `{}` or
  treating a source slice as successful syntax.
- **For-loop ranges are always typed expressions.** The parser no longer publishes
  `Expression::Opaque(String)` when range parsing fails. Malformed ranges recover as explicit error
  nodes, roll back speculative qualified-reference identities, and preserve later siblings.
- **`PARSE_AST_VERSION` is now 84.** Return-reference bodies now contain typed documentation,
  result-expression, and recovery elements; calculation/constraint returns and return references
  roll back speculative arena identities on failure; occurrence portions use the
  `OccurrencePortionKind` enum. Connection-like part members that are not yet implemented are
  explicit `UnsupportedGrammarNode`s rather than header-scanned `OpaqueMemberDecl`s. Package-body
  diagnostic collection is exhaustive and reports every retained KerML/library fallback instead
  of silently ignoring new or opaque variants.
- **`PARSE_AST_VERSION` is now 83.** State action targets, requirement redefinition contents,
  collection-operator bodies, and nested use-case assertion members are structured typed syntax
  rather than opaque or copied strings.
- **Collection operator brace bodies are structured syntax.** `Expression::CollectionOp` now
  retains a typed `CollectionOperatorBody` instead of an opaque copied `String`: ordered
  `in`/`out`/`inout` parameters, optional `ref` and typing syntax, the semantic result expression,
  source-backed reference identities, and exact body/declaration token spans remain available to
  emitters, diagnostics, navigation, serialization, and semantic snapshots. Malformed bodies fail
  transactionally into the enclosing recovery node without leaking speculative reference IDs.
- **Grammar conformance is now machine-readable and scope checked.** The public
  `SUPPORTED_GRAMMAR` constant exposes the release tag, repository, and deterministic content hash
  derived from the single `docs/conformance-target` pin; builds with the BNF checkout present reject
  mismatched grammar bytes. Package-body recovery starters now come from one typed production table
  whose spec entries are linted against a nullable-aware `FIRST(PackageBodyElement)` derivation.
  Spec-valid package forms that reach an unimplemented dispatch branch produce typed
  `UnsupportedGrammarForm` nodes instead of blaming authored input with malformed nodes.
  `PARSE_AST_VERSION` is now 82.
- **Snapshot regeneration uses available CPU cores.** Both the default `cargo test` snapshot
  contract and the `snapshot_tool` CLI evaluate independent fixtures concurrently, then restore
  deterministic path order before comparison, reporting, or updates.
- **Semantic references are now typed, source-backed, and document-local (#119).** `parse()` and
  editor parsing return a `ParsedDocument` that atomically owns the BOM-normalized source,
  `QualifiedReferenceArena`, and root AST. Imports, exposes, expressions, requirement references,
  type references, and specialization relationships store opaque `QualifiedReferenceId` values;
  consumers resolve them through `ParsedDocument::qualified_reference()` to borrow exact authored
  text, aggregate/segment spans, absolute-scope metadata, and typed `::`/`.` separators without
  splitting or reparsing display strings. Import/expose wildcard, recursive, and filter forms use
  the typed `ImportShape` representation, including distinct `::*`, `::**`, and `::*::**` shapes;
  their aggregate suffix, exact `::`/`*`/`**` token, combined-recursive, and filter-delimiter spans
  retain precise authored provenance without downstream source scanning. Qualified package,
  library-package, and namespace declaration names use a distinct `QualifiedDeclarationName`
  role wrapper over the same packed storage, preserving their scope, segments, separators, and
  spans without misclassifying simple declaration labels as references. State `entry`, `do`, and
  `exit` action targets are likewise arena-backed references, including qualified, dotted, quoted,
  and absolute paths; declaration labels remain a separate identity role. Actor redefinition
  assignment values are spanned `Expression` nodes, and reference redefinition bodies are nested,
  spanned `UseCaseDefBody` trees, replacing both former opaque `String` fields.
  The former `RelationshipTarget` and `FeatureChain` representations and legacy string/display
  accessors were removed rather than retained as compatibility layers. Serde now operates on the
  atomic parsed-document envelope and validates arena ranges and every AST identity when reading
  or writing. `PARSE_AST_VERSION` is now 80 for this breaking schema/API migration.

### Fixed

- **`AttributeUsage` emit duplicated the name for `::>`/`:>`/`:>>`-name-standing-in-prefix
  forms (#113).** `emit_attribute_usage` unconditionally emitted `usage.name` *and* the
  subsets/redefines/references clause for the three anonymous "target stands in for name"
  forms (`attribute :>> target;`, `attribute ::> target;`, `attribute :> target;`,
  `AttributeUsageHead::PrefixRedefines`/`PrefixReferences`/`PrefixSubsets` in
  `src/parser/attribute.rs`) -- re-emitting `attribute :>> differencesOf[1];` as `attribute
  differencesOf[1] :>> differencesOf;`, a structurally different, self-referential construct.
  Fixed by gating the name (and any trailing typing/multiplicity/`ordered`/`nonunique`, which
  parse *after* the target reference for these forms, not after a name) on `name_span.is_some()`
  -- `name_span` is already `None` only for these three derived-name forms, so no new AST field
  was needed. Mirrors `emit_part_usage`'s `redefines_only` handling, keyed off "derived" rather
  than "empty" since `AttributeUsage`'s convention (unlike `PartUsage`/`ItemUsage`) is to derive
  a display name from the target rather than leave it empty. Confirmed against real usage in
  `Simple Tests/CalculationTest.sysml` (`::>`) and `Geometry Examples/
  CarWithShapeAndCSG.sysml` (`:>`); also fixes the pre-existing `:>>` form, caught via the
  `examples/` roundtrip scan (#83) regressing on `Mass Roll-up Example/
  MassConstraintExample.sysml`'s `attribute :>> m : MassValue;` while developing the fix (typing
  trailing the redefines target, not the name, needed the same reordering). New regression tests
  in `tests/gh113_attribute_prefix_name_emit.rs`. Promotes `Simple Tests/CalculationTest.sysml`
  into `EXAMPLES_ROUNDTRIP_PASS`. No `PARSE_AST_VERSION` bump -- emit-only fix, no AST shape
  change.
- **Action-body control-flow gaps (#86).**
  - `in_out_decl`'s `:>>` redefinition branch (`src/parser/action.rs`) accepts an optional
    trailing `: Type` clause between the redefinition target and the `= value`
    (`out attribute :>> a_out : AccelerationValue = Acceleration(dt, tm, tp);`).
  - The literal `metadata` keyword form of `MetadataUsage` (BNF `('@' | 'metadata')`) is now
    dispatched inside action bodies -- `crate::parser::metadata::metadata_usage` already fully
    implemented it, it just wasn't reachable outside package-body scope (new
    `ActionDefBodyElement::MetadataUsage` / `ActionUsageBodyElement::MetadataUsage` variants).
  - `textual_representation` (`src/parser/requirement.rs`) no longer treats `rep` as a mandatory
    prefix -- the BNF makes `('rep' Identification)?` fully optional, so a bare
    `language "alf" /* ... */` now parses; it's also now dispatched inside action bodies (new
    `ActionDefBodyElement::TextualRep` / `ActionUsageBodyElement::TextualRep` variants).
  - `then_action`'s target list (`src/parser/action.rs`) accepts bare `fork`/`accept`/`decide`
    control-node references (`then accept S;`, `then fork F { in a; out b1; out b2; }`,
    `then decide D;`) -- new `ThenTarget::Fork`/`Accept`/`Decide` variants reusing
    `fork_stmt`/`transition_accept`/`decision_stmt`, which already fully parsed these standalone.
  - `if_stmt` accepts the non-brace `then`/`else` succession shorthand (`if x == 1 then A1;`,
    `if x > 1 then A2; else A3;`, a guarded succession per BNF `GuardExpressionMember` +
    `TransitionSuccessionMember`) and `else if ...` chaining (BNF `IfNode`'s
    `IfNodeParameterMember` else-alternative). Both wrap into the same AST shape the equivalent
    braced spelling already produces, so no new `IfStmt` fields were needed.
  - `send`'s standalone-statement payload (`control_node_payload_stmt` in `src/parser/payload.rs`,
    and the `action <name> send ...` inline-suffix form in `action_usage`) now accepts a general
    expression (`send new Publish(someTopic, somePublication) via publicationPort;`), not just
    `name : Type` -- BNF `SendNode`'s payload is `NodeParameterMember` (`FeatureBinding` =
    `OwnedExpression`), unlike `accept`'s typed-name-only `PayloadParameter`. Both `accept`/`send`
    now accept an optional `via <expr>` clause; `send` additionally accepts a trailing
    `to <expr>` clause, including with an empty payload (`send via this to aa.target;`) -- new
    `SendPayload` enum (`ActionUsage.send: Option<SendPayload>`, was `Option<PayloadClause>`) and
    new `ActionUsage.via`/`.to: Option<Node<Expression>>` fields. Also fixes a related
    correctness bug (not just a missing-feature gap): `action <name> send ...` previously fused
    incorrectly -- the name and the send payload landed as two disconnected sibling elements
    instead of one named send node, since the inline suffix only recognized `accept`.
  - `PARSE_AST_VERSION` bumped 71 -> 72 for the `ActionUsage`/`ThenTarget`/`ActionDefBodyElement`/
    `ActionUsageBodyElement` shape changes above.
- **Keyword-less minimal feature-declaration shorthand gaps (#87).**
  - A fully bare `name;` (no type, no value) is now accepted as a `DefaultReferenceUsage` inside
    `part def` bodies (`bare_or_valued_feature_binding`, a new value-optional sibling of the
    existing `feature_value_binding`) -- e.g. `part def V { m; }`. Action bodies keep the
    value-mandatory `feature_value_binding` so the existing targeted bare-identifier recovery
    diagnostic there still fires.
  - The keyword-less `name = expr;` binding shorthand is now dispatched at package-body scope too
    (new `PackageBodyElement::DefaultReferenceUsage` variant), previously only reachable inside
    part/attribute/action bodies -- e.g. `pressure = force / length^2;`. Value-mandatory here for
    the same bare-identifier-diagnostic reason as above.
  - The shorthand now accepts a leading `:>`/`:>>` specialization clause before the value (new
    `DefaultReferenceUsage.subsets`/`.redefines` fields) -- e.g. `torquePerCurrent :>
    Quantities::scalarQuantities = ISQ::torque / ISQ::electricCurrent;`, and the value-less form
    `inflationPressure :> pressure;` where the value is inherited from what it subsets.
  - `item x;` (bare, untyped item usage) is now dispatched inside occurrence definition/usage
    bodies (new `OccurrenceBodyElement::ItemUsage` variant) -- `item_usage` itself already fully
    supported the bare form, it just wasn't reachable there (`part_usage` already was).
  - `PARSE_AST_VERSION` bumped 72 -> 73 for the `DefaultReferenceUsage`/`PackageBodyElement`/
    `OccurrenceBodyElement` shape changes above.
  - Promoted `Simple Tests/AnalysisTest.sysml` into `EXAMPLES_ROUNDTRIP_PASS`.

- **Usage-kind body-member dispatch gaps (#89).** A grab-bag of usage-kind body members that were
  each supported *somewhere* in the grammar but not dispatched in the specific body context real
  examples use them in.
  - Bare `part <name>;` is now dispatched inside `connection def` bodies (new
    `ConnectionDefBodyElement::PartUsage` variant) -- `Simple Tests/ConnectionTest.sysml:31`.
  - `perform action <name>[mult] :> <target>;` -- `perform_action_decl` now accepts an optional
    multiplicity after the name and a `:>` subsets clause (mutually exclusive with `:>>`
    redefines), not just `:>>`/`:`/`=` (new `Perform.multiplicity`/`.subsets` fields) --
    `Camera Example/Camera.sysml:4`.
  - `alias <name> for <target>;` is now dispatched inside `part def`/`part` usage bodies (new
    `PartDefBodyElement::AliasDef`/`PartUsageBodyElement::AliasDef` variants), previously only
    reachable at package scope -- `Simple Tests/AliasTest.sysml:7,16`.
  - `include <usecase>;` and `use case <name> : Type { ... }` are now dispatched inside part usage
    bodies (new `PartUsageBodyElement::IncludeUseCase`/`::UseCaseUsage` variants) --
    `Simple Tests/UseCaseTest.sysml:33-35`.
  - Named `assert <name> { ... }` (referencing a previously-declared `constraint` by name and
    rebinding its `in` parameters) no longer requires the `constraint` keyword
    (`assert_constraint_member`'s `constraint` tag is now optional), and is now also dispatched at
    package-body scope (new `PackageBodyElement::AssertConstraint` variant) --
    `Simple Tests/ConstraintTest.sysml:78`.
  - `verification <name> : Type { ... }` is now dispatched inside a plain part-usage body, not
    just nested case/requirement contexts (new `PartUsageBodyElement::VerificationCaseUsage`
    variant) -- `Simple Tests/VerificationTest.sysml:35`.
  - `variant <name> { ... }` / `variant '<name>' { ... }` -- the untyped `variant` reference form
    now accepts an optional nested body (new `VariantUsage.body` field), not just a bare `;` --
    `Simple Tests/VariabilityTest.sysml:16`, `Variability Examples/VehicleVariabilityModel.sysml:78`.
    The untyped, bare-`;` reference form is also now dispatched inside action usage bodies (new
    `ActionUsageBodyElement::VariantUsage` variant) -- `VehicleVariabilityModel.sysml:128-134`.
  - `render rendering <name> : Type[mult];` inside `view def` bodies -- `view_rendering_usage` now
    accepts an optional `rendering` keyword before the name (BNF `ViewRenderingUsage`'s second
    alternative), which was previously consumed as the usage's own name -- `Simple Tests/
    ViewTest.sysml:32`.
  - Directed (`in`/`out`) `item` usage is now dispatched inside `part def` bodies (new
    `directed_item_usage` dispatch arm, mirroring the existing action-body one) --
    `Timeslice and Snapshot Examples/TimeVaryingAttribute.sysml:14`.
  - `PARSE_AST_VERSION` bumped 73 -> 74 for the AST shape changes above.
  - Promoted `Camera Example/Camera.sysml`, `Mass Roll-up Example/MassConstraintExample.sysml`, and
    `Simple Tests/AliasTest.sysml` into `EXAMPLES_ROUNDTRIP_PASS`.

- **Attribute/reference usage modifier gaps (#88).**
  - `attribute_usage` (`src/parser/attribute.rs`) now accepts `::>` (reference-subsetting) as a
    name-standing-in prefix, same pattern as the existing `:>>` (redefines) handling (new
    `AttributeUsageHead::PrefixReferences`) -- `attribute ::> m = ms.totalMass;` (`Simple Tests/
    CalculationTest.sysml:14`).
  - `attribute_usage`'s `RefPrefix` handling now accepts the full BNF-legal modifier stack:
    `derived`? (`abstract`|`variation`)? `constant`? `ref`? (previously only `derived`/`constant`
    were recognized; `abstract`/`variation`/`ref` were incorrectly assumed illegal on an
    attribute usage) -- new `AttributeUsage.usage_prefix`/`.is_reference` fields. `derived
    constant ref attribute y :> x;` (`Simple Tests/PartTest.sysml:9`), `abstract attribute
    minMass :> ISQ::mass;` (`Mass Roll-up Example/MassRollup.sysml:21`).
  - `part_ref_usage` (`src/parser/part/usage.rs`) now accepts a leading `in`/`out`/`inout`
    direction prefix (new `RefDecl.direction` field) -- the comma-separated multi-target type
    list already worked via `optional_typings`. `private in ref y: A, B;` (`Simple Tests/
    ItemTest.sysml:15`).
  - `attribute_usage` now accepts a bare `:>` (subsets, no name) as a name-standing-in prefix,
    same pattern as `::>`/`:>>` above (new `AttributeUsageHead::PrefixSubsets`) -- `attribute :>
    differencesOf[1] { ... }` (`Geometry Examples/CarWithShapeAndCSG.sysml:84`, also
    `SimpleQuadcopter.sysml`).
  - `PARSE_AST_VERSION` bumped 76 -> 77 for the AST shape changes above
    (stacks on #91's 75 -> 76 bump).

- **Standalone `locale` package member and quoted `calc` usage name/type (#91).**
  - `doc_comment`/`comment_annotation` (`src/parser/requirement.rs`) now peek for a leading
    `locale` keyword before attempting `identification` -- previously `identification` greedily
    consumed the bare word `locale` itself as the doc/comment's own name whenever no real
    identification was present, leaving nothing for the subsequent `locale` keyword check to
    match. Fixes `doc locale "en_US" /* ... */` with no identification (`Simple Tests/
    CommentTest.sysml:32`).
  - New `bare_locale_comment` parser (reusing `CommentAnnotation`, since KerML `Comment`'s
    `('comment' Identification?)?` prefix is entirely optional) dispatches a standalone `locale
    "en_US" /* ... */` package member with no `comment` keyword at all -- previously not
    dispatched anywhere. `Simple Tests/CommentTest.sysml:25`.
  - `calc_usage` is now dispatched inside `part_usage_body_element` (new
    `PartUsageBodyElement::CalcUsage` variant) -- previously only `calc_def_required` (`CalcDef`)
    was, so a calc *usage* nested in a part usage body had no dispatch path at all.
    `calc_usage` itself already fully supported quoted names. `Analysis Examples/Turbojet Stage
    Analysis.sysml:88`.
  - `PARSE_AST_VERSION` bumped 75 -> 76 for the `PartUsageBodyElement::CalcUsage` addition
    (stacks on #90's 74 -> 75 bump).
  - Promoted `Simple Tests/CommentTest.sysml` into `EXAMPLES_ROUNDTRIP_PASS`.

- **`individual` prefix and `timeslice`/`snapshot` usage gaps (#90).**
  - `DefinitionPrefixOptions`/`parse_definition_prefix` (`src/parser/definition_prefix.rs`) gained
    a shared, opt-in `individual` prefix (BNF `OccurrenceDefinitionPrefix`'s
    `(isIndividual ?= 'individual')?`, following `abstract`), wired into `occurrence_def`,
    `item_def`/`item_def_required`, `action_def`, and `analysis_case_def` (new `is_individual`
    fields on `OccurrenceDef`/`ItemDef`/`ActionDef`/`AnalysisCaseDef`) -- e.g. `individual
    analysis def FuelEconomyAnalysis_1 :> FuelEconomyAnalysis;` / `individual action def
    FuelConsumption_1 :> FuelConsumption;` (`Individuals Examples/
    AnalysisIndividualExample.sysml:76-77`), `individual occurrence def IO2 { ... }`
    (`Simple Tests/IndividualTest.sysml:3`), `individual item def John :> Person { ... }`
    (`Individuals Examples/JohnIndividualExample.sysml:19`).
  - The same `individual` prefix (BNF `OccurrenceUsagePrefix`) is now also accepted on
    `action_usage`, `item_usage`, and `analysis_case_usage` (new `is_individual` fields), and
    `ATTRIBUTE_OPAQUE_STARTERS` gained `individual` so `individual item ii : II1;` /
    `individual item :>> i : II2;` opaquely capture the same way the un-prefixed `item` starter
    already does -- adjacent gaps in the same real fixture (`Simple Tests/IndividualTest.sysml`)
    exposed once the def-level cascade above was cleared, plus `individual analysis
    fuelEconomyAnalysis_1 : FuelEconomyAnalysis_1 { ... }` (`Individuals Examples/
    AnalysisIndividualExample.sysml:79`).
  - `timeslice_usage`/`snapshot_usage` are now dispatched inside `attribute_body_element` (shared
    by `item def`/`item` usage bodies) -- both already fully parsed (used elsewhere, e.g. part def
    bodies), just weren't reachable here -- e.g. `timeslice asPresident : Person [0..*] { ... }`
    (`Individuals Examples/JohnIndividualExample.sysml:11`).
  - `PARSE_AST_VERSION` bumped 74 -> 75 for the `is_individual` field additions above.

- **Literal `redefines` keyword edge cases and unnamed typed succession statement (#92).**
  - `attribute_feature_binding`/`redefinition_feature_binding` (`src/parser/attribute.rs`) now
    accept the literal `redefines` keyword, not just the symbolic `:>>` operator (both are
    documented synonyms via `redefine_operator`, already used elsewhere e.g. `part_usage`) -- a
    bare `redefines <target> = <value>;` standalone body member with no `attribute`/`part`
    keyword at all now parses. `Mass Roll-up Example/Vehicles.sysml:26`.
  - `part_usage_redefines_only` (`src/parser/part/usage.rs`) now accepts an explicit `: Type`
    clause after the redefines target (new typing-clause parsing; when present, the display name
    is now derived from the target, matching the pre-existing `ref part :>> elements:
    SparePart;` convention this shape previously reached via a different fallback path) --
    previously only the type-less bare/braced-body forms were accepted. `v1 Spec Examples/
    8.4.5 Constraining Decomposition/Vehicle Decomposition - Updated.sysml:43`.
  - `succession_usage` (`src/parser/occurrence_body.rs`) now recognizes a bare `:` (type clause,
    no name) as the name-less case, and now parses an optional `: Type` clause on the succession
    itself (new `SuccessionUsage.type_name` field, mirroring `FirstStmt::succession_type`'s
    identical field for the sibling action-body form) -- previously a leading `:` fell through to
    the name parser and failed outright, and no type-clause parsing existed at all. Also newly
    dispatched inside `part_usage_body_element` (new `PartUsageBodyElement::SuccessionUsage`
    variant) -- previously only `ConnectionDefBodyElement`/`OccurrenceBodyElement` had it, so a
    succession usage nested in a part usage body had no dispatch path regardless of naming.
    `Vehicle Example/VehicleIndividuals.sysml:49`.
  - `PARSE_AST_VERSION` bumped 77 -> 78 for the `SuccessionUsage.type_name` field and
    `PartUsageBodyElement::SuccessionUsage` variant (stacks on #88's 76 -> 77 bump).
  - Promoted `Mass Roll-up Example/Vehicles.sysml` and `Vehicle Example/VehicleIndividuals.sysml`
    into `EXAMPLES_ROUNDTRIP_PASS`.

## [0.54.0] - 2026-08-07

### Fixed

- **Connector-end / interface / flow shorthand gaps (#85).**
  - `end_decl` (`src/parser/connector.rs`, shared by `connection.rs`/`interface.rs`): accepts
    `item` as an end's leading kind keyword (`end [0..1] item cart: ShoppingCart[1];`); accepts a
    `#tag` metadata-prefix annotation before an end (`end #cause cause1 : Causer1;`, distinct from
    the existing `#name`-as-derived-name form); accepts a trailing `crosses <target>;` clause
    after the type (new `EndDecl.crosses` field); accepts a trailing `::>`/`references` clause
    *in addition to* (not just instead of) an explicit `: Type` (`end port p3: P ::> p.p1;`).
  - `interface_def_body_element` (`src/parser/interface.rs`): accepts bare `flow <a> to <b>;`
    shorthand connecting two of the interface's own ends (new `InterfaceDefBodyElement::FlowUsage`
    variant).
  - `interface_usage` (`src/parser/part/usage.rs`): accepts a named-but-untyped `connect` form,
    `interface name connect a to b { ... }` (new `InterfaceUsage::TypedConnect.name` field); its
    typed, non-`connect` declaration body now accepts `end` members, parallel to the
    already-supported `connection name: Type { end ...; }` form (new
    `InterfaceUsageBodyElement::EndDecl` variant).
  - Promoted `Simple Tests/ConjugationTest.sysml` and `Vehicle Example/VehicleDefinitions.sysml`
    into `EXAMPLES_ROUNDTRIP_PASS` (#83). Several other files targeted by #85 (Association
    Examples, Cause and Effect Examples, Flashlight Example, Requirements Examples, the other
    Vehicle Example file) now parse past their originally-reported failure into a *different*,
    previously-hidden gap further into the same file -- tracked separately, not blocking here.

### Added

- **`examples/` robustness tracker (#83, part 1).**
  - Added `EXAMPLES_ROUNDTRIP_PASS` / `examples_roundtrip_scan` in `tests/roundtrip_validation.rs`,
    running the same parse → opacity-clean → emit → reparse → AST-eq pipeline as the pinned
    `ROUNDTRIP_PASS` gate against the release's much wider, uncurated `sysml/src/examples/` tree
    (95 files) instead. Unlike the conformance gate, this doesn't require 100% — it only fails on
    a regression or an unpromoted pass, tracking general parser robustness against real-world
    SysML v2 source rather than just the curated validation corpus.
  - Baseline: 22/95 roundtrip clean. Documented in `docs/CONFORMANCE.md`'s new "Robustness
    tracker" section.

### Fixed

- **`parse()` / `parse_for_editor()` equivalence on clean input (#70).**
  - Documented the invariant (crate docs in `src/lib.rs`, `docs/CONFORMANCE.md`'s new "Entry
    points" section): on input where `parse()` succeeds, `parse_for_editor()` must report zero
    diagnostics and build the identical AST.
  - Added `tests/validation/parse_entry_point_equivalence.rs` (handwritten cases plus `1a`/`2a`
    validation fixtures) so a future change to either entry point's root-level loop can't
    silently regress this without failing a test.
  - While writing that regression coverage, found and fixed a real gap it was meant to guard
    against: a bare (no `private`/`abstract` prefix) nested `calc def` inside a `calc` body was
    never dispatched to `CalcDef` — `starts_with_keyword(.., b"calc")` alone can't distinguish it
    from the `def`-less `calc` usage form, so it silently fell through as an unrecovered `calc`
    usage parse failure.

  - Promoted remaining analysis fixtures into `ROUNDTRIP_PASS` (`10b`/`10c`/`10d`): directed
    `in calc`/`in requirement`/`in part`/`in attribute`, nested calc rollups, `return :>>` /
    `return part|attribute` (incl. `:>`/`:=`), analysis `for` loops, and keyword-less `:>>`
    bindings inside `require name { … }`.
  - L2.5 validation inventory is now **56 required-pass / 0 known-gap**.

- **L2.5 remaining AST-eq gaps (#78).**
  - Extended span-ignoring `PartialEq` to attribute/part/port/ref/metadata/import/state
    `Then`/`Final` (and related) AST types — clears phantom AST-eq where Debug matched after
    span strip.
  - `dependency from A to B` no longer stores `"from"` as a client (validation `12a`).
  - Bare `allocate src to dst` emits as shorthand (not `allocation allocate …`) — `12b`.
  - `ref` / `ref part` in attribute & item bodies; actor multiplicity; `occurrence def` emit;
    use-case `actor :>>` / `ref :>>` emit; objective body emit (no `requirement objective`).
  - Requirement/concern body: `subject;` / `subject = expr` / braced subject, `stakeholder :>>`,
    keyword-less `:>>` bindings, `variant`, `requirement references`, `require name` (with/without
    `constraint`), `in :>>` params, verify `:>>`.
  - Expressions: `if ? else` Conditional, `all Name` extent, `->forAll {…}` brace CollectionOp.
  - Package/part emit: verification/case/viewpoint/rendering, `#` metadata prefix, nested analysis.
  - Promoted ~33 fixtures into `ROUNDTRIP_PASS` (span phantoms + structural follow-ups).

- **L2.5 ExtendedLibraryDecl / KermlFeatureDecl opacity (#73).**
  - `requirement <'1.1'> name : Type { … }` parses as `RequirementUsage` (short name), not
    `ExtendedLibraryDecl` (validation `08`, `09`).
  - `allocation … allocate end ::> src to end ::> dst` parses reference ends (validation `12b-1`).
  - `abstract occurrence name : Type[*] nonunique;` no longer falls to `KermlFeatureDecl` (`14c`).
  - Named `assume|require constraint name { … }` keeps the name and assume/require keyword.
  - `allocation def` emit + structured `end` members in occurrence/definition bodies; dotted
    `perform a.b :>> …` stays bare (no `action` keyword) so part bodies reparse.
  - `EndDecl` PartialEq ignores name/type spans (same convention as `Membership`).
  - `end feature` / `end occurrence` kinds accepted (Systems/Kernel library Flows & Transfers).
  - Promoted `12b-Allocation-1` into `ROUNDTRIP_PASS`.
  - Remaining on `08`/`09`/`14c`: nested `Other` / verification-case emit — #72-class leftovers.

- **L2.5 Other opacity in state and attribute bodies (#72).**
  - State `entry`/`do`/`exit` keep a referenced action name (`do 'sense temperature' { out temp; }`)
    and record whether the `action` keyword was written.
  - State bodies parse `in`/`out`/`inout` parameters and shorthand `accept … then …` transitions
    (no `transition` keyword).
  - Attribute / item bodies parse `assert constraint` (clears `15_01` / `15_08` Other recovery).
  - Transition emit always writes `first` when a source is present; member-access emit quotes
    spaced names.
  - Promoted `5-State-based Behavior-1a` and `15_01-Constants` into `ROUNDTRIP_PASS`.
  - Remaining after #78: other `Other` / emit-opaque fixtures still under #72.

- **L2.5 AST-eq / emit-shape gaps for function-based behavior (#74).**
  - Redefines-only part usages emit multiplicity after `:>> target` (`part :>> part3[0..1];`).
  - `perform action` is preferred for non-dotted performs so part-usage bodies reparse.
  - `in item`/`in part` are no longer swallowed by action-body `in_out_decl` fallback.
  - Standalone `accept`/`send` control nodes emit without a duplicated `action accept accept`.
  - Unnamed flows use shorthand `flow <from> to <to>` (avoid reparsing `from` as a name).
  - Feature refs / chains quote name segments; `PayloadClause` / `ActionUsage` PartialEq ignore
    source spans (same convention as `Membership` / `Node`).
  - Promoted all `3a-Function-based Behavior-{1,2,3}` and `3c-{1,3}` into `ROUNDTRIP_PASS`.
  - Remaining intentional gaps: `3c-2`/`7a1` still AST-eq for other reasons; `3e` still hits
    opaque `Other` deeper in the fixture despite the perform-action smoke.

- **Import emit drops quotes on package names that need them (#71).**
  `emit_import` wrote `Import.target` raw, so spaced / digit-leading names like
  `'2a-Parts Interconnection'::*` reprinted as bare tokens and failed reparse. Targets are
  now emitted via `format_qualified_name`, which quotes each `::` segment when required while
  leaving `*` / `**` / `$` alone. The same quoting is applied to typing / subsetting / alias
  relationship targets and string `type_name` fields. Adjacent emit fixes needed for the listed
  fixtures to reparse: omit redundant `occurrence` after `individual`/`snapshot`/`timeslice`,
  emit occurrence-body state usages as `exhibit` (§6 G30), and quote `.`-separated feature paths
  per segment. Incidental full-roundtrip promotions: `4a`, `13b`.

### Added

- **L2.5 emit-fidelity roundtrip gate expanded beyond `01-Parts Tree/` (#68).**
  Known-gap scan now covers the full pinned `sysml/src/validation/` tree. Emitter support for
  `port`/`interface`/`connect`, actions/states/perform, requirements/use cases, constraints/calcs,
  flows, views/metadata, dependencies, and related validation constructs unblocks
  `02-Parts Interconnection` (`2a`/`2c` required-pass) and surfaces remaining gaps as parser
  opacity / AST-eq / reparse issues rather than missing emit arms. Incidental promotions:
  `14b`, `15_02`, `15_03`, `15_06`, `15_07`. See `docs/CONFORMANCE.md` and
  `tests/validation/README.md` for the per-folder inventory.

### Fixed

- **Several relationship/`ref` bodies discarded their entire content, with no diagnostic.**
  `alias`/`import`/`dependency` bodies, plain `connect` statement bodies, and connection/
  interface/part-usage/state `ref` bodies all parsed via `advance_to_closing_brace`
  (`src/parser/body.rs`), which skips to the matching `}` and returns `()` — content between the
  braces was entirely discarded, not even captured as opaque text. A `doc`/`comment`/`@metadata`
  annotation (or, for `ref` bodies, real nested members) inside any of these would silently
  vanish with zero indication anything was dropped — worse than the `Other(preview)`/`Error`
  fallbacks used elsewhere in this parser for exactly this reason.
  - New `RelationshipBodyElement` (BNF `RelationshipBody : Relationship = ';' | '{'
    (ownedRelationship += OwnedAnnotation)* '}'`, used by `AliasMember`/`Import`/`Dependency`)
    and shared `relationship_body_annotations` helper (`src/parser/body.rs`): doc/comment/rep/
    metadata retained, anything else recovers to `Error` instead of vanishing. Wired into
    `alias.rs` (`AliasBody::Brace` gained a real `elements` field), `import.rs` (`Import` gained
    `body_elements`), `dependency.rs` (`Dependency` gained `body_elements`), and `connector.rs`'s
    plain `connect` statement (`ConnectStmt` gained `body_elements`) — the last two follow the
    `ConnectBody` + separate sibling-field pattern `Satisfy` already established, since
    `ConnectBody` itself is shared as a bare marker across several differently-shaped contexts
    (`connect_body`, unchanged, is still used marker-only by ~7 unrelated callers with no
    evidence of real content).
  - `RefBody::Brace`'s `elements` field changed from a hardcoded `Vec<Node<ActionDefBodyElement>>`
    (populated only for the action-context `ref` body; every other context produced an empty
    `vec![]` despite already computing real elements internally, via `consume_part_usage_
    structured_brace`/`consume_state_structured_brace`, and then discarding them due to the type
    mismatch) to a new `RefBodyElement` wrapper enum with per-context variants (`Action`,
    `PartUsage`, `State`, plus the same annotation set as `RelationshipBodyElement`). BNF
    `ReferenceUsage` resolves `ref`'s body to a generic `Usage` body, so real content follows
    whatever the owning context allows; connection/interface `ref` bodies don't have a dedicated
    member grammar yet, so they get the annotation-only baseline instead.
  - `collect_errors.rs` updated to recurse through the new wrapper types so errors inside these
    bodies are still surfaced as diagnostics.
  - `PARSE_AST_VERSION` bumped 69 → 70 (new/changed AST variants: `AliasBody::Brace`,
    `Import`/`Dependency`/`ConnectStmt`/`Bind` gained fields, `RefBody::Brace`'s element type
    changed).

- **Nested `constraint` members mis-parsed inside `constraint def` / `requirement def`
  bodies.** Neither `ConstraintDefBodyElement` (`src/ast/view.rs`) nor `RequirementDefBodyElement`
  (`src/ast/requirement.rs`) had a dispatch arm for a nested `constraint` member: inside a
  `constraint def { ... }` body it fell back to the generic `expression` parser (mis-parsed as a
  garbage `Expression`); inside a `requirement def { ... }` body it fell through to the opaque
  `Other(preview)` text bucket. Confirmed against real, vendored OMG Systems Library content —
  `Systems Library/Requirements.sysml`'s `RequirementConstraintCheck` (`constraint
  assumptions[0..*] :> constraintChecks, subperformances { ... }`, nested inside a `constraint def`
  body) and `RequirementCheck` (`constraint assumptions :>> RequirementConstraintCheck::assumptions;`,
  nested inside a `requirement def` body, redefining an inherited constraint). `constraint_usage`
  itself already fully supported every one of these shapes standalone — it was only missing a
  body-element enum variant + dispatch arm in these two contexts, the same "nested-dispatch bug
  class" already closed for `dependency`/`case`/`concern`/`first`/`succession` members elsewhere.
  Both body-element enums gained a `Constraint(Node<ConstraintUsage>)` variant, wired into
  `constraint_def_body_element` (`src/parser/constraint.rs`) and `requirement_def_body_element`
  (`src/parser/requirement.rs`); `collect_errors.rs` now recurses into nested `Constraint`
  elements so errors inside a nested constraint body are still surfaced. Distinct from
  `RequireConstraint`, which continues to handle the separate `assume`/`require`-prefixed member
  kind. Fixes [#59](https://github.com/elan8/sysml-v2-parser/issues/59).
  `PARSE_AST_VERSION` bumped 68 → 69 (new AST variants).

- **Identifiers beginning with `null`, `true`, or `false` were mis-lexed.** `literal_boolean`
  and `null_expression` (`src/parser/expr.rs`) matched these three literal keywords with a bare
  `tag()`, without the trailing word-boundary check `keyword_token` already applies to every
  other bare keyword in the same file (`not`/`and`/`or`/`istype`/`as`/`new`/…) — so an identifier
  that merely starts with one of them split into the literal plus an unparseable trailing
  fragment, e.g. `flow env to nullPoint.env;` failed with an unexpected-token error. Fixed with a
  new `literal_keyword_token` helper: deliberately distinct from `keyword_token`, since that
  helper's boundary check (`starts_with_keyword`) only accepts whitespace or `{`/`:`/`;`/`[` as a
  follower — correct for operator/declaration keywords, which are always followed by an operand
  or a body-opening token, but wrong for *literal* keywords, which can legally sit immediately
  before any non-identifier byte a value can precede (`f(true)`, `x == null`, `[false, true]`).
  `literal_keyword_token` instead rejects only a following identifier-continuation byte
  (alphanumeric or `_`). Fixes [#58](https://github.com/elan8/sysml-v2-parser/issues/58).
  No AST changes; `PARSE_AST_VERSION` unchanged.

- Preserve the optional initializer expression on analysis and verification case return
  declarations such as `return attribute result : Real = expression;` and
  `return :>> result = expression;`. `CaseReturnDecl` now exposes the parsed expression through
  `value_expression`, so semantic consumers do not need to reparse source text. Declarations
  without an initializer remain supported and `return ref` continues to use its dedicated AST
  variant. Fixes [#56](https://github.com/elan8/sysml-v2-parser/issues/56).
  `PARSE_AST_VERSION` bumped 67 → 68.

### Changed

- **Deduplicated `subsetting_relationship_node`** ([#34](https://github.com/elan8/sysml-v2-parser/issues/34))
  — the same "wrap a single bare feature-name target in a `SubsettingRelationship` node" helper
  was implemented three times: the real, shared one in `usage.rs` (used by
  `subsets`/`redefinition`/`reference_subsetting`/`cross_subsetting`/`intersecting`), and
  byte-identical copies in `attribute.rs` and `part/body.rs` for their own ad hoc `:>`/`:>>`
  prefix shapes (`attribute_feature_binding`, `metadata_binding`, `exhibit_state`,
  `connection_usage_member`), each admitting in its own doc comment to "mirroring" the shared one.
  `usage.rs` gained a new `single_target_subsetting(span, kind, name)` convenience — the same
  pattern `single_target_redefines` (now implemented in terms of it) already established — and
  both local copies were deleted in favor of it. No behavior change: `cargo test` and
  `cargo test --test validation -- --include-ignored` pass identically before and after,
  confirmed via a clean re-run (no golden AST snapshot changes, since output is byte-identical).
  Also documented `DefinitionPrefixOptions`'s two ad hoc disambiguation booleans
  (`reject_header_keyword`/`reject_plain_typed_header_without_def`) in `definition_prefix.rs`
  itself as a known, not-yet-unified pattern — no third case has appeared yet to justify
  unifying them, per the issue's acceptance criteria.

`PARSE_AST_VERSION` bumped 70 → 71: this release's AST changes are additive (new struct fields,
new enum variants) but still shape-breaking for exhaustive matches/cached parses -- `InOutDecl`
gained `is_redefinition`; `CalcDefBodyElement` gained `CalcDef`; `InterfaceDefBodyElement` gained
`FlowUsage`; `InterfaceUsageBodyElement` gained `EndDecl`; `InterfaceUsage::TypedConnect` gained
`name`; `EndDecl` gained `crosses`.

## [0.53.0] - 2026-08-03

### Fixed

- **`end` declaration: unidentified dual-name/kind shape**
  ([#53](https://github.com/elan8/sysml-v2-parser/issues/53)) — resolves the shape tracked as a
  known issue in [0.52.0](#0520---2026-08-03): `end theCauses [*] occurrence theCause :> causes
  :>> source { ... }` (`Domain Libraries/Cause and Effect/CausationConnections.sysml`) and `end
  touchesToo [0..*] item touchedItemToo :>> separateSpaceToo, thisOccurrence;` (`Items.sysml`).

  Reverse-engineering the BNF text further did not turn up a matching production. The shape was
  identified empirically instead: the text after the first name/multiplicity (`occurrence
  theCause :> causes :>> source { ... }` / `item touchedItemToo :>> separateSpaceToo,
  thisOccurrence;`) parses standalone, with zero diagnostics, as a complete `occurrence_usage`/
  `item_usage`. So `end`'s target position has a third alternative beyond `:` typing and
  `::>`/`references`: a fully embedded, kind-prefixed nested usage. `EndDecl` gained a
  `nested_usage: Option<Box<EndNestedUsage>>` field (new `EndNestedUsage` enum, `Occurrence`/
  `Item` variants — only these two are evidenced). `end_decl` (`src/parser/connector.rs`) also
  gained a "middle" multiplicity position, between the end's own name and this nested usage
  (`theCauses [*]`/`touchesToo [0..*]` above), distinct from the pre-existing leading and trailing
  multiplicity positions.

  Both previously-known-issue files now parse with zero diagnostics; the `KNOWN_ISSUE_FILES`/
  `is_known_issue_file` tracking removed from `tests/validation/full_library_suite.rs` and
  `tests/conformance_scorecard.rs` since the underlying gate now genuinely passes. New regression
  tests in `tests/gh53_end_decl_nested_usage.rs` cover both nested-usage kinds plus confirmation
  that the pre-existing `end` forms (typed, `::>`/`references`, trailing `:>>` redefines) are
  unaffected. `PARSE_AST_VERSION` bumped 66 → 67.

## [0.52.0] - 2026-08-03

### Fixed

- **`interface_def_body` silently discarded unparseable content with no diagnostic**
  ([#51](https://github.com/elan8/sysml-v2-parser/issues/51)) — `interface_def_body`
  (`src/parser/interface.rs`) was a hand-rolled `many0` loop whose only fallback for an
  unrecognized member was `advance_to_closing_brace`, which skips straight to the closing `}`
  with no diagnostic at all. Routed through the same `parse_structured_brace_members` +
  recovery-node pattern `connection_member_body` already used; `InterfaceDefBodyElement` gained an
  `Error` variant to carry the recovery node, mirroring `ConnectionDefBodyElement::Error`.

  Fixing this properly required also wiring `collect_errors.rs` to walk `ConnectionDef`/
  `InterfaceDef` bodies at all — previously **neither** was ever collected into
  `parse_with_diagnostics`'s `result.errors`, at any nesting level (package-level, nested in a
  `part def`/`part` usage), so even `connection_member_body`'s already-correct recovery nodes were
  silently invisible before this fix. `collect_connection_def_body_errors`/
  `collect_interface_def_body_errors` added and wired into `PackageBodyElement`,
  `PartDefBodyElement`, and `PartUsageBodyElement` (connection only; interfaces aren't dispatched
  there) dispatch.

  Making these diagnostics visible for the first time surfaced several previously-invisible real
  parser gaps against the vendored SysML v2 Systems/Domain Libraries (confirmed via the strict
  full-library "zero diagnostics" validation gate going from silently green to honestly red — 5
  of 94 files). Fixed the following, each confirmed against real library usage:
  - `end` declarations: `:>>` redefines trailing the typed (`:`) form (`end source: Anything :>>
    BinaryLinkObject::source;`, `Connections.sysml`) — `EndDecl` gained a `redefines` field.
  - `end` declarations: leading multiplicity before the kind keyword/name (`end [1] part bead :
    TireBead;`, `end [*] ref cause: Situation;`) and `ref` as an accepted end kind keyword
    alongside `part`/`port`.
  - `assert constraint` members with a visibility prefix, dispatched into connection/interface
    def bodies for the first time (`private assert constraint disjointCauseEffect { ... }`,
    `CausationConnections.sysml`/`DerivationConnections.sysml`) — `AssertConstraintMember` gained
    a `membership` field; `assert_constraint_member` now parses a leading visibility prefix
    everywhere it's used, not just in these two files.
  - `occurrence` usages with `abstract`/`constant` prefixes and a multiplicity, dispatched into
    connection def bodies for the first time (`abstract constant ref occurrence causes[1..*] :>>
    causes :> participant { ... }`, `CausationConnections.sysml`) — `OccurrenceUsage` gained
    `is_abstract`, `is_constant`, and `multiplicity` fields (none of which `occurrence_usage`
    supported in *any* context before this).
  - Named `succession` usages, dispatched into connection def bodies for the first time (`private
    succession causalOrdering first [nCauses] causes.startShot then [nEffects] effects { ... }`,
    `CausationConnections.sysml`) — `SuccessionUsage` gained a `name` field (previously
    `succession_usage` had no way to name the succession itself, in any context).
  - `ref_decl` rebuilt to support an optional kind keyword (`part`/`port`/`item`/`requirement`),
    optional name (anonymous when redefining), `:>>` redefines (multi-target aware, reusing the
    shared `redefinition` parser), `:>` subsets (new `RefDecl.subsets` field), multiplicity, and
    `ordered`/`nonunique` modifiers — previously required a name and a `:` type unconditionally
    with none of the above (`ref port :>> participant : Port [2..*] nonunique ordered { ... }`,
    `ref port :>> Interface::participant, BinaryConnection::participant[2] nonunique ordered;`,
    `Interfaces.sysml`; `ref requirement originalRequirement[1] :>> originalRequirements :>
    participant { ... }`, `DerivationConnections.sysml` — `requirement` here is just another
    `ref_decl` kind keyword, not the separate `requirement_usage` parser).

  New regression tests in `tests/gh51_connection_interface_body_gaps.rs` cover each fix using the
  real (trimmed) library lines that motivated it. `PARSE_AST_VERSION` bumped 65 → 66.

  **Not fixed, tracked separately as [#53](https://github.com/elan8/sysml-v2-parser/issues/53):**
  a still-unidentified `end` declaration shape appearing in exactly 2 files (`end theCauses [*]
  occurrence theCause :> causes :>> source { ... }` in `CausationConnections.sysml`; `end
  touchesToo [0..*] item touchedItemToo :>> separateSpaceToo, thisOccurrence;` in `Items.sysml`) —
  a first name, multiplicity, kind keyword, and *second* name all on one `end`. The
  `ConnectorEnd`/`ConnectorEndMember` BNF productions found while auditing this cover the `connect
  ... to ...` statement's own arguments, not this member-declaration form, and no other matching
  production was located; landing a guess at unverified grammar risks a silently-wrong AST, which
  is worse than the current honest diagnostic. Both files are carried as a documented, tracked
  exception in `tests/validation/full_library_suite.rs`'s `KNOWN_ISSUE_FILES` so the strict gate
  stays otherwise green.

## [0.51.9] - 2026-08-03

### Changed

- **Deduplicated `connection.rs`/`interface.rs`'s seven near-identical connector-end functions**
  ([#33](https://github.com/elan8/sysml-v2-parser/issues/33)) — `end_decl`, `ref_body`,
  `ref_decl`, `connect_body`, the connection-end wrapper, `connect_ends`, and `connect_stmt` were
  each implemented twice, once per file, with only cosmetic naming differences
  (`connection_end`/`connect_end`, etc.). This had already cost real double work once
  ([#19](https://github.com/elan8/sysml-v2-parser/issues/19) required the same reference-
  subsetting fix in both files), and, worse, had silently drifted into two genuine behavior gaps
  neither file's own tests caught:
  - `connection.rs`'s typed `end` form (`end name : Type;`) never accepted the `~`
    conjugated-type prefix `interface.rs`'s did (e.g. `end p1 : ~PowerPort;`, real usage in
    `KitchenTimer.sysml`/`SurveillanceDrone.sysml`) — no BNF basis for the restriction, since
    `ConnectorEnd` and `InterfaceEnd` (§8.2.2.13.2/§8.2.2.14.2) are the same production.
  - `interface.rs`'s `connect_ends` never accepted the §6 G24 per-endpoint multiplicity
    (`connect [0..1] a to [1] b;`) `connection.rs`'s did — same reasoning, no BNF basis for
    restricting it to connections only.

  Both gaps are now fixed for both contexts by construction: all seven functions moved into a new
  shared module, `src/parser/connector.rs`, and `connection.rs`/`interface.rs` call into it
  instead of each maintaining their own copy. The one genuine, evidenced difference between the
  two — connections accept the `#name` derived-end-name form (tested in
  `tests/derivation_connections.rs`), interfaces have no matching real-usage evidence for it —
  stays an explicit parameter (`end_decl`'s `allow_derived_name`) rather than being blindly
  unified or silently dropped.

  New regression tests in `tests/gh33_connector_consolidation.rs` assert that a capability added
  to the shared implementation is visible from both `connection_def` and `interface_def` parsing,
  so a future two-file-fix regression like #19 would be caught structurally here rather than
  relying on remembering to test both files. No AST schema changes (same fields throughout, only
  the accepted syntax widened); `PARSE_AST_VERSION` unchanged.

  Also noted, not fixed here (pre-existing, unrelated to this consolidation): `interface_def_body`
  silently discards unparseable body content via `advance_to_closing_brace` with no diagnostic,
  unlike `connection_member_body`'s proper `parse_structured_brace_members`-based recovery — found
  while testing the `#name` parameter above (confirmed on `main` before this change too, with a
  minimal unrelated-malformed-content repro). Tracked as a follow-up, not part of GH-33's
  connector-end-duplication scope.

## [0.51.8] - 2026-08-03

### Fixed

- **`missing_expression_after_operator_diagnostic` scanned the unbounded rest of the file, same
  far-field poisoning bug shape as GH-18** ([#29](https://github.com/elan8/sysml-v2-parser/issues/29))
  — follow-up audit after #18/#28. That earlier fix added `local_statement_window`
  (`src/parser/diagnostics.rs`) to bound `invalid_unit_reference_diagnostic` and
  `bare_comma_sequence_diagnostic` to the current statement, but was scoped narrowly to the one
  function #18 reported. `missing_expression_after_operator_diagnostic`
  (checking `.contains("= ;")`/` then ;`/` to ;`/` by ;`, etc.) had the identical bug: it scanned
  `String::from_utf8_lossy(fragment)` over the unbounded rest of the file rather than the local
  window, so an unrelated `= ;`-like substring in a comment far below the real error could
  override the true diagnostic (e.g. a genuinely malformed `bind x = 123abc!;` misreported as
  "expected expression after '='" because of a doc comment two lines later reading `// unrelated
  note: default = ; here`).

  Routed it through the same `local_statement_window` bound. Audited every other classifier in
  `src/parser/diagnostics.rs` and `src/parser/recovery.rs` for the same anti-pattern
  (unbounded `.contains()`/string-scan instead of a bounded local window): `missing_type_diagnostic`,
  `invalid_expose_separator_diagnostic`, `missing_semicolon_or_body_diagnostic`,
  `invalid_typing_operator_diagnostic`, `invalid_bare_identifier_in_body_diagnostic`, and
  `unexpected_keyword_in_scope_diagnostic` already self-bound via prefix-anchored token walks or
  their own delimiter-position logic; the `recovery.rs` classification sites bound their scans to
  the actual recovery-consumed span, not the unbounded fragment. Confirmed
  `missing_expression_after_operator_diagnostic` was the only other instance.

  No AST changes; diagnostics-only.

## [0.51.7] - 2026-08-03

### Fixed

- **`bind_` never parsed the `binding` name/type/multiplicity prefix, nor per-endpoint
  multiplicity on `bind`'s own operands** ([#48](https://github.com/elan8/sysml-v2-parser/issues/48),
  Gap 2 of [#42](https://github.com/elan8/sysml-v2-parser/issues/42), refined with additional
  real-usage evidence found while scoping it — Gap 1 landed in
  [#47](https://github.com/elan8/sysml-v2-parser/pull/47)). `bind_`
  (`src/parser/part/usage.rs`) only ever parsed the bare `bind a = b;` form.

  Two gaps, both evidenced by the vendored SysML v2 Systems Library / spec Annex examples:
  - **Gap 2a**: the optional `'binding' UsageDeclaration` prefix (BNF `BindingConnectorAsUsage`,
    §8.2.2.13.2) — naming/typing the binding connector itself, e.g. `binding ab bind a = b;` /
    `binding ab1 : AB bind a = b;` (`ConnectionTest.sysml` lines 23–24). Added `binding_prefix`,
    mirroring `succession_prefix`'s exact structure for the sibling `SuccessionAsUsage` production
    (added for #38) — same optional name / `: Type` / multiplicity shape.
  - **Gap 2b**, found while checking real usage before scoping Gap 2a (not in the original #42
    description): each of `bind`'s own two operands (`ConnectorEnd` per the BNF) may carry its own
    leading multiplicity, e.g. `binding [1] bind [0..*] base.edges = [0..*] be;` — 13 occurrences
    in Systems Library `Domain Libraries/Geometry/ShapeItems.sysml`. Mirrors `connect_`'s
    `from_multiplicity`/`to_multiplicity` (§6 G24).

  `Bind` (`src/ast/structure.rs`) gained `binding_name`, `binding_type`,
  `binding_multiplicity`, `left_multiplicity`, and `right_multiplicity` fields, following
  `FirstStmt`'s flat-fields shape (plain `Option<Node<Multiplicity>>` alongside the existing
  `left`/`right: Node<Expression>`) rather than restructuring into `ConnectionEnd`, to keep the
  change additive for existing `Bind` construction sites. `PARSE_AST_VERSION` bumped 64 → 65.

## [0.51.6] - 2026-08-03

### Fixed

- **Bare `bind a = b;` not dispatched inside a `part def` body**
  ([#42](https://github.com/elan8/sysml-v2-parser/issues/42) Gap 1) — discovered while fixing
  [#40](https://github.com/elan8/sysml-v2-parser/issues/40). Real usage:
  `sysml-v2-release/sysml/src/examples/Simple Tests/ConnectionTest.sysml` line 22, inside a
  `part def P { ... }` body. `bind_` (BNF `BindingConnectorAsUsage`, §8.2.2.13.2,
  `src/parser/part/usage.rs`) was already wired into part *usage* bodies
  (`part_usage_body_element`), but `part_def_body_element` (`src/parser/part/body.rs`) had no
  `Bind`/`bind_` arm and `PartDefBodyElement` had no `Bind` variant at all — same class of missing
  dispatch-wiring gap as `first`/`succession` (#40). `PartDefBodyElement` gained a `Bind` variant
  wrapping the existing `Bind` AST node (no new AST type). `PARSE_AST_VERSION` bumped 63 → 64.

  Out of scope, per the issue: the `binding <name> (: Type)? bind ...` named-prefix form (Gap 2)
  doesn't parse anywhere yet, in either def or usage bodies — a bigger fix requiring `bind_` to
  grow a new sub-parser mirroring #38's `succession_prefix` pattern, tracked separately.

## [0.51.5] - 2026-08-03

### Fixed

- **`state`/`exhibit state` rejected `direction`/`derived`/`individual` prefixes**
  ([#45](https://github.com/elan8/sysml-v2-parser/issues/45)) — follow-up to
  [#27](https://github.com/elan8/sysml-v2-parser/issues/27). Per BNF `RefPrefix`/
  `OccurrenceUsagePrefix` (§8.2.2.9.2, §8.2.2.18.2), `state_usage`
  (`src/parser/state.rs`) and `exhibit_state` (`src/parser/part/body.rs`) only ever implemented
  the visibility/`abstract`/`ref` slice of that prefix — direction (`in`/`out`/`inout`),
  `derived`, and `individual` were all rejected, even though the same keyword handling already
  backs `part`/`item`/`attribute` usages elsewhere in this parser
  (`crate::parser::attribute::direction_prefix`, the `derived`/`individual` opt-tag pattern in
  `src/parser/part/usage.rs::part_usage`).

  Checked against the vendored SysML v2 Systems Library / spec Annex examples before scoping:
  zero real usage of `constant` or portion kind (`snapshot`/`timeslice`) on state usages, so those
  two remaining `OccurrenceUsagePrefix` slots are intentionally still not implemented — direction,
  `derived`, and `individual` were the only ones the issue asked for and that mirror
  already-proven patterns elsewhere in the crate.

  `StateUsage` (`src/ast/behavior.rs`) and `ExhibitState` (`src/ast/structure.rs`) both gained
  `direction: Option<InOut>`, `is_derived: bool`, and `is_individual: bool` fields, in the BNF's
  prefix order (direction, `derived`, `abstract`, `ref`, `individual`). `exhibit_state` was
  updated in lockstep with `state_usage` so the two shared-grammar parsers stay in sync, per the
  approach #27 established. `PARSE_AST_VERSION` bumped 62 → 63.

## [0.51.4] - 2026-08-03

### Fixed

- **`exhibit state` rejected modifiers legal on plain `state` usages**
  ([#27](https://github.com/elan8/sysml-v2-parser/issues/27)) — follow-up to
  [#17](https://github.com/elan8/sysml-v2-parser/issues/17). `exhibit_state`
  (`src/parser/part/body.rs`) was a narrow, bespoke parser never wired up to the shared
  prefix/specialization machinery `state_usage` (`src/parser/state.rs`) already uses, so
  `abstract`/visibility/`:>` subsets/multiplicity/`ordered`/`nonunique` were all rejected on
  `exhibit state` even though the equivalent plain `state` usage accepted them.

  `exhibit_state` is now built on the same shared helpers `state_usage` composes
  (`visibility_prefix`, `abstract`/`ref` prefix handling, `specialization_clauses`,
  `optional_typings`, `multiplicity_node`, `skip_usage_feature_modifiers`), so the two stay in
  sync with the shared `StateUsageBody` tail. `ExhibitState` (`src/ast/structure.rs`) gained
  `is_abstract`, `is_reference`, `typing`, `multiplicity`, `subsets`, and `membership` fields to
  carry the newly-parsed data, mirroring `StateUsage`'s shape; `exhibit_state_as_state_usage`
  (`src/parser/part/usage.rs`) now threads these through instead of hardcoding them away.
  `PARSE_AST_VERSION` bumped 61 → 62.

  Out of scope, per the issue: direction (`in`/`out`/`inout`), `derived`, `individual`, and
  portion-kind prefixes are part of the full BNF `OccurrenceUsagePrefix` but aren't supported by
  `state_usage` itself either — extending both together is a separate, pre-existing gap, not one
  `exhibit_state` drifted into on its own.

## [0.51.3] - 2026-08-03

### Fixed

- **`part def` bodies did not accept `first`/`succession` succession syntax**
  ([#40](https://github.com/elan8/sysml-v2-parser/issues/40)) — flagged as out of scope while
  fixing [#38](https://github.com/elan8/sysml-v2-parser/issues/38): not even the bare
  `first a then b;` form (already supported inside action bodies) parsed inside a `part def`
  body. Real-world evidence: `sysml-v2-release/sysml/src/examples/Simple
  Tests/ConnectionTest.sysml`'s `part def P { ... }` body uses all three forms back to back
  (`first a then b;`, `succession s first a then b;`, `succession s1 : AB first a then b;`), all
  of which previously failed with `unexpected keyword 'first' in part definition body` /
  `unexpected token in part definition body`.

  `part_def_body_element` (`src/parser/part/body.rs`) now dispatches to the same `first_stmt`
  parser action bodies already use (`src/parser/action.rs`, made `pub(crate)`), wrapped so the
  `then` clause is mandatory (see below). `first` was added to `PART_BODY_STARTERS`
  (`succession` was already listed as a starter keyword but had no parser wired up, so it always
  fell through to recovery). `PartDefBodyElement` gained a `FirstStmt` variant wrapping the
  existing `FirstStmt` AST node (no new AST type). `PARSE_AST_VERSION` bumped 60 → 61.

  Per BNF `SuccessionAsUsage` (§8.2.2.13.3), this succession form is reachable from the generic
  `DefinitionBodyItem` grammar `PartDefinition` uses (via `NonOccurrenceUsageElement`), so it
  belongs in a `part def` body. `merge`/`decide`/`join`/`fork` (BNF `ActionNode`) do **not**: per
  the BNF, `ActionNodeMember` is reachable only from `ActionBodyItem` (action def/usage bodies),
  not `DefinitionBodyItem` — no example in the vendored library nests them in a `part def` body
  either. An earlier draft of this fix wired all four in; checked against the spec text and
  narrowed to just `first`/`succession`. Likewise, the `then`-less `first target;` initial-node
  marker (BNF `InitialNodeMember`) is `ActionBodyItem`-only, so the shared `first_stmt` parser is
  wrapped to reject it here even though it's accepted (as an optional `then`) inside action
  bodies.

  Out of scope: a bare `bind a = b;` member (without the `binding` keyword) inside a `part def`
  body also fails to parse — a separate, pre-existing gap `ConnectionTest.sysml` happens to
  exercise a few lines above the succession syntax fixed here — tracked separately, not fixed in
  this change.

## [0.51.2] - 2026-08-03

### Fixed

- **Optional `succession` keyword prefix on `first ... then ...` statements not accepted**
  ([#38](https://github.com/elan8/sysml-v2-parser/issues/38)) — per BNF `SuccessionAsUsage`
  (§8.2.2.13.3), the `first ... then ...` control node inside an action body may carry a leading
  `succession` keyword, optionally itself named, typed, and/or given a multiplicity, before the
  `first`/`then` clause. `first_stmt` (`src/parser/action.rs`) only ever accepted the bare
  `first ... then ...;` form. Real-world evidence: the vendored SysML v2 Systems Library uses the
  unnamed, multiplicity-bearing form extensively (`Flows.sysml`: `succession [seBeforeNum] first
  [0..1] sourceEvent then [0..1] self;`; `States.sysml`: `succession stateSequencing first [0..1]
  exclusiveStates then [0..1] exclusiveStates { ... }`), and `sysml/src/examples/Simple
  Tests/ConnectionTest.sysml` uses the named and named+typed forms (`succession s first a then
  b;`, `succession s1 : AB first a then b;`).

  `first_stmt` now accepts an optional `succession` (name)? (`: Type`)? (`[mult]`)? prefix, plus
  an optional multiplicity on each of the `first`/`then` ends (`first [0..1] a then [0..1] b`),
  mirroring the `[mult]` end-multiplicity handling `connection.rs`'s `connect_ends` already has
  for `connect [0..1] a to [1] b;`. `FirstStmt` gained `succession_name: Option<String>`,
  `succession_type: Option<String>`, `succession_multiplicity: Option<Node<Multiplicity>>`,
  `first_multiplicity: Option<Node<Multiplicity>>`, and `then_multiplicity:
  Option<Node<Multiplicity>>` (all additive, `None` for the pre-existing bare `first ... then
  ...;` form). `PARSE_AST_VERSION` bumped 59 → 60.

  Out of scope: `part def` bodies don't accept *any* `first`/`succession`/`merge`/`decide`
  control-node syntax at all (a much wider, pre-existing gap `ConnectionTest.sysml` also happens
  to exercise) — tracked separately, not fixed here.

## [0.51.1] - 2026-07-31

### Fixed

- **Connection/interface `end` decl rejected a structural kind keyword and trailing multiplicity**
  (GH-19/GH-20 follow-up) — the real-world examples from both issues (and the upstream
  [elan8/spec42#1](https://github.com/elan8/spec42/issues/1)/[#2](https://github.com/elan8/spec42/issues/2)
  reports) use `end part hub : Hub;` and `end part hub ::> mainSwitch[1];`, neither of which
  actually parsed even after the GH-19/GH-20 fixes landed: `connection.rs`'s `end_decl` accepted
  no structural kind keyword at all (`interface.rs` only accepted a bare `port`, and it lacked a
  word-boundary check — see below), and the shared `usage::reference_subsetting` parser only
  consumes the qualified target, never a following `[mult]` bracket (matching every other
  subsetting-family clause; callers that need one, like `part_usage_redefines_only`, already parse
  it as a separate subsequent step). Both examples silently recovered to `Error` nodes with
  `result.errors` staying empty, exactly the "recovery hides it" trap both issues warned about.

  Both `end_decl`s now accept an optional `part`/`port` keyword after `end` (BNF
  `InterfaceOccurrenceUsageElement`/`StructureUsageElement`; not retained as a separate field --
  `EndDecl` doesn't model usage-kind distinctions, same simplification already made for
  `interface.rs`'s pre-existing `port` handling) and an optional trailing multiplicity on both the
  `:` typing and `::>`/`references` forms, captured in a new `EndDecl.multiplicity:
  Option<Node<Multiplicity>>` field. `PARSE_AST_VERSION` bumped 58 → 59 (additive field).

  Also fixed a latent word-boundary bug surfaced while adding the `part`/`port` keyword: a bare
  `tag("part")`/`tag("port")` (the latter pre-existing in `interface.rs`) matches as a prefix of
  any longer name starting with those letters (e.g. `end party ::> acmeLtd;` would have silently
  split into keyword `part` + one-letter name `y`). Both keyword checks now require a following
  `ws1` (mandatory whitespace), matching the same safety net `end`'s own `tag` + `ws1` already
  relies on.

## [0.51.0] - 2026-07-31

### Fixed

- **Package-level `connection name : Type { ... }` misclassified as `ConnectionDef`**
  ([#20](https://github.com/elan8/sysml-v2-parser/issues/20)) — package/namespace-level structure
  dispatch tries `connection_def` before `ConnectionUsage`'s `connection_usage_member`.
  `connection_def` keeps `def` optional (PAR-006b) so genuine bare Systems-Library definitions
  (`abstract connection connections: Connection[0..*] nonunique :> linkObjects, parts { ... }`)
  still parse, but that same `def`-optional grammar is a strict superset of an ordinary named
  typed connection *usage* (SysML v2 §7.13.2, e.g. `connection connection1 : DeviceConnection {
  end part hub ::> mainSwitch[1]; ... }`), so the usage form never fell through to
  `connection_usage_member` — it was misclassified as a `ConnectionDef` with `: DeviceConnection`
  stored as a `TypingRelationship { kind: Typing }` on `specializes`. Downstream, Spec42
  materialized it as a `connection def` and reported a false `incompatible_specializes_kind`
  warning treating `connection1` as specializing a definition it can't specialize
  ([elan8/spec42#1](https://github.com/elan8/spec42/issues/1)).

  Added `DefinitionPrefixOptions::reject_plain_typed_header_without_def` (mirroring PAR-007's
  `reject_header_keyword`): fails the definition parse for the `def`-less, non-`abstract` plain
  `: Type` header shape (`kind: Typing`, no `:>`/`specializes` clause), leaving it for the sibling
  usage parser. `abstract` and an actual `:>`/`specializes` clause in the header remain
  unambiguous definition-only signals (a usage's own `:>` is a `subsets` clause positioned *after*
  the body, not in this header), so the bare Systems-Library form and explicit `connection def
  name : Type;` are both unaffected. Applied only to `connection_def`'s package-level entry point,
  not `connection_def_required` (already `def`-required for nested contexts). No AST changes;
  `PARSE_AST_VERSION` unchanged.

- **Connection/interface `end` decl `::>`/`references` not modeled as reference subsetting**
  ([#19](https://github.com/elan8/sysml-v2-parser/issues/19)) — `end name ::> target;` was
  accepted by `connection.rs`'s `end_decl`, but the target was stored as typing (`type_name`)
  rather than reference subsetting, and the keyword spelling `end name references target;`
  (`references_operator` already treats `::>` and `references` as equivalent for usage headers —
  that equivalence was missing here) wasn't accepted at all, falling through to recovery.
  `interface.rs`'s `end_decl` only ever accepted `:` typing, so both spellings failed there.
  Both `end_decl`s now try `usage::reference_subsetting` (the same `::>`/`references` parser
  every other reference-subsetting clause uses) before falling back to `:` typing, and `EndDecl`
  gained a new `references: Option<Node<SubsettingRelationship>>` field (`kind:
  SubsettingKind::References`) alongside the existing `type_name`/`uses_derived_syntax` fields
  (kept for display/backward compatibility, mirroring `RefDecl.type_name`/`typing`). `end name :
  Type;` typing is unaffected. `PARSE_AST_VERSION` bumped 57 → 58 (additive `EndDecl` field).
  Downstream, Spec42 should wire `referencesFeature`/`ReferenceSubsetting` from this field instead
  of `endType` for the `::>`/`references` form.

- **Misleading diagnostics and spurious recovery failures**
  ([#18](https://github.com/elan8/sysml-v2-parser/issues/18)) — three related recovery/diagnostic
  bugs, none requiring grammar changes:
  - `invalid_unit_reference_diagnostic` scanned the *unbounded* rest of the file for `[`/`]`
    patterns, so bracket-like text anywhere below the real error — including inside a `//` or
    `/* */` doc comment (e.g. a `[ ]` TODO marker) — could override the true diagnostic with a
    spurious `expected unit name inside '[ ]'`. It now scans only the local
    statement/member window (`local_statement_window` in `diagnostics.rs`), stopping before any
    comment, at the first depth-0 `;`, or at an unmatched closing delimiter.
  - `unexpected_keyword_in_scope_diagnostic` labeled *any* unrecognized leading identifier as an
    "unexpected keyword", even when it wasn't a SysML keyword at all (e.g. `test`,
    `distancePerVolume`), inverting the debugging signal (implying unsupported valid syntax
    instead of an input defect). It now checks the identifier against the reserved keyword list
    of the SysML v2 textual notation (OMG SysML v2.0, §8.2.2.1.2; new
    `lex::is_reserved_keyword`) and only reports `unexpected_keyword_in_scope` for genuine
    keywords; anything else is reported as a new `unrecognized_declaration_in_scope` diagnostic
    (`unrecognized declaration '<name>' in <scope>`).
  - A comma-separated value list without sequence brackets (e.g. `part :>> readings = a, b;`)
    stopped the expression parser at the comma and fell through to a generic `unexpected token in
    part definition body` diagnostic that didn't call out the comma or the missing `( ... )`. A
    new `bare_comma_sequence_diagnostic` detects a depth-0 `,` following a value-assignment `=`/
    `:=` within the local statement window and reports a targeted `bare_comma_in_feature_value`
    diagnostic instead.

  No AST changes; `PARSE_AST_VERSION` unchanged. Two new stable diagnostic codes added to the
  catalog: `unrecognized_declaration_in_scope`, `bare_comma_in_feature_value`.

- **`exhibit state ... parallel { ... }` was rejected**
  ([#17](https://github.com/elan8/sysml-v2-parser/issues/17)) — `exhibit_state` went straight
  from the (optional) pre-body redefinition to `state_def_body`, never trying the
  `parallel`/`initial` modifier that plain `state` usages accept there. But per the OMG spec
  (§8.2.2.18.2), `ExhibitStateUsage` shares the same `StateUsageBody` production as `StateUsage`,
  so the modifier is equally legal on `exhibit state` — as seen in the spec's own Annex A example,
  `exhibit state vehicleStates parallel { ... }` (`5-State-based Behavior-2.sysml`). `exhibit_state`
  now tries the same optional `parallel`/`initial` modifier before the body that `state_usage`
  does. No AST changes (the modifier was already discarded, unretained, by `state_usage` too);
  `PARSE_AST_VERSION` unchanged.

- **Parsing the SysML v2 spec's own Annex A vehicle example crashed the process with
  `STATUS_STACK_OVERFLOW`** — unlike the earlier `expr.rs` stack-overflow class of bug (see
  below), this wasn't unbounded/attacker-controlled recursion: the file's real, legitimate
  nesting (well under `MAX_SYNTAX_NESTING`) was simply deep enough that unoptimized (debug)
  builds — which spend far more stack per recursive-descent call frame than release builds do —
  could exhaust a caller's default thread stack (e.g. the ~1 MiB Windows main-thread default)
  purely from valid content, aborting the whole process instead of returning a `ParseError`.
  `parse_root` and `parse_with_diagnostics` now run their recursive descent through
  `stacker::maybe_grow`, switching to a generously-sized (64 MiB) stack segment whenever the
  caller's current stack doesn't already have enough headroom for the worst case the grammar's
  `MAX_SYNTAX_NESTING` limit allows; well-provisioned callers pay nothing extra. Added a
  regression test parsing the real Annex A fixture (gated via `SYSML_V2_RELEASE_DIR`, same as
  other release-fixture tests). No AST or behavior changes; `PARSE_AST_VERSION` unchanged.

- **Typed `interface` usage rejected as a body member, and bracketed connect-end multiplicity
  broke every interface usage that had it** ([#16](https://github.com/elan8/sysml-v2-parser/issues/16)) —
  `interface_usage` unconditionally required either a `connect` clause or a bare `from to to`
  form, so the plain declared form (BNF `InterfaceUsageDeclaration`'s `('connect'
  InterfacePart)?` is optional) — e.g. `interface hubToRim : SpokeInterface;` — failed to parse
  at all and fell through to opaque recovery with a misleading `expected valid part definition
  body element` / `missing semicolon before next declaration` diagnostic depending on what
  followed. Separately, `connector_end_expression` had no support whatsoever for the leading
  cross-multiplicity Annex/Systems-Library fixtures write on connect ends (`connect [1] a to [1]
  b;`), so *every* interface usage using that real, spec-legal shape failed too. Fixing the
  multiplicity gap also surfaced a latent whitespace bug: combining a leading `[mult]` with a
  named end-reference (`connect [1] p1 ::> a.p1 to ...;`) failed because the multiplicity
  consumer didn't skip the following whitespace before the name/`::>` check ran.
  <br>Added `InterfaceUsage::Declaration` for the connect-less form, gave `connector_end_expression`
  bracketed-multiplicity support (discarded, matching the existing end-name handling — neither is
  modeled on `InterfaceUsage::from`/`to` yet), and fixed the whitespace gap. Verified against the
  SysML v2 spec grammar (`InterfaceUsageDeclaration`, §8.2.2.14.2) and real Annex fixtures
  (`SysML v2 Spec Annex A SimpleVehicleModel.sysml`, `IssueMetadataExample.sysml`). Bump
  `PARSE_AST_VERSION` 56 → 57.

- **`action def` nested inside a `part def` body rejected** ([#14](https://github.com/elan8/sysml-v2-parser/issues/14)) —
  `part_def_body_element` dispatched every other nested `def` kind added under PAR-002
  (`state def`, `flow def`, `connection def`, `port def`, `calc def`, …) but never wired
  `action_def`, so a nested action *definition* fell through to opaque recovery with a
  misleading `expected ';' or '{' after action definition header` diagnostic, even though
  the corresponding usage (`action getTile;`) and the same definition at package level both
  parsed fine. Added a new `PartDefBodyElement::ActionDef` variant and dispatch `action_def`
  before `action_usage` (matching the guard pattern used for the other kinded defs). Bump
  `PARSE_AST_VERSION` 55 → 56.

- **Structural / assert / variation members rejected in action bodies** ([#13](https://github.com/elan8/sysml-v2-parser/issues/13)) —
  `ActionBodyItem` (BNF §8.2.2.17.1 via `NonBehaviorBodyItem`) admits
  `StructureUsageMember` (`part`, `item`, `snapshot`/`PortionUsage`) and
  `BehaviorUsageMember` (`assert constraint`), plus `RefPrefix.isVariation` on
  nested `action` usages. The action-body dispatcher previously only covered
  control nodes, behavior steps, and opaque `attribute`/`calc`/`event` decls, so
  these members reported `unexpected keyword … in action body`.
  <br>Wired `PartUsage` / `ItemUsage` / `AssertConstraint` / `OccurrenceUsage`
  into both `ActionDefBodyElement` and `ActionUsageBodyElement`, and added
  `ActionUsage.is_variation`. Also accept `then perform …` (`ThenTarget::Perform`)
  and anonymous `action accept`/`action send` payload forms used by the Systems
  Library / Annex fixtures. Loop keyword remains `for` (`ForLoopNode`), not
  `foreach`. Bump `PARSE_AST_VERSION` 54 → 55.

- **Arbitrary non-SysML text accepted inside part bodies** ([#12](https://github.com/elan8/sysml-v2-parser/issues/12)) —
  `part def` / `part` usage bodies silently accepted junk such as `%%% … %%%` with no diagnostic:
  definition recovery fell through to `PartDefBodyElement::Other` (ignored by error collection),
  and usage bodies hard-failed on non-starters so the package path swallowed the whole decl as
  `ExtendedLibraryDecl`. Both paths now always recover unrecognized members as Error nodes with
  `unexpected token in part … body`, matching package/attribute/port. `part_usage_body_brace`
  uses the shared structured-brace recovery helper.
  <br>Closing the silent fallthrough also required wiring part-*usage* body members that Annex
  fixtures already rely on and that part-*definition* bodies already accepted:
  keyword-less `:>>` redefinition bindings (`redefinition_feature_binding`), `metadata` usage,
  and `analysis` / `analysis def` case members. Bump `PARSE_AST_VERSION` 53 → 54.

- **`ref part … :> …` rejected as unexpected token** ([#10](https://github.com/elan8/sysml-v2-parser/issues/10)) —
  `part_ref_usage` only accepted `(visibility)? ref (part)? (:>>)? name (: type)? (= value)? body`,
  so a post-name `:>` / `subsets` clause failed in both part definition and usage bodies. Per BNF
  §8.2.2.6.2 / §8.2.2.11, `ref part` is `PartUsage` with `BasicUsagePrefix.isReference`, which
  includes the full `FeatureSpecializationPart` (same as plain `part x : T :> y;`).
  <br>`part_usage` / `part_def_or_usage` now parse the leading `ref` and set new
  `PartUsage.is_reference`. `part_ref_usage` rejects `part` (like other kinded refs) so bare
  `ref name …` remains `ReferenceUsage` / `RefDecl`. Bump `PARSE_AST_VERSION` 52 → 53.

## [0.50.0] - 2026-07-30

### Fixed

- **PARSER_BACKLOG_ROADMAP.md §6 G4–G20 grammar gaps** — closed the remaining confirmed
  spec-Annex construct families from the 2026-07-30 strict-vs-recovery audit (on top of the
  already-committed G1–G3 and short-name work at `PARSE_AST_VERSION` 50–51):
  - **G4** — `constraint` usage/def wired into `PartDefBodyElement` and `PartUsageBodyElement`.
  - **G5** — `variation`/`abstract` prefix on `perform`/`requirement` members in part usage
    bodies; `RequirementUsage.is_variation`.
  - **G6** — directed `part`/`item`/`attribute` usages inside `perform { }` bodies via new
    `PerformBodyElement::{PartUsage,ItemUsage,AttributeUsage}` variants.
  - **G7** — `event occurrence <name>;` / `then event occurrence …` in `occurrence_usage`.
  - **G8** — named `transition '<name>' first … accept at/when/after … then …;` with time
    triggers on `TransitionAccept`.
  - **G9** — `value :>> name : Type;` in attribute-definition bodies.
  - **G10** — leading multiplicity on `attribute occurs[0..1]: Real;`.
  - **G11** — `port :>> name = value { body }` via `PortUsage.value`.
  - **G12** — payload-first `flow of <name> : Type …` in part usage bodies.
  - **G13** — standalone `first <name>;` initial-node marker in action bodies.
  - **G14** — `loop { }` control node (`LoopStmt`).
  - **G15** — keyword-less `:>> name (= value)? (;|{ … })` in occurrence usage bodies.
  - **G16** — `import` in part usage bodies.
  - **G17** — nested `allocate … to …;` in allocation/occurrence usage bodies.
  - **G18** — `exhibit (state)? <name> :>> <target>;` with optional `state`, pre-body `:>>`,
    and `redefines` preserved on `StateUsage`.
  - **G19** — anonymous `action { }` in part usage bodies.
  - **G20** — anonymous `perform action { }` (optional name on `perform_action_decl`).
  - Surfaced narrower follow-up gaps while closing the above — now tracked as **G21–G30** in
    `PARSER_BACKLOG_ROADMAP.md` §6 (short-name on usages, anonymous occurrence redefines, extra
    `then` succession targets, item members in part usage bodies, keyword-less value bindings,
    occurrence members in attribute bodies, exhibit in occurrence bodies, …).
- Bump `PARSE_AST_VERSION` from `51` to `52` for the `PerformBodyElement` additions and the
  cumulative AST field additions in this pass (`Perform.usage_prefix`/`value`, `PortUsage.value`,
  `LoopStmt`, `RequirementUsage.is_variation`, `OccurrenceBodyElement::StateUsage`, …).
  Regenerated AST snapshot fixtures where needed (`UPDATE_VALIDATION_AST=1 cargo test --test
  validation -- --include-ignored`).
- Added `.cargo/config.toml` with `RUST_MIN_STACK=8388608` so the nesting-limit regression test
  is stable on Windows (deeper parser recursion after this pass exceeded the default 1 MiB stack).

- **`attribute_usage` had no `<shortName>` handling at all**, unlike `attribute_def` --
  `AttributeUsage`/`AttributeDef` share the same BNF `UsageDeclaration`/`DefinitionDeclaration` ->
  `Identification` production (§8.2.2.2, `( '<' ShortName '>' )? ( Name )?`), but only the `def`
  side ever parsed it; `attribute_usage`'s head-dispatch only tried
  `alt((prefix_redefinition_target, name))` for the name/redefines part. Confirmed real usage
  (not speculative) in the OMG Geometry domain library's
  `VehicleGeometryAndCoordinateFrames.sysml`: `attribute <wcf> wheelCoordinateFrame :
  CoordinateFrame;` and `attribute <lbpr> lugBoltPlacementRadius :>> radius default 60 [mm];`
  both failed with `recovered_part_def_body_element` before this fix.
  <br>Since `AttributeUsage`/`PartUsage`/`ItemUsage`/`PortUsage` all reach `Identification` through
  the same shared `UsageDeclaration` production, checked the other three usage kinds and found
  they had the identical gap -- fixed all four together rather than leaving three known-bad.
- Added `short_name: Option<String>` to `AttributeUsage`, `PartUsage`, `ItemUsage`, and
  `PortUsage`, and a shared `short_name_prefix` lexer helper (`src/parser/lex.rs`) factored out of
  `identification`'s existing `( '<' ShortName '>' )?` half, reused by all four usage parsers'
  head-dispatch logic (`attribute::attribute_usage`, `part::usage::part_usage`, `item::item_usage`,
  `port::port_usage`). Each dispatch now re-consumes whitespace/comments after the short name's
  closing `>` before proceeding (a short name leaves fresh un-consumed whitespace there that the
  no-short-name path previously got for free from the mandatory keyword-`ws1`) -- caught by the
  new regression tests below, which initially failed with a `TakeWhile1` parse error until this
  whitespace handling was added.
- Bump `PARSE_AST_VERSION` from `49` to `50` for the four struct field additions. Regenerated AST
  snapshot fixtures (`UPDATE_VALIDATION_AST=1 cargo test --test validation -- --include-ignored`)
  — reviewed the diff: only the new `short_name: None` field appears, in
  `functional_allocation_4a.txt` and `parts_tree_1a.txt` (neither fixture's source uses
  `<shortName>` syntax).
- Added regression tests: `attribute_body_tests` (attribute.rs), `short_name_tests`
  (part/usage.rs), `redefines_tests` (item.rs), and `par_002_widening_tests` (port.rs) --
  covering the plain named form, the form combined with a leading redefines/subsets clause, and
  confirming the no-short-name path is unaffected (`short_name: None`).
  <br>Confirmed against real usage: `VehicleGeometryAndCoordinateFrames.sysml` no longer produces
  any `recovered_part_def_body_element` diagnostics.

- **`perform <path>` (part usage body, no `action` keyword) only accepted a brace body and had
  no `:>>` redefinition clause**, so real Systems Library / OMG spec Annex usage like
  `perform 'provide power';`, `perform providePower.generateTorque;`, and
  `perform providePower.generateTorque :>> generateTorque;` fell through to opaque recovery
  (`recovered_part_usage_body_element`). `perform_body()` already had a way to model a semicolon
  body (`PerformBody::Semicolon`) but `perform_usage()` never offered it as an alternative —
  only `perform_action_decl()` (the `perform action <name>` declaration form) did. `Perform` had
  no `redefines` field at all.
  PARSER_BACKLOG_ROADMAP.md §6, group G1 — part of the audit blocking Spec42 v1.0 (see that
  section for the full 25-file breakdown this PR chips away at).
- Added `redefines: Option<String>` to `Perform` (parsed via the existing `qualified_name`
  parser, same one `perform_action_decl`'s `type_name` already uses); `perform_usage()` now
  accepts an optional `:>>` clause and a `;`-only body, mirroring `perform_action_decl`'s
  existing `alt((tag(";"), perform_body))` pattern.
- Bump `PARSE_AST_VERSION` from `47` to `48` for the `Perform` field addition. Regenerated AST
  snapshot fixtures (`UPDATE_VALIDATION_AST=1 cargo test --test validation -- --include-ignored`)
  — reviewed the diff: only the new `redefines: None`/`Some(..)` field appears, in exactly the
  5 `Perform` nodes `functional_allocation_4a.txt` contains, all `None` (that fixture doesn't use
  the redefine form).
  <br>Confirmed against real usage in the OMG spec Annex validation corpus: of the 4 files
  originally attributed to this gap, `12b-Allocation-1.sysml` is now fully clean;
  `8-Requirements.sysml`, `12b-Allocation.sysml`, and `5-State-based Behavior-2.sysml` each have
  a second, distinct gap that was previously hidden behind this one (`private import` inside a
  part usage body; nested `allocate` inside an allocation usage's own body; `exhibit <name> :>>
  <target>;` redefinition) — not fixed here, tracked as new roadmap items.
- Added regression tests in `src/parser/part/usage.rs::perform_semicolon_and_redefine_tests`
  covering plain/dotted/quoted-name semicolon bodies, the `:>>` redefine clause with both body
  forms, and confirming the `perform action <name>` declaration form is unaffected.
- **`connection <name> : Type[mult];` usage form and `assert constraint` were wired into
  `PartDefBodyElement` but not `PartUsageBodyElement`**, so real OMG spec Annex usage inside a
  `part` usage body — as opposed to a `part def` body — fell through to opaque recovery.
  PARSER_BACKLOG_ROADMAP.md §6, groups G2/G3.
- `connection_usage_member` additionally had **no multiplicity support at all** (in either body
  kind) — `connection trailerHitch : TrailerHitch[0..1];` (`3c-Function-based Behavior-structure
  mod.sysml`) failed on the `[0..1]` regardless of which body it was in. Added
  `ConnectionUsageMember.multiplicity: Option<Node<Multiplicity>>`, parsed via the existing
  `multiplicity_node` parser right after the type.
- `assert_constraint_member` additionally **never parsed a name at all** (in either body kind) —
  `assert constraint engineSelectionRational { }` (`10b-Trade-off Among Alternative
  Configurations.sysml`) isn't just missing from `PartUsageBodyElement`, it was unmodeled in the
  grammar entirely; only the anonymous `assert constraint { }` form worked. Added
  `AssertConstraintMember.name: Option<String>`.
- Added `PartUsageBodyElement::Connection(Node<ConnectionUsageMember>)` and
  `PartUsageBodyElement::AssertConstraint(Node<AssertConstraintMember>)` variants and wired
  `connection_usage_member`/`assert_constraint_member` into `part_usage_body_element`'s dispatch,
  mirroring the existing `PartDefBodyElement` wiring (`connection_def_required` before
  `connection_usage_member`, same "def must be tried before usage" ordering rationale already
  documented for `port`/`flow`/`calc`).
- Bump `PARSE_AST_VERSION` from `48` to `49` (stacks on G1's `47` → `48` bump above). No
  checked-in AST snapshot fixture exercises `connection`/`assert constraint` usage syntax, so no
  snapshot regeneration was needed this time (verified: `cargo test --test validation --
  --include-ignored` passes unchanged).
- Added regression tests: `src/parser/connection.rs` (multiplicity), `src/parser/occurrence_body.rs`
  (`assert_constraint_name_tests`), `src/parser/part/usage.rs::par_002_nested_def_tests` (the new
  `PartUsageBodyElement` variants, plus confirming `connection def` isn't shadowed by the new
  `connection_usage_member` wiring).
  <br>Confirmed against real usage in the OMG spec Annex validation corpus: of the 4 files
  originally attributed to G2/G3, `7a-Variant Configuration - General Concept.sysml` and
  `10b-Trade-off Among Alternative Configurations.sysml` are now fully clean.
  `3c-Function-based Behavior-structure mod-1.sysml`/`-2.sysml` each have a second, distinct gap
  sitting directly behind the one fixed here (anonymous `action { }` and anonymous
  `perform action { }` — the latter's `name` is currently mandatory) — not fixed here, added to
  PARSER_BACKLOG_ROADMAP.md §6 as G19/G20.

- **`parse`/`parse_root` silently accepted invalid SysML v2 input** (misspelled keywords,
  unknown keywords, and other body members the grammar could not match) by returning `Ok` for
  any document that structurally parsed (balanced braces, full input consumed), even when the
  recursive-descent grammar had internally given up on a body member and embedded it as a
  recovery placeholder (`PartDefBodyElement::Error`, `PackageBodyElement::Error`, etc.) instead
  of failing. `parse_with_diagnostics`/`parse_for_editor` already walked the AST for these
  placeholders via `collect_recovery_errors` and correctly reported them; `parse_root` never did
  the same walk, so the two public entry points disagreed on whether a document was valid.
  Reported in [elan8/sysml-v2-parser#2](https://github.com/elan8/sysml-v2-parser/issues/2).
- `parse_root` now calls `collect_recovery_errors` on the successfully-structured AST before
  returning `Ok`, and returns the first embedded diagnostic as an `Err` if any are present. This
  closes the "strict silently accepts invalid SysML" half of the reported divergence.
- Added regression tests in `src/parser/parse.rs::tests` covering the misspelled-keyword,
  unknown-keyword, and typo'd-declaration cases from the issue, confirming `parse_root` and
  `parse_with_diagnostics` now agree on the verdict.
- Updated four existing tests (`tests/parser/recovery.rs`,
  `tests/parser/structure.rs::test_flow_and_allocation_brace_bodies_parse`/
  `test_metadata_def_brace_body_parse`, `tests/recovery_package.rs`) that had asserted `parse()`
  succeeds on inputs containing an embedded recovery placeholder -- i.e. they encoded the bug
  as intended behavior. They now assert `parse()` rejects that input and moved the
  recovers-and-keeps-later-siblings assertions to `parse_with_diagnostics`, which is where that
  guarantee actually belongs.

**Was held pending follow-up; now unblocked and merge-ready.** Running this fix against the full
(normally `#[ignore]`d) validation suite originally showed
`full_validation_suite::test_full_validation_suite` drop from 56/56 to 31/56 (25 of the official
SysML v2 spec Annex example files hit constructs the grammar didn't support yet in specific
nested contexts -- `parse_with_diagnostics` already flagged all 25 as invalid; `parse_root` was
just silently agreeing to disagree). PARSER_BACKLOG_ROADMAP.md §6's follow-up work (G1-G20, see
below and #7/#8/#9) closed all but one of those, and rebasing this branch onto that work closed
the last one directly: `14c-Language Extensions.sysml`'s FMEA library example uses `#<tag>` as a
`PrefixMetadataMember`-style prefix on the following declaration (`#fmeaspec requirement req1 {
... }`, `#prevention connect a to b;`) inside package and item-def bodies, neither of which had
any `#`/`@` annotation or `connect a to b;` support at all before this. Added
`metadata_keyword_prefix` (a new function, not a widening of `metadata_keyword_usage`'s existing
guard -- that guard is relied on elsewhere to correctly fail and fall through to
`hash_annotation`'s opaque-capture form, e.g. `#refinement dependency X to Y;` in action/
requirement bodies) plus `PackageBodyElement`/`AttributeBodyElement::{Connect,
MetadataKeywordUsage}`, wired carefully *after* every more-specific dispatcher (in particular
`connection_def`'s own `DefinitionPrefixOptions::with_hash_annotation()`, so `#derivation
connection { ... }` still becomes one annotated `ConnectionDef`, not a stray tag). Result: a
genuine **56/56** -- no test exception needed. Four existing tests/fixtures that used
`#fmeaspec requirement req1 { }` specifically as a stand-in for "an unsupported annotation" were
updated to a still-genuinely-unsupported example (`#tag : Foo::Bar::Baz weirdstuff;`) now that
the original is valid, fully-supported syntax. All gates green: `cargo test` (361 lib tests),
`cargo test --test validation -- --include-ignored` (25/25, 56/56 files), `cargo fmt`,
`cargo clippy`.

Investigated the issue's other reported direction ("recovery rejects valid SysML") before
deciding not to add speculative grammar branches for it, per this project's practice of verifying
against real usage before scoping a fix:
- The comment-continuation false positive (a `/* ... */` continuation line resembling `Word:
  text` misflagged as a bare declaration) was already fixed independently in
  [#4](https://github.com/elan8/sysml-v2-parser/pull/4) (issue #1), which removed the whole
  non-spec heuristic that caused it.
- `allocate` in a part-definition body already works correctly on both entry points when given
  real (dotted-qualified-name) syntax, confirmed against `AllocationTest.sysml` in the vendored
  library -- the issue's own example (`allocate action step to image;`, space-separated instead
  of dotted) isn't valid `path_expression` syntax, so this isn't a parser gap.
- `foreach ... in ... { }` isn't SysML v2 syntax; the real loop keyword is `for`, which already
  parses correctly in an action-definition body on both entry points.
- `part`/`snapshot` inside an action body have zero confirmed real usage in the vendored SysML v2
  library (`snapshot` is real and well-supported inside `occurrence`/individual-occurrence bodies,
  just not inside `action` bodies) -- backlog candidates, not scoped here without real-usage
  evidence.

- **`ItemUsage` never accepted the anonymous redefinition form** (`item :>> name[multiplicity]?
  (: type)? (= value)? body`) that `PartUsage`/`AttributeUsage` already support, and had no
  `value`/`redefines` fields at all. `item_usage`'s mandatory `name(input)?` call meant any
  `item :>> shape ...` member fell straight through to opaque body-element recovery. Confirmed
  real usage (not speculative) in the OMG Geometry domain library's
  `VehicleGeometryAndCoordinateFrames.sysml` example (`item :>> shape = new Box(4800 [mm], 1840
  [mm], 1350 [mm]);` and `item :>> shape : Cylinder { :>> radius = ...; :>> height = ...; }`),
  discovered while validating that library end-to-end.
- Made `item_usage`'s name `opt(name)` (mirroring `part_usage`/`view_usage`'s pattern) and added
  `redefines: Option<Node<SubsettingRelationship>>` / `value: Option<Node<FeatureValue>>` fields to
  `ItemUsage`. Unlike `part_usage_redefines_only`/`view_usage_redefines_only`'s separate
  `prefix_redefinition_target` branch, no extra dispatch was needed here: the existing
  `feature_usage_header` call already recognizes a leading `:>>` redefines clause together with a
  `: Type` clause via its own `specialization_clauses`, so making the name optional was sufficient.
- Bump `PARSE_AST_VERSION` from `46` to `47` for the `ItemUsage` field changes above.
- Added regression tests in `src/parser/item.rs::redefines_tests` covering both real-usage forms
  (redefines+value, redefines+type+body) plus regression coverage for the pre-existing named form.

## [0.49.0] - 2026-07-30

### Fixed

- **`ViewRenderingUsage`/`RenderingUsage` bodies were opaque -- `render`/`rendering` usages could
  never carry a real nested member, silently discarding everything inside their `{ ... }` block.**
  Both parsers called the shared `interface::connect_body`, which only distinguishes `;` from
  `{...}` and captures zero structure inside the braces. This blocked the Systems Library's
  `asElementTable`/`columnView` mechanism (`Views.sysml`'s `view columnView[0..*] ordered { ... }`
  feature): a model redefining a table column via `view :>> columnView[1] { render
  asTextualNotation; }` inside a `render`/`rendering` binding parsed the redefinition as
  unrecoverable opaque text. Confirmed real usage (not speculative KerML completeness) in
  `sysml-v2-release/sysml/src/training/42. Views/Views Example.sysml` and
  `.../validation/11-View and Viewpoint/11a-View-Viewpoint.sysml`, both using exactly this form.
- New `RenderingUsageBody`/`RenderingUsageBodyElement` AST types give `ViewRenderingUsage.body`/
  `RenderingUsage.body` real structure (`Semicolon | Brace { elements: Vec<...> }`, mirroring
  `RenderingDefBody`'s existing shape) with `Doc` and nested `ViewUsage` variants; unrecognized
  content still falls through to the existing brace-member recovery path as an `Error` node,
  scoped to what's confirmed needed rather than a wider `UsageBody` grammar with no concrete
  real-usage backing. `ConnectBody` itself is untouched -- it's shared by `connect`/`dependency`/
  `requirement`-body call sites that don't need this.
- **`ViewUsage` never accepted the anonymous redefinition form** (`view :>> name[multiplicity]?
  { ... }`) that every other usage kind (`part`, `attribute`, ...) already supports -- confirmed
  against the BNF (`ViewUsage = OccurrenceUsagePrefix 'view' UsageDeclaration? ValuePart?
  ViewBody`, where `UsageDeclaration` legally omits the name). `view_usage`'s existing header
  parsing already extracted `header.redefines` via the shared `parse_feature_usage_header`, but
  silently discarded it -- the same "genuine parsing gap, value parsed then dropped" pattern found
  repeatedly elsewhere in this rollout. Added `redefines`/`multiplicity` fields to `ViewUsage` and
  a `view_usage_redefines_only` parser mirroring `part_usage_redefines_only`'s exact shape
  (`prefix_redefinition_target` + optional `multiplicity_node`, straight to the body -- no `:
  Type` header, matching how `part :>> wheels[4];` works).
- Bump `PARSE_AST_VERSION` from `45` to `46` for the `ViewUsage`/`ViewRenderingUsage`/
  `RenderingUsage` field changes above.
- Added regression tests locking in both real-usage forms (`view :>> columnView[1] { ... }`
  standalone, and nested inside `render`/`rendering` usage bodies) in
  `src/parser/view.rs::column_view_tests`.

## [0.47.1] - 2026-07-26

### Fixed

- Parse `dependency` declarations as definition members inside `part def` bodies, as required by
  the SysML v2 `DefinitionBodyItem -> DefinitionMember -> DefinitionElement` grammar.
- Bump `PARSE_AST_VERSION` from `44` to `45` for the new `PartDefBodyElement::Dependency` variant.
- Restore a clean `cargo clippy --all-targets -- -D warnings` gate on current Rust toolchains
  without changing the established direct-node AST representation.

## [0.47.0] - 2026-07-24

**`PARSE_AST_VERSION` bumped `43` → `44`** — this release changes AST-observable expression
parsing behavior (not just internals), found while writing regression tests for 0.46.1's
stack-safety rewrite.

### Fixed

- **Keyword tokens in expression position silently misparsed identifiers that merely started with
  the same letters.** `not`, `and`, `or`, `xor`, `implies`, `istype`, `hastype`, `as`, and `new`
  were all matched via a bare `tag()` with no word-boundary check (unlike the `meta` postfix
  operator a few lines away, which already used `starts_with_keyword` correctly). Confirmed via
  direct parse probes against the real SysML v2 release library, not just synthetic examples:
  - `notEmpty(x)` (Kernel Semantic Library, `Occurrences.kerml`/`Objects.kerml`, ~16 real call
    sites) parsed as `UnaryOp(Not, Invocation(FeatureRef("Empty"), [x]))` instead of
    `Invocation(FeatureRef("notEmpty"), [x])`.
  - `newSeq` (Kernel Function Library, `SequenceFunctions.kerml`, used as a bound value in
    `binding seq = newSeq;` four times) parsed as `Constructor { type_name: "Seq", args: [] }`
    instead of `FeatureRef("newSeq")`.
  - The same class of bug was latent (not confirmed triggering on real content, but reachable) for
    any expression-position identifier starting with `order`/`as`/`hastype`/etc.
  - None of this produced a parse error or diagnostic — it silently built a wrong-but-valid-looking
    AST, which is why the existing diagnostic-count and node-type-mapping tests never caught it.
    All nine keyword tokens now go through the same `keyword_token` helper (built on the existing
    `starts_with_keyword`), requiring a real word boundary after the keyword.
  - Same root cause also meant `a && b` / `a || b` (the symbolic spellings of `and`/`or`, never
    observed in the release corpus but valid per the grammar) failed to parse at all: an unguarded
    bare `&`/`|` in `additive_op_token` was tried before the two-character symbolic forms and
    greedily consumed the first character. Fixed by trying the symbolic forms first.
- **Expression spans no longer include trailing whitespace/comments past the expression's own
  text.** The previous recursive `postfix()` unconditionally stripped leading whitespace as its
  first statement on every call, including its final, non-matching return — so every expression's
  span silently extended through any run of whitespace/comments up to (but not including) the next
  real token, e.g. `1750 [kg]` immediately followed by ` {` used to span through the trailing
  space. 0.46.1's rewrite initially reproduced this for byte-for-byte compatibility; this release
  intentionally drops it since it was never a deliberate design choice, just an artifact of how the
  old recursive parser happened to be structured.

## [0.46.1] - 2026-07-24

### Fixed

- **Stack overflow on deeply/pathologically nested expressions.** Expression parsing
  (`parenthesized`/`tuple` groups, function/constructor argument lists, `#(index)`) was
  recursive-descent: each nesting level consumed a native call-stack frame with no limit, so input
  like `((((...))))`, `f(g(h(i(...))))`, or a long `.member` / arrow-invocation chain could crash
  the whole process with `STATUS_STACK_OVERFLOW` — not a recoverable parse error, a hard process
  abort. The 0.45.1 `MAX_SYNTAX_NESTING` guard only counted `{`/`}` structural brace nesting and
  never covered this. `src/parser/expr.rs` is now an iterative precedence-climbing (Pratt) parser:
  every construct that used to re-enter the grammar through a real recursive call instead pushes an
  explicit frame onto a heap `Vec` and resumes it later, so nesting depth costs heap growth, not
  call-stack depth. Unlike the brace-nesting guard, there is no arbitrary depth limit here — deeply
  nested expressions now simply parse successfully instead of being rejected or crashing. Covered by
  new regression tests parsing 200,000-deep nested parens, 200,000-deep postfix chains, and
  50,000-deep nested invocations.
- **Stack overflow when dropping a deeply nested `Expression` tree**, a second instance of the same
  class of bug found while writing the regression tests above: even after the parser fix, Rust's
  default recursive `Drop` glue for the `Box<Node<Expression>>` chain built by a deeply nested
  expression could itself overflow the stack when the tree went out of scope, independent of how it
  was parsed. `Expression` now has an explicit iterative `Drop` impl that unwinds arbitrarily deep
  trees via the same heap-`Vec`-instead-of-call-stack technique.
- No public AST or behavior changes: every existing test (including span-exact expectations)
  passes unchanged, and both fixes are internal to how the existing grammar is parsed and torn
  down.
- Nested `action def …` inside action bodies no longer cascades a
  `missing_body_or_semicolon` diagnostic after an incomplete sibling
  (e.g. `bind status = ;`). `action_usage` rejects `action def`, and nested
  definitions are accepted as body members. Same `def` guard on
  `state_usage`.

## [0.46.0] - 2026-07-23

### Added

- **`PartDefBodyElement::ActionUsage` / `StateUsage`** and matching
  `PartUsageBodyElement::ActionUsage` — Systems Library forms such as
  `abstract ref action performedActions: Action[0..*] :> actions, enactedPerformances`
  and `ref state …` now parse as real usage AST instead of
  `OpaqueMember` / recovery.
- **`ActionUsage` / `StateUsage` reference and specialization fields:**
  `is_abstract`, `is_reference`, structured `typing`, `multiplicity`,
  `subsets`, and `redefines`. Feature-style headers preserve `[0..*]` and
  multi-target `:>` clauses.
- **`PortUsage::is_abstract`** and acceptance of a leading `abstract`
  keyword (e.g. `abstract port ownedPorts: Port[0..*] :> …`).
- **`RefDecl::membership`** with visibility capture on
  `part_ref_usage` (`private ref …`). Kinded refs (`ref action` /
  `ref state` / …) are rejected by `part_ref_usage` so dedicated parsers
  own them.
- **Implicit empty `ActionUsage` bodies** when the next token starts a
  sibling statement/succession (Systems Library `LoopAction` style
  without braces).

### Changed

- **`PARSE_AST_VERSION`** bumped `42` → `43`.
- Opaque part-def catch-all no longer claims `action` / `state` / bare
  `port`; those go through dedicated parsers.

## [0.45.1] - 2026-07-23

### Fixed

- Bounded parser stack consumption for adversarially deep models. Package-body dispatch now
  heap-allocates its large transient `Node<PackageBodyElement>` result without changing the public
  AST or serialized shape, and inputs deeper than 32 structural brace scopes are rejected with
  `nesting_too_deep` before recursive descent. Flat models remain unrestricted by this guard; a
  10,000-member flat package is covered by regression testing.

## [0.45.0] - 2026-07-20

Closes the S42-004 "multi-target typing" gap flagged in Babel42's Systems Modeling API gaps
document, plus a more severe bug found while verifying it against real usage: a `ref` declaration
combining `:>>` redefinition with a `:` typing clause (e.g. Systems Library `Actions.sysml`'s `ref
sentMessage :>> sentTransfer: MessageTransfer, MessageAction { ... }`, used in both `SendAction`
and `AcceptMessageAction`) previously silently discarded the redefines target *and* the entire
typing clause as unparsed text once `:>>` was seen — worse than the "only the first type survives"
framing in the gaps doc, since neither type nor the redefines target survived at all. Confirmed via
direct parse probes, not just static reading: `PartUsage`/`ref` declarations already parsed a
comma-separated multi-target clause structurally in most call sites (`typings`/`optional_typings`
already returned every target), but every consumer collapsed it to a single joined `type_name:
String` before it ever reached the AST — the fix reuses the same `TypingRelationship`/
`typing_node` machinery `AttributeUsage`/`AttributeDef` already ship, rather than inventing a new
mechanism.

Two related gaps found during the real-usage audit are deliberately left open, out of scope for
this release: `ref` inside a `part def`/`port def` body has no visibility/direction-prefix support
at all, and `ref <kind> name : Type[mult] :> subsets` forms (`ref action`, `ref state`, `ref port`,
etc., real usage confirmed in the Systems Library, e.g. `Parts.sysml`'s `abstract ref action
performedActions: Action[0..*] :> actions, enactedPerformances`) fall through to an opaque,
inert `Other(...)` catch-all rather than a real AST node. Both are real, but distinct bug classes
from multi-target typing, and the second is comparable in size to the P5+ "unified definition/
usage/specialization grammar layer" item this repo's own backlog already flags as "do not
big-bang rewrite."

### Added

- **`PartUsage::typing`, `RefDecl::typing`** (`Option<Node<TypingRelationship>>`): structured,
  multi-target-capable typing clause alongside the existing joined-display-string `type_name`,
  mirroring `AttributeUsage.typing`/`AttributeDef.typing`. Populated in every `PartUsage`
  constructor (`part_usage_named`, `anonymous_part_usage`) from the same `typings`/
  `optional_typings` result already computed for `type_name` — no new grammar. `RefDecl`'s ad hoc
  single-target call sites (`connection.rs`, `interface.rs`, `part_ref_usage`, `state.rs`) wrap
  their existing single `qualified_name` result into the same field via a new
  `usage::single_target_typing` helper, for API consistency, with no parsing-behavior change.
- **`RefDecl::redefines`** (`Option<Node<SubsettingRelationship>>`): the `:>>` target, via a new
  `usage::single_target_redefines` helper. Only `action_ref_decl` (action/action-usage bodies)
  actually parses this clause today — the other `ref`-declaration sites have no confirmed real
  `:>>` usage and are left as `None`, unchanged from before.
- `usage::typing_fields_from_result`: shared helper turning a `typings`/`optional_typings` result
  into the `(type_ref_span, type_name, typing)` triple, factoring out the pattern
  `part_usage_named`/`anonymous_part_usage`/`action_ref_decl` each need.

### Fixed

- **`action_ref_decl`** (`ref` inside action def/usage bodies): now parses an optional `:>>`
  redefines clause followed by an optional `:` typing clause (multi-target aware) as two separate,
  sequential clauses, instead of one `:>>`/`:>`/`:` token deciding a single all-or-nothing branch.
  Previously, `:>>` matched but never consumed a target, and everything up to the body/terminator
  — including a following `: Type1, Type2` clause — was silently skipped as opaque text via
  `take_until_terminator`. This is live in the gated Systems Library today (`Actions.sysml`'s
  `SendAction.sentMessage`/`AcceptMessageAction.acceptedMessage`), confirmed via
  `cargo test --test validation -- --include-ignored` (`full_library_suite::
  test_full_library_strict_no_diagnostics` and the rest of the 25-test gate still pass — the bug
  produced no diagnostic before or after, only silent data loss, so the strict gate alone would
  not have caught either the bug or the fix).

### Changed

- **`PARSE_AST_VERSION`** bumped `41` → `42`: `PartUsage` and `RefDecl` each gained new fields.
- Regenerated AST snapshot fixtures (`UPDATE_VALIDATION_AST=1 cargo test --test validation --
  --include-ignored`) and updated the hand-authored `parts_interconnection_2a.rs` expected-AST
  fixture for the new `PartUsage`/`RefDecl` fields.

## [0.44.0] - 2026-07-20

Closes the `Intersecting` gap flagged in Babel42's Systems Modeling API gaps document
(S42-002/S42-008, 2026-07-19 audit) as a "confirmed genuine parser gap": `intersects
<target>` (KerML `Intersecting`, e.g. `attribute reading : Weight intersects a, b;`) was
grammatically recognized (`usage::skip_intersects_clause`) but the parsed targets were
discarded entirely via `opt(...)` with no captured value — the same bug class already fixed
for `references`/`crosses` (`ReferenceSubsetting`/`CrossSubsetting`). The same audit's other
flagged items were investigated and found out of scope for this repo: `TypeFeaturing`,
`FeatureInverting`, `Unioning`, `Disjoining`, `Differencing`, and `FeatureChaining`-as-a-
relationship-metaclass have zero real usage in the SysML systems library or examples
(`sysml-v2-release/`) and remain backlog, not a release blocker. General (non-port)
Conjugation, also flagged, turned out not to be a parser gap at all: the `~` conjugated-typing
prefix is already parsed generically for every usage kind via `usage::conjugated_qualified_name`
— nothing about it is port-specific in this parser — so that item requires only a Spec42-side
fix (its graph builder only materializes the implicit conjugate/`PortConjugation` edge for
`PortDefinition` today), not a `sysml-v2-parser` change.

### Added

- **`SubsettingKind::Intersects`** (`src/ast/core.rs`) and `parser::usage::intersecting()`,
  mirroring `cross_subsetting`. `intersects` is now a normal `SpecializationClause` alternative
  parsed by `specialization_clauses()` instead of being tokenized and dropped by a separate
  `skip_intersects_clause` (removed).
- **`AttributeUsage::intersects`, `PortUsage::intersects`, `OccurrenceUsage::intersects`**
  (`src/ast/structure.rs`): `Option<Node<SubsettingRelationship>>`, matching the existing
  `subsets`/`redefines`/`references`/`crosses` field set on these three usage kinds — the only
  ones that already carried the full subsetting-family field set before this change.
- Parser tests covering `intersects` in `specialization_clauses()` (single target, multi-target
  comma-separated, and mixed with `subsets`/`crosses` in one clause) and end-to-end coverage on
  `AttributeUsage`, `PortUsage`, and `OccurrenceUsage`.

### Changed

- **`PARSE_AST_VERSION`** bumped `40` → `41`: `AttributeUsage`, `PortUsage`, and `OccurrenceUsage`
  each gained a new field.

## [0.43.0] - 2026-07-19

Closes a flow-payload parsing gap found while auditing Babel42's flow-payload-resolution work
(Spec42 0.44.11) against the OMG SysML v2.0 spec text: `succession flow dataFlow of Payload from
a to b;`'s `of X` clause was parsed by the generic `expression` combinator, which only handles a
bare type reference. Per §8.2.2.16, `of X` denotes a `PayloadFeature` — an optionally-named
`Feature`, typed and/or given a multiplicity — not a plain expression. Confirmed by direct
testing: `of qty : Payload` (a named payload feature, no multiplicity) and `of qty :
Payload[1..3]` (named + multiplicity) both previously failed to parse entirely, dropping the
whole flow statement into an `Error` recovery node.

### Added

- **`ast::PayloadFeature`** (`src/ast/behavior.rs`): `{ name: Option<String>, type_name:
  Option<String>, multiplicity: Option<Node<Multiplicity>> }`, mirroring `ItemUsage`'s shape with
  the name relaxed to optional (a payload feature's `Identification` is itself optional per
  spec). `parser::flow::payload_feature` disambiguates the named form (`name : Type`) from a bare
  type reference by trying the named form first and only committing to it if a typing clause
  genuinely follows the identifier; multiplicity is accepted after either form (leading
  multiplicity, the grammar's third alternative, is out of scope — no observed real-world usage).
  `FlowUsage::payload` changes from `Option<Node<Expression>>` to `Option<Node<PayloadFeature>>`.
- Parser tests in `tests/parser/flow_usage.rs` covering all four payload forms: bare type
  reference, named (no multiplicity), named with multiplicity, and bare qualified type reference,
  plus a regression test confirming a flow with no `of` clause still has `payload: None`.

### Changed

- **`PARSE_AST_VERSION`** bumped `39` → `40`: `FlowUsage::payload`'s type changed.

## [0.42.0] - 2026-07-18

Closes the `expose` classification gap flagged in Babel42's Systems Modeling API gaps document:
`expose_member` already distinguished the four suffix forms (`::*::**` / `::**` / `::*` / plain)
to build the concatenated `target` string, but discarded which one matched instead of retaining
it structurally — `expose` is normatively an Import per its own BNF doc comment
(`MembershipImport = QualifiedName (::**)?`, `NamespaceImport = QualifiedName :: * (::**)?`), the
same distinction `ast::Import` already makes for ordinary `import` statements.

### Added

- **`ast::ExposeMember::is_import_all`/`::is_recursive`** (`src/ast/view.rs`), mirroring
  `Import`'s fields of the same name. `parser::view::expose_member` now records which of the four
  suffix branches matched instead of only concatenating `target`.
- Parser tests in `src/parser/view.rs`'s `expose_diagnostic_tests` module covering all four forms:
  plain (`expose vehicle;`), `::*`, `::**`, and `::*::**`.

### Changed

- **`PARSE_AST_VERSION`** bumped `38` → `39`: `ExposeMember` gained two new fields, so cached
  parses built against 0.41.x schema must be invalidated.

## [0.41.0] - 2026-07-18

Closes the `ConcernDefinition`/`ConcernUsage` gap flagged in Babel42's Systems Modeling API gaps
document: `concern_usage` already parses both the `concern` and `concern def` textual forms, but
discarded which one was used, so downstream consumers could not distinguish a concern definition
from a concern usage the way they already can for `case`/`case def`.

### Added

- **`ast::ConcernUsage::is_definition`** (`src/ast/requirement.rs`): true when the optional `def`
  keyword was present. `parser::requirement::concern_usage` now records the match instead of
  discarding it via `nom::combinator::opt(...)`.
- Parser tests in `src/parser/requirement.rs`'s `membership_tests` module covering the bare
  `concern c1 : ConcernType;` form (`is_definition: false`) and the `concern def ...` form,
  including with a typing target and a body (`is_definition: true`).

### Changed

- **`PARSE_AST_VERSION`** bumped `37` → `38`: `ConcernUsage` gained a new field, so cached parses
  built against 0.40.x schema must be invalidated.

## [0.40.0] - 2026-07-17

Closes the `constraint`/`ConstraintUsage` gap flagged in Babel42's Systems Modeling API gaps
document: `constraint c : C;` had no distinct usage-side AST node and folded into `ConstraintDef`
at parse time.

### Added

- **`ast::ConstraintUsage`** (`src/ast/view.rs`) and its parser, `constraint_usage`
  (`src/parser/constraint.rs`), built on `parser::usage::feature_usage_header` — the same shared
  header parser `allocation_usage`/`flow_usage`/`requirement_usage`/etc. already use for
  typing/subsetting/multiplicity/`nonunique`. New `PackageBodyElement::ConstraintUsage` variant,
  dispatched right after `constraint_def` in `package.rs`'s package-level `alt(...)` (mirroring
  `case_def`/`case_usage`).
- Regression tests in `src/parser/constraint.rs`'s new `constraint_usage_tests` module locking in
  every real `Systems Library`/example-corpus bare-`constraint` shape: simple typed
  (`constraint c : C;`), typed-and-braced, untyped-and-braced, the `constraintChecks` shape
  (`abstract` + typing + trailing multiplicity + `nonunique` + subsetting, from
  `Systems Library/Constraints.sysml`), subsetting-only with multiple targets
  (`assertedConstraintChecks`), leading-multiplicity-then-subsetting with no typing
  (`Requirements.sysml`'s `assumptions[0..*] :> constraintChecks, subperformances`), and
  redefinition with a qualified feature-chain target (`assumptions :>>
  RequirementConstraintCheck::assumptions`).

### Changed

- **`constraint_def` now requires the `def` keyword** (`.def_required()`), the PAR-001
  disambiguation pattern, now that `constraint_usage` exists to catch the bare form instead of
  silently misclassifying it as a definition.
- **`PARSE_AST_VERSION`** bumped `36` → `37`: `constraint`/`ConstraintUsage` classification
  changed, so cached parses built against 0.39.x schema must be invalidated.

### Notes

- **This is not the same change CHANGELOG 0.33.0 reverted.** That earlier attempt made
  `constraint_def` (and `calc_def`/`port_def`) require `def` with **no usage-parser fallback at
  all**, so the standard library's bare, `def`-less namespace-level forms became unparseable —
  breaking the full `SYSML_V2_RELEASE_DIR` validation gate. This change adds the missing
  `constraint_usage` parser first, verified to cover every real bare-`constraint` shape in the
  release corpus (see the regression tests above), before requiring `def` on the definition side.
  `calc_def`/`port_def` remain deliberately `Optional` — this change does not touch them.
- Verified against the full SysML v2 release validation suite (`cargo test --test validation --
  --include-ignored` with `SYSML_V2_RELEASE_DIR` pointing at the fetched
  `Systems-Modeling/SysML-v2-Release` checkout): all 25 tests pass, including
  `full_library_suite::test_full_library_suite`,
  `full_library_suite::test_full_library_strict_no_diagnostics`,
  `full_library_suite::test_systems_library_strict_no_diagnostics`, and
  `full_validation_suite::test_full_validation_suite` — the exact gates the 0.33.0 attempt broke.

## [0.39.0] - 2026-07-16

- **Enumerated values are now spanned AST nodes** — `EnumerationBody::Brace { values }` changed
  from `Vec<String>` to `Vec<Node<EnumeratedValue>>` (new `EnumeratedValue { name: String }`
  struct). `enumerated_value()` (`src/parser/enumeration.rs`) now returns a `Node<EnumeratedValue>`
  whose span covers the value's name, so each enumerated value inside `enum def { ... }` can
  become an addressable Spec42/Babel42 element instead of a bare display string. The optional
  inline `{ ... }` body and `= expr` initializer remain discarded — only name + span are
  retained, matching the existing `enumerated_value` production's scope. Breaking AST change;
  minor bump.

## [0.38.0] - 2026-07-16

- **PAR-007: `connection`/`interface` usage misclassified when typed with an inline `connect`
  clause** — `connection link : Link connect a to b;` and
  `interface iface : IfaceType connect a to b;` were silently accepted by `connection_def`/
  `interface_def` as empty-bodied *definitions*, with the `connect ... to ...` clause discarded
  entirely: the shared plain `: Type` header scan (`specialization::
  parse_optional_definition_header_after_identification`) greedily consumes everything up to
  `;`/`{`, extracts only a leading type name, and silently drops the rest. Fixed narrowly, not by
  reordering dispatch or requiring `def` (an earlier PAR-006b attempt at the latter broke real
  Systems Library parsing and was reverted — see `connection_def`'s doc comment): the header scan
  now also returns the raw swallowed text
  (`specialization::parse_optional_definition_header_with_raw`), and
  `DefinitionPrefixOptions::reject_header_keyword` fails the definition parse when that text
  contains a top-level `connect` keyword, so `connection_def`/`interface_def` correctly leave the
  input for the usage parser instead. Widened `connection_usage_member`
  (`part::body::connection_usage_member`) to parse an inline `connect` clause (binary and n-ary
  `connect (a, b, c, ...)` forms), populating new `ConnectionUsageMember::connect_from`/
  `connect_to`/`connect_extra_ends` fields. Added a package-level `interface_usage` dispatch arm
  (`part::usage::interface_usage`, previously only reachable nested inside a part body) and a new
  `PackageBodyElement::InterfaceUsage` variant, since no package-level interface-usage fallback
  existed at all before this fix. `PARSE_AST_VERSION` is 36.

## [0.37.0] - 2026-07-16

- **AttributeUsage multiplicity** — `AttributeUsage` now retains a structured
  `multiplicity: Option<Node<Multiplicity>>` from `MultiplicityPart` instead of
  discarding the parsed range after reading `ordered`/`nonunique`. `PARSE_AST_VERSION`
  is 35.

## [0.36.0] - 2026-07-16

Closes the gaps-doc "Parser work still required" backlog (typed `FeatureValue`, structured
relationship targets, a modifier-completeness audit, and a first-class `Membership` node), on top
of 0.35.0's PAR-002..006 backlog. `PARSE_AST_VERSION` moved from 15 (at 0.35.0) to 34 across this
backlog's many breaking AST-schema changes — most of it from the `Membership` rollout, which now
covers 51 of 53 member-bearing structs crate-wide (the 2 exclusions are BNF-confirmed: no textual
visibility position exists for them). Along the way this also found and fixed ~12 real parser bugs
beyond each item's original scope: 11 instances of a `*_def` parser that legally accepts a
`private`/`protected`/`public` prefix per the BNF but never parsed one (across `part`, `port`,
`item`, `connection`, the requirement/case family, `action`, `alias`, `flow`, `allocation`,
`state`/`individual`/`interface`, `metadata`/`enum`, `occurrence`/`succession`,
`constraint`/`calc`, and the view family), plus a case where `:=` was mis-parsed as the start of a
`:` typing clause. This release also includes a rustfmt/clippy technical-debt pass (see below) and
folds in two small fixes that had landed on top of the 0.35.0 tag but were never separately
published (`1eec56f` clippy fix, `1dfdcd3` header type-reference-drop fix). See the individual
entries below for what changed and why.

### Technical debt: rustfmt and clippy cleanup

Housekeeping pass after the PAR-002..006 and post-PAR-006 "Parser work still required" backlogs
(neither ran `cargo fmt`/full `cargo clippy` as part of their per-increment gate, only
`cargo test`/`clippy -W clippy::all`'s error count, so warnings and formatting drift accumulated).

- **`cargo fmt`**: 48 files had formatting drift (239 diff hunks); applied mechanically, no
  behavior change.
- **`clippy::large_enum_variant`** (`AttributeBodyElement`, `RequirementDefBodyElement`):
  `AttributeUsage`'s size relative to `Doc`/`Error` is inherent (it carries a `Membership` plus
  relationship nodes) and shared by ~10 other body-element enums crate-wide with the same variant
  shape; boxing it in just these two flagged enums would be an inconsistent partial fix, so both
  are `#[allow(clippy::large_enum_variant)]` with a documented reason instead.
- **`clippy::type_complexity`** (`connection.rs`/`interface.rs`'s `connect_ends`,
  `usage.rs`'s `optional_typings`): fixed properly with named type aliases (`ConnectEnds`,
  `TypingsResult`) rather than suppressed — genuine readability win, no behavior change.
- **`clippy::needless_lifetimes`** (3 test helper functions): auto-fixed via `cargo clippy --fix`.
- **Removed 10 blanket `#![allow(dead_code, unused_imports)]` module attributes**
  (`bnf_surface.rs`, `connection.rs`, `constraint.rs`, `interface.rs`, `part/mod.rs`, `port.rs`,
  `requirement.rs`, `state.rs`, `usecase.rs`, `view.rs`) that were masking real warnings.
  Underneath them: ~15 genuinely unused imports (auto-fixed via `cargo fix --lib`), and 3
  genuinely dead functions removed entirely (`safe_constraint_def_body_element` in
  `constraint.rs`, `constraint_body` in `requirement.rs`, `keyword_use_case_def` in
  `usecase.rs`, and `parse_usage_header` in `definition_header.rs` -- all confirmed unreferenced
  anywhere, including tests, before deleting). `bnf_surface.rs`'s five functions are a deliberate
  exception: each names and exercises one BNF production for grammar-coverage traceability, and is
  called only by that module's own `#[cfg(test)]` block -- restored `#![allow(dead_code)]` there
  with a comment explaining why, rather than deleting genuinely-intentional documentation code.
  `part/mod.rs`'s `part_def`/`part_def_body` re-exports got the same narrow, documented
  `#[allow(unused_imports)]` treatment for the same reason (used only by a `#[cfg(test)]` block in
  `package.rs`, so a plain non-test `cargo build` sees the re-export as unused).

All three gates green throughout: `cargo build --all-targets` (zero warnings), `cargo test`
(100% pass), `cargo clippy --all-targets -- -W clippy::all` (zero warnings, down from 5 lib + 3
test-file warnings), and the full `SYSML_V2_RELEASE_DIR` validation gate (25/25).

### Item 4b closing sweep: scope-boundary list is now empty

Per the task's closing instruction, re-swept the whole `src/ast` tree for every `*Def`/`*Usage`-
shaped struct after all six increments above (and the `VariantUsage`/`ActorUsage`/
`MetadataKeywordUsage` BNF checks) landed. Every `pub struct *Def`/`pub struct *Usage` declaration
lives in exactly four files (`src/ast/behavior.rs`, `src/ast/requirement.rs`,
`src/ast/structure.rs`, `src/ast/view.rs`) -- confirmed by grepping the whole `src/ast` tree, no
other file declares one. Mechanically checked all 53 matching struct declarations for a
`membership:` field:

- **51 of 53 have it**, closing every struct this rollout (across this session and every prior
  session) has touched: `ActionDef`/`ActionUsage`, `FlowDef`/`FlowUsage`,
  `AllocationDef`/`AllocationUsage`, `StateDef`/`StateUsage`, `RequirementDef`/`RequirementUsage`,
  `ItemUsage`, `EnumerationUsage`, `ConcernUsage`, `CaseDef`/`CaseUsage`,
  `AnalysisCaseDef`/`AnalysisCaseUsage`, `VerificationCaseDef`/`VerificationCaseUsage`,
  `UseCaseDef`/`UseCaseUsage`, `ActorUsage`, `PartDef`/`PartUsage`, `AttributeDef`/`AttributeUsage`,
  `ItemDef`, `IndividualDef`, `VariantUsage`, `PortDef`/`PortUsage`, `InterfaceDef`,
  `ConnectionDef`, `MetadataDef`/`MetadataUsage`, `EnumDef`, `OccurrenceDef`/`OccurrenceUsage`,
  `SuccessionUsage`, `AliasDef`, `ConstraintDef`, `CalcDef`/`CalcUsage`, and the full view family
  (`ViewDef`/`ViewUsage`/`ViewpointDef`/`ViewpointUsage`/`RenderingDef`/`RenderingUsage`/
  `ViewRenderingUsage`).
- **2 of 53 deliberately do not**, both already documented with a confirmed (not guessed) grammar
  reason: `ThenUseCaseUsage` (`src/ast/requirement.rs`, wraps an already-`membership`-bearing
  `UseCaseUsage` -- it is the `then use case ...` clause, not a member) and
  `MetadataKeywordUsage` (`src/ast/structure.rs`, the `#keyword` shorthand -- confirmed above its
  BNF production `PrefixMetadataMember : OwningMembership = '#' ownedRelatedElement =
  PrefixMetadataUsage` has no `MemberPrefix`, so no textual position exists for a visibility
  keyword). `Objective` and `ExhibitState` (not `*Def`/`*Usage`-named, so outside this grep's
  matching set, but already individually confirmed and documented as non-members or out-of-scope
  in earlier entries in this file) round out the complete set of deliberate exclusions.

No code changes in this entry -- it is a verification pass confirming the prior six increments'
combined effect actually closed the scope, not a new increment. `cargo build --all-targets`,
`cargo test` (246 lib tests + full integration suite), `cargo clippy -- -W clippy::all` (same 5
pre-existing warnings), and the `SYSML_V2_RELEASE_DIR` validation gate (25/25) all remain green
against the final combined state. `PARSE_AST_VERSION` stays at 34 (last bumped by the
`VariantUsage`/`ActorUsage` entry above).

**Item 4b (the first-class `Membership` node item from the "Parser work still required" gaps doc)
is now complete.** The scope-boundary list every prior entry in this rollout carried forward is
empty except for the small set of struct-level, BNF-confirmed exclusions listed above -- there is
no remaining "not yet covered" struct family.

### Changed: `VariantUsage`/`ActorUsage` join the `Membership` rollout with new grammar-backed kinds; `MetadataKeywordUsage` confirmed out of scope

Resolves the three "unverified against BNF" structs the "Item 4b final sweep" entry flagged
(`VariantUsage`, `MetadataKeywordUsage`, `ActorUsage`) by actually checking each production in
`sysml-v2-release/bnf/SysML-textual-bnf.kebnf`, per the task's "don't invent fields with no grammar
backing" discipline used throughout this whole backlog.

**`VariantUsage`** (`src/ast/structure.rs`, the `variant ...;` member inside a `variation part def`
body) -- confirmed **in scope**: BNF `VariantUsageMember : VariantMembership = MemberPrefix
'variant' ownedVariantUsage = VariantUsageElement` carries its own `MemberPrefix` (visibility-
legal) and is explicitly typed as a distinct `VariantMembership` kind in the grammar itself, not a
plain `FeatureMembership`. Added `membership: Membership` with a **new**
`MembershipKind::VariantMembership` variant (`src/ast/membership.rs`, plus a `Membership::variant`
convenience constructor) rather than reusing `FeatureMembership`, since the BNF names it
distinctly. `variant_usage` (`src/parser/part/usage.rs`) previously accepted no visibility prefix at
all -- confirmed by probing `variation part def P { variant part v1: V1; }` vs. a `private`-prefixed
variant -- now calls `lex::visibility_prefix` at its start, threading the resulting `Membership`
through all five of its internal branches (typed `part`/`attribute`/`item`/`port` forms and the
untyped bare-reference form).

**`ActorUsage`** (`src/ast/requirement.rs`, distinct from the larger `RequirementActorDecl`/
`ActorDecl`) -- confirmed **in scope**: BNF `ActorMember : ActorMembership = MemberPrefix
ownedRelatedElement += ActorUsage` also carries its own `MemberPrefix` and its own distinctly-named
`ActorMembership` kind. Added `membership: Membership` with a **new**
`MembershipKind::ActorMembership` variant (plus `Membership::actor`). `actor_usage`
(`src/parser/usecase.rs`) previously accepted no visibility prefix -- now calls
`lex::visibility_prefix` at its start, ordered before the `actor` keyword.

**`MetadataKeywordUsage`** (`src/ast/structure.rs`, the `#keyword` annotation shorthand) --
confirmed **out of scope, for real grammar reasons**: its production is `PrefixMetadataMember :
OwningMembership = '#' ownedRelatedElement = PrefixMetadataUsage` -- no `MemberPrefix` anywhere in
this production, unlike every other struct touched by this rollout. The `#` sigil is the entire
prefix; there is no legal position for `private`/`protected`/`public` before it anywhere in the
grammar. Left with no `membership` field, matching the PAR-003b `unique`/`readonly`/`variable`
precedent for confirmed (not guessed) grammar exclusions.

No test-fixture struct-literal ripple and no `src/ast/mod.rs` normalize-match ripple -- neither
`VariantUsage` nor `ActorUsage` has hand-built literal construction sites or a dedicated normalize
helper.

`PARSE_AST_VERSION` bumped 33 -> 34. Added five regression tests locking in visibility capture (and
the new `VariantMembership`/`ActorMembership` kinds) across both the typed and untyped
`variant_usage` forms and `actor_usage` (`src/parser/part/usage.rs`, `src/parser/usecase.rs`). Full
`cargo test` (246 lib tests + full integration suite), `cargo clippy -- -W clippy::all` (same 5
pre-existing warnings, zero new), and the full `SYSML_V2_RELEASE_DIR` validation gate (25/25) are
green, with **zero validation snapshot regeneration required** this increment.

This closes the "three unverified-against-BNF structs" item from the "Item 4b final sweep" entry:
two were genuine gaps (now fixed, with new grammar-backed `MembershipKind` variants), one is a
confirmed, documented exclusion.

### Changed: `Membership` rollout extended to the view family (`ViewDef`/`ViewUsage`/`ViewpointDef`/`ViewpointUsage`/`RenderingDef`/`RenderingUsage`/`ViewRenderingUsage`)

Sixth increment of the Item 4b final-sweep follow-up list, and the last of the "same mechanical
shape" batch the sweep entry called out as cheap, high-confidence follow-up work. Landed as one
increment (all seven structs, `src/ast/view.rs`/`src/parser/view.rs`) since they share one file and
one mechanical shape, matching the `RequirementUsage`-family precedent from earlier in this rollout.
Adds `membership: Membership` to `ViewDef`/`ViewpointDef`/`RenderingDef` (`kind:
OwningMembership`) and `ViewUsage`/`ViewpointUsage`/`RenderingUsage`/`ViewRenderingUsage` (`kind:
FeatureMembership`).

Confirmed all seven productions legally carry a visibility prefix before writing any code:
`ViewDefinition`/`ViewpointDefinition`/`RenderingDefinition` are all `OccurrenceDefinitionPrefix`-
backed, `ViewUsage`/`ViewpointUsage`/`RenderingUsage` are all `OccurrenceUsagePrefix`-backed, and
`ViewRenderingMember : ViewRenderingMembership = MemberPrefix 'render' ownedRelatedElement +=
ViewRenderingUsage` explicitly carries its own `MemberPrefix` (the same `render r1 : R1;` member
form nested inside a view/rendering definition body) -- all confirmed against
`SysML-textual-bnf.kebnf`'s Clause 8.2.2.26.

**Found the same genuine parsing gap an eleventh time, across all seven parsers.** None of
`view_def`, `viewpoint_def`, `rendering_def` (`DefinitionPrefixOptions`-routed), or the hand-rolled
`view_usage`, `viewpoint_usage`, `rendering_usage`, `view_rendering_usage` accepted a
`private`/`protected`/`public` prefix before this increment -- confirmed by probing `package P {
private view def V1; }` and `package P { view v: V1 { private render r1 : R1; } }` against the
pre-change parser (both fell through to recovery). Fixed the three `*Def` parsers via
`.with_captured_visibility()`, and the four hand-rolled `*Usage` parsers via a direct
`lex::visibility_prefix` call at each one's start (ordered before the existing `abstract` prefix on
`view_usage`/`viewpoint_usage`/`rendering_usage`; `view_rendering_usage` has no `abstract` prefix of
its own).

No test-fixture struct-literal ripple and no `src/ast/mod.rs` normalize-match ripple -- none of the
seven structs have hand-built literal construction sites or dedicated normalize helpers in this
crate.

`PARSE_AST_VERSION` bumped 32 -> 33. Added eleven regression tests locking in visibility capture and
the no-prefix default across the family (`src/parser/view.rs`). Full `cargo test` (241 lib tests +
full integration suite), `cargo clippy -- -W clippy::all` (same 5 pre-existing warnings, zero new),
and the full `SYSML_V2_RELEASE_DIR` validation gate (25/25) are green, with **zero validation
snapshot regeneration required** this increment.

Updates the Item 4b scope-boundary list: the entire view family is no longer in the "not yet
covered" set. This closes the "same mechanical shape" batch the "Item 4b final sweep" entry called
out; still not covered: `VariantUsage`/`MetadataKeywordUsage`/`ActorUsage` (the three
unverified-against-BNF structs) -- see the next increment.

### Changed: `Membership` rollout continues to `ConstraintDef`/`CalcDef`/`CalcUsage`

Fifth increment of the Item 4b final-sweep follow-up list. Adds `membership: Membership` to
`ConstraintDef` (`src/ast/view.rs`, `kind: OwningMembership`), `CalcDef` (`kind:
OwningMembership`), and `CalcUsage` (`kind: FeatureMembership`).

**Found the same genuine parsing gap a tenth time, but with a twist**: unlike every prior struct in
this rollout, `constraint_def` and `parse_calc_def` (`src/parser/constraint.rs`) already had *some*
visibility handling -- both used `DefinitionPrefixOptions::with_private()`
(`VisibilityPrefix::OptionalPrivate`), which matched a bare `private` prefix but **discarded** it
(never populated `DefinitionPrefixResult::visibility`) and, unlike `Captured`, did not accept
`protected`/`public` at all. Both switched to `.with_captured_visibility()`, a strict superset (adds
`protected`/`public` support and captures the result instead of discarding it) confirmed
behavior-preserving for every previously-accepted input. `calc_usage` (hand-rolled, no
`DefinitionPrefixOptions` involved) had no visibility handling of any kind and now calls
`lex::visibility_prefix` directly at its start, same as every other hand-rolled `*_usage` parser in
this rollout.

**Removed the now-dead `VisibilityPrefix::OptionalPrivate` variant and
`DefinitionPrefixOptions::with_private()`** (`src/parser/definition_prefix.rs`) -- `constraint`/
`calc` were the only two call sites (per that enum variant's own doc comment), and both migrated to
`Captured` above, so keeping the discard-only mode around would have been orphaned dead code (it
was in fact flagging `cargo build`'s `dead_code` lint until removed). The one unit test exercising
`OptionalPrivate`'s `private`-before-`abstract` ordering (`prefix_private_before_abstract_constraint`,
`src/parser/definition_prefix.rs`) was updated to use `.with_captured_visibility()` instead, keeping
the same ordering assertion and additionally asserting the now-captured `visibility` field.

No test-fixture struct-literal ripple and no `src/ast/mod.rs` normalize-match ripple -- none of the
three structs have hand-built literal construction sites or dedicated normalize helpers in this
crate (`ConstraintDef`/`CalcDef`/`CalcUsage` all use the whole-value clone path where they appear in
body enums).

`PARSE_AST_VERSION` bumped 31 -> 32. Added six regression tests locking in visibility capture and
the no-prefix default for `constraint_def`/`calc_def`/`calc_usage` (`src/parser/constraint.rs`).
Full `cargo test` (231 lib tests + full integration suite), `cargo clippy -- -W clippy::all` (same 5
pre-existing warnings, zero new -- the `OptionalPrivate`/`with_private` dead-code warnings introduced
by this increment's own migration were fixed by removing the dead code rather than suppressed), and
the full `SYSML_V2_RELEASE_DIR` validation gate (25/25) are green, with **zero validation snapshot
regeneration required** this increment.

Updates the Item 4b scope-boundary list: `ConstraintDef`/`CalcDef`/`CalcUsage` are no longer in the
"not yet covered" set. Still not covered: the view family and the three unverified-against-BNF
structs -- same reasoning as the "Item 4b final sweep" entry below.

### Changed: `Membership` rollout continues to `OccurrenceDef`/`OccurrenceUsage`/`SuccessionUsage`

Fourth increment of the Item 4b final-sweep follow-up list. Adds `membership: Membership` to
`OccurrenceDef` (`src/ast/structure.rs`, `kind: OwningMembership`), `OccurrenceUsage` (`kind:
FeatureMembership`), and `SuccessionUsage` (`kind: FeatureMembership`).

**Found the same genuine parsing gap a ninth time.** `occurrence_def` (`src/parser/occurrence.rs`)
never accepted a `private`/`protected`/`public` prefix -- confirmed by probing `package P { private
occurrence def O1; }` against the pre-change parser (fell through to recovery). Fixed via
`.with_captured_visibility()` on its `DefinitionPrefixOptions`.

**`OccurrenceUsage`'s four real member-position entry points** (`src/parser/occurrence_body.rs`)
-- `occurrence_usage`, `individual_usage`, `snapshot_usage`, `timeslice_usage` -- each independently
verified against the BNF first: `OccurrenceUsage`/`IndividualUsage`/`PortionUsage` (the
`snapshot`/`timeslice` production) are all `OccurrenceUsagePrefix`-backed (`BasicUsagePrefix` +
optional `individual`/portion-kind), so all four legally carry a visibility prefix and each now
calls `lex::visibility_prefix` at its own start (threaded through the shared `occurrence_usage_tail`
helper via a new `membership: Membership` parameter, since each entry point captures its own
distinct visibility before delegating).

**`then_timeslice_usage`** (the `then timeslice ...` succession-continuation form) does **not** get
real visibility capture -- confirmed against the BNF this is not a distinct production with its own
`BasicUsagePrefix` the way the other four are (no `ThenTimeSliceUsage`/similar production exists
anywhere in `SysML-textual-bnf.kebnf`; `timeslice`/`snapshot` only appear via the shared
`PortionKind` alternative on `OccurrenceUsagePrefix`). Treated as an ad hoc site, `visibility: None`,
same as this rollout's other no-grammar-backing sites, and documented inline on the function.

**`succession_usage`** (the standalone `succession ... first ... then ...;` form,
`src/parser/occurrence_body.rs`) also gets real visibility capture: BNF `SuccessionAsUsage =
UsagePrefix ('succession' UsageDeclaration)? 'first' ... 'then' ...` is `UsagePrefix`-backed (which
includes visibility), and this parser is invoked directly from `occurrence_body_element`'s real
member-position dispatch, not a payload-sharing helper -- confirmed the same way `allocate_usage`
was confirmed in this session's first increment.

`src/ast/mod.rs`'s dedicated `normalize_occurrence_def` helper needed a one-line
`membership: o.membership.clone()` addition; `OccurrenceUsage`/`SuccessionUsage` have no dedicated
normalize functions and use the whole-value clone path, and neither has hand-built literal
construction sites in the test suite.

`PARSE_AST_VERSION` bumped 30 -> 31. Added nine regression tests locking in visibility capture (and
`then_timeslice_usage`'s deliberate lack of it) across `occurrence_def`, `occurrence_usage`,
`individual_usage`, `snapshot_usage`, `timeslice_usage`, `then_timeslice_usage`, and
`succession_usage` (`src/parser/occurrence.rs`, `src/parser/occurrence_body.rs`). Full `cargo test`
(225 lib tests + full integration suite), `cargo clippy -- -W clippy::all` (same 5 pre-existing
warnings, zero new), and the full `SYSML_V2_RELEASE_DIR` validation gate (25/25) are green, with
**zero validation snapshot regeneration required** this increment.

Updates the Item 4b scope-boundary list: `OccurrenceDef`/`OccurrenceUsage`/`SuccessionUsage` are no
longer in the "not yet covered" set. Still not covered: `ConstraintDef`, `CalcDef`/`CalcUsage`, the
view family, and the three unverified-against-BNF structs -- same reasoning as the "Item 4b final
sweep" entry below.

### Changed: `Membership` rollout continues to `MetadataDef`/`MetadataUsage`/`EnumDef`/`EnumerationUsage`

Third increment of the Item 4b final-sweep follow-up list. Adds `membership: Membership` to
`MetadataDef` (`src/ast/structure.rs`, `kind: OwningMembership`), `MetadataUsage` (`kind:
FeatureMembership`), `EnumDef` (`kind: OwningMembership`), and `EnumerationUsage`
(`src/ast/requirement.rs`, `kind: FeatureMembership`) -- `EnumerationUsage` already had `is_end`
from the earlier modifier-completeness audit item; this adds the still-missing `membership`
alongside it.

**Found the same genuine parsing gap an eighth time, across all four parsers.** None of
`metadata_def` (`src/parser/metadata.rs`), `metadata_usage`, `enum_def` (`src/parser/enumeration.rs`),
or `enum_usage` accepted a `private`/`protected`/`public` prefix before this increment -- confirmed
by probing `package P { private metadata def M1; }` and `package P { private enum def E1; }`
against the pre-change parser (both fell through to recovery). Fixed `metadata_def`/`enum_def` via
`.with_captured_visibility()` on their `DefinitionPrefixOptions`, and `metadata_usage`/`enum_usage`
via a direct `lex::visibility_prefix` call at each one's start. `enum_usage`'s visibility capture is
ordered before its existing `is_end` prefix check (visibility, then `end`, then the `enum` keyword),
matching `attribute_usage`'s established `visibility_prefix` -> `EndUsagePrefix`/`RefPrefix` ordering
from the modifier-completeness audit.

`src/ast/mod.rs`'s dedicated `normalize_metadata_def`/`normalize_enum_def`/
`normalize_enumeration_usage` helpers each needed a one-line `membership: ...clone()` addition (they
null out individual fields rather than whole-value-clone); `MetadataUsage` has no dedicated
normalize function and uses the whole-value clone path. No test-fixture struct-literal ripple --
none of the four structs have hand-built literal construction sites in the test suite.

`PARSE_AST_VERSION` bumped 29 -> 30. Added twelve regression tests locking in visibility capture and
the no-prefix default (`src/parser/metadata.rs`, `src/parser/enumeration.rs`). Full `cargo test`
(215 lib tests + full integration suite), `cargo clippy -- -W clippy::all` (same 5 pre-existing
warnings, zero new), and the full `SYSML_V2_RELEASE_DIR` validation gate (25/25) are green, with
**zero validation snapshot regeneration required** this increment.

Updates the Item 4b scope-boundary list: `MetadataDef`/`MetadataUsage`/`EnumDef`/`EnumerationUsage`
are no longer in the "not yet covered" set. Still not covered:
`OccurrenceDef`/`OccurrenceUsage`/`SuccessionUsage`, `ConstraintDef`, `CalcDef`/`CalcUsage`, the view
family, and the three unverified-against-BNF structs -- same reasoning as the "Item 4b final sweep"
entry below.

### Changed: `Membership` rollout continues to `StateDef`/`StateUsage`/`IndividualDef`/`InterfaceDef`

Second increment of the Item 4b final-sweep follow-up list. Adds `membership: Membership` to
`StateDef` (`src/ast/behavior.rs`, `kind: OwningMembership`), `StateUsage` (`kind:
FeatureMembership`), `IndividualDef` (`src/ast/structure.rs`, `kind: OwningMembership`), and
`InterfaceDef` (`src/ast/structure.rs`, `kind: OwningMembership`) -- `Transition` (also in
`state.rs`) is a distinct control-flow construct with no independent membership form of its own and
was left untouched, matching this rollout's precedent of only wiring the field onto real
member-bearing `*Def`/`*Usage` structs.

**Found the same genuine parsing gap a seventh time, across all four parsers.** None of `state_def`
(`src/parser/state.rs`), `state_usage`, `individual_def` (`src/parser/individual.rs`), or
`parse_interface_def` (`src/parser/interface.rs`, backing both `interface_def` and
`interface_def_required`) accepted a `private`/`protected`/`public` prefix before this increment --
confirmed by probing `package P { private state def S1; }`, `package P { private individual def
I1; }`, and `package P { private interface def I1; }` against the pre-change parser (all fell
through to recovery). Fixed `state_def`/`individual_def`/`parse_interface_def` via
`.with_captured_visibility()` on their `DefinitionPrefixOptions`, and `state_usage` via a direct
`lex::visibility_prefix` call at its start, ordered before the optional `abstract` prefix.

**`ExhibitState`** (`src/ast/structure.rs`, the `exhibit state name ...` shorthand parsed by
`part/body.rs::exhibit_state`) has its own `OccurrenceUsagePrefix`-backed visibility grammar per the
BNF's `ExhibitStateUsage` production, but `ExhibitState` itself has no `membership` field and is not
in this item's scope (it is a distinct struct from `StateUsage`, not one of the eleven families this
sweep covers). Its adapter site in `src/parser/part/usage.rs::exhibit_state_as_state_usage`, which
repackages an already-parsed `ExhibitState` into a `StateUsage` (used where a state usage is
expected but the source wrote the `exhibit state` shorthand instead), therefore has no visibility
data available to thread through and sets `visibility: None`, documented inline as an ad hoc site
per this rollout's established convention -- adding `membership` to `ExhibitState` itself, if ever
needed, is a separate, explicitly out-of-scope follow-up.

One test-fixture struct-literal ripple: `tests/validation/parts_interconnection_2a.rs`'s two
hand-built `InterfaceDef` literals (`interface_def_engine_to_transmission`,
`interface_def_driveshaft`) needed `membership: owning_membership()` added (both source
declarations have no explicit visibility prefix). `src/ast/mod.rs`'s `normalize_interface_def`
needed a one-line `membership: i.membership.clone()` addition; `StateDef`/`StateUsage`/
`IndividualDef` have zero hand-built literal construction sites and their consumers all use the
whole-value `n.value.clone()` path.

`PARSE_AST_VERSION` bumped 28 -> 29. Added twelve regression tests locking in visibility capture
and the no-prefix default (`src/parser/state.rs`, `src/parser/individual.rs`,
`src/parser/interface.rs`). Full `cargo test` (207 lib tests + full integration suite), `cargo
clippy -- -W clippy::all` (same 5 pre-existing warnings, zero new), and the full
`SYSML_V2_RELEASE_DIR` validation gate (25/25) are green, with **zero validation snapshot
regeneration required** this increment.

Updates the Item 4b scope-boundary list: `StateDef`/`StateUsage`/`IndividualDef`/`InterfaceDef` are
no longer in the "not yet covered" set. Still not covered: `MetadataDef`/`MetadataUsage`,
`EnumDef`/`EnumerationUsage`, `OccurrenceDef`/`OccurrenceUsage`/`SuccessionUsage`, `ConstraintDef`,
`CalcDef`/`CalcUsage`, the view family, and the three unverified-against-BNF structs -- same
reasoning as the "Item 4b final sweep" entry below.

### Changed: `Membership` rollout continues to `FlowDef`/`FlowUsage`/`AllocationDef`/`AllocationUsage`

First increment of the Item 4b final-sweep follow-up list (see the "Item 4b final sweep" entry
below for the full remaining scope this continues). Adds `membership: Membership` to `FlowDef`
(`src/ast/behavior.rs`, `kind: OwningMembership`), `FlowUsage` (`kind: FeatureMembership`),
`AllocationDef` (`kind: OwningMembership`), and `AllocationUsage` (`kind: FeatureMembership`).

**Found the same genuine parsing gap a sixth time, across all four parsers.** None of `flow_def`
(`src/parser/flow.rs`), `flow_usage_member`, `allocation_def` (`src/parser/allocation.rs`), or
`allocation_usage` accepted a `private`/`protected`/`public` prefix before this increment --
confirmed by probing `package P { private flow def F1; }` and `package P { private allocation def
A1; }` against the pre-change parser (both fell through to recovery). Fixed `flow_def`/
`allocation_def` via `.with_captured_visibility()` on their `DefinitionPrefixOptions`, and
`flow_usage_member`/`allocation_usage` via a direct `lex::visibility_prefix` call at each one's
start, ordered before the optional `abstract` prefix, matching this rollout's established ordering.

**`allocate_usage`** (`src/parser/allocation.rs`, the bare `allocate a to b;` shorthand form) also
got a `lex::visibility_prefix` call, unlike this rollout's usual "ad hoc site, `visibility: None`"
treatment for shared/no-grammar sites -- confirmed against the BNF first: `AllocationUsage =
OccurrenceUsagePrefix AllocationUsageDeclaration UsageBody` where `AllocationUsageDeclaration :
AllocationUsage = 'allocation' UsageDeclaration ('allocate' ConnectorPart)? | 'allocate'
ConnectorPart` -- both the `allocation ... allocate ...` and bare `allocate ... to ...` forms are
alternatives of the *same* production reachable through the *same* `OccurrenceUsagePrefix`, so the
bare-`allocate` shorthand legally carries a visibility prefix too, and `allocate_usage` is invoked
directly from real member-position call sites (`package.rs`), not a payload-sharing helper with no
visibility grammar of its own -- unlike `AllocationUsage`'s truly ad hoc analogues elsewhere in this
rollout (e.g. `ActionUsage`'s `control_node_payload_stmt` site).

No test-fixture struct-literal ripple (`FlowDef`/`FlowUsage`/`AllocationDef`/`AllocationUsage` have
zero hand-built literal construction sites in the test suite; every consumer in `src/ast/mod.rs`'s
normalize matches uses the whole-value `n.value.clone()` path). `PARSE_AST_VERSION` bumped 27 -> 28.
Added ten regression tests locking in visibility capture and the no-prefix default
(`src/parser/flow.rs`, `src/parser/allocation.rs`), including one covering `allocate_usage`'s
visibility prefix specifically. Regenerated the `function_based_behavior_3a` validation snapshot
(contains `flow`/`message` declarations) whose `Debug` output changed shape from the new field; full
`cargo test` (199 lib tests + full integration suite), `cargo clippy -- -W clippy::all` (same 5
pre-existing warnings, zero new), and the full `SYSML_V2_RELEASE_DIR` validation gate (25/25) are
green.

Updates the Item 4b scope-boundary list: `FlowDef`/`FlowUsage`/`AllocationDef`/`AllocationUsage` are
no longer in the "not yet covered" set. Still not covered: `StateDef`/`StateUsage`, `IndividualDef`,
`InterfaceDef`, `MetadataDef`/`MetadataUsage`, `EnumDef`/`EnumerationUsage`,
`OccurrenceDef`/`OccurrenceUsage`/`SuccessionUsage`, `ConstraintDef`, `CalcDef`/`CalcUsage`, the view
family, and the three unverified-against-BNF structs (`VariantUsage`/`MetadataKeywordUsage`/
`ActorUsage`) -- same reasoning as the "Item 4b final sweep" entry below.

### Changed: `AliasDef`/`Import` join the `Membership` rollout -- closes the two structs deferred since the very first increment

Closes the two struct families every prior entry in this rollout explicitly deferred ("`AliasDef`/
`Import` -- `MembershipKind::Alias`/`MembershipKind::Import` variants exist ... but nothing
constructs them yet"). This is the third and last of this session's three increments (see the two
entries below for `RequirementUsage`-family and `ActionDef`/`ActionUsage`); together they close the
item's confirmed scope and this session runs the final sweep documented below.

**`AliasDef`** gets a non-optional `membership: Membership` field, `kind:
`[`MembershipKind::Alias`]` -- the variant reserved since the rollout's first increment, previously
unconstructed. **Found the same genuine parsing gap a fifth time**: BNF `AliasMember : Membership =
MemberPrefix 'alias' ( '<' memberShortName = NAME '>' )? ( memberName = NAME )? 'for' memberElement
= [QualifiedName] RelationshipBody` (`SysML-textual-bnf.kebnf`) legally permits a
`private`/`protected`/`public` prefix before `alias`, but `alias_def` (`src/parser/alias.rs`) never
parsed one at all before this increment -- confirmed by probing `package P { private alias m for
ISQ::mass; }` against the pre-change parser (fell through to recovery). Fixed by calling the shared
`lex::visibility_prefix` at the very start of `alias_def`, same as every hand-rolled usage parser in
this rollout.

**`Import`** gets `membership: Membership`, `kind:` `[`MembershipKind::Import`]`. **Design
decision, since this struct is different from every other one in this rollout**: `Import` already
had its own `visibility: Option<Visibility>` field from before this item (`src/ast/common.rs`), so
the two options left open by the very first increment's scope-boundary note were: (a) add
`membership: Membership` alongside the old field, keeping both, or (b) replace the old field with
`membership`. This lands **(b), a straight replacement** -- `Import.visibility` already captured
exactly the same information `Membership.visibility` does (an optional
`private`/`protected`/`public` prefix, no separate ownership/kind data worth preserving alongside
it), and grepping the whole crate (`src/`, `tests/`) found no in-crate consumer reading
`Import.visibility` other than its own constructor in `import.rs` -- a dual-field design would only
have added a redundant, confusing field with no compatibility benefit for zero real consumers. The
old field's hand-inlined `opt(alt((tag("public"), tag("private"), tag("protected"))))` visibility
match in `import_` was also replaced by the shared `lex::visibility_prefix` call (for consistency
and span capture), so this is a pure behavior-preserving refactor plus the new wrapper -- `Import`
already had full visibility grammar coverage, so (unlike every other struct in this rollout) there
was no parsing gap to fix here.

Fixed the two downstream `Import` struct-literal construction sites in
`tests/validation/parts_interconnection_2a.rs` (both `public import ...;`) via a new
`public_import_membership()` test helper alongside the fixture's existing
`owning_membership()`/`feature_membership()` helpers. `PARSE_AST_VERSION` bumped 26 -> 27 for the
breaking AST-schema change. Added four regression tests (`alias_def`/`import_` visibility capture
and no-prefix default, `src/parser/alias.rs`, `src/parser/import.rs`). Regenerated the
`parts_tree_1a`, `function_based_behavior_3a`, and `functional_allocation_4a` validation snapshots
(all contain `import` declarations) whose `Debug` output changed shape from the field rename; full
`cargo test`, `cargo clippy -- -W clippy::all` (same 5 pre-existing warnings, zero new), and the
full `SYSML_V2_RELEASE_DIR` validation gate (25/25, including `full_library_suite` which exercises
every `import`/`alias` declaration in the Kernel and Systems libraries) are green.

### Changed: first-class `Membership` node extended to `ActionDef`/`ActionUsage`

Second of this session's three increments. Adds `membership: Membership` to `ActionDef`
(`src/ast/behavior.rs`, `kind: OwningMembership`) and `ActionUsage` (`kind: FeatureMembership`).
**Found the same genuine parsing gap again**: `action_def` (`src/parser/action.rs`) never accepted
a `private`/`protected`/`public` prefix -- confirmed by probing `package P { private action def
Foo; }` pre-change. Fixed via `.with_captured_visibility()` on `action_def`'s
`DefinitionPrefixOptions`, and a direct `lex::visibility_prefix` call at the start of the primary
`action_usage` parser (`src/parser/action.rs`), matching every hand-rolled usage parser in this
rollout.

`ActionUsage` has a second, ad hoc construction site with no visibility grammar of its own:
`control_node_payload_stmt` (`src/parser/payload.rs`), the standalone `accept`/`send`
control-node-statement shorthand (e.g. `accept msg : Type;` as its own statement, not part of an
`action ...` declaration) -- this always sets `visibility: None`, matching this rollout's
established convention for ad hoc, no-visibility-grammar sites (see `AttributeUsage`'s three ad hoc
sites in the item's first increment).

No test-fixture struct-literal ripple (`ActionDef`/`ActionUsage` have zero hand-built literal
construction sites in the test suite); only `src/ast/mod.rs`'s dedicated `normalize_action_def`/
`normalize_action_usage` helpers needed a one-line `membership: ...clone()` addition (they null out
individual fields rather than whole-value-clone). `PARSE_AST_VERSION` bumped 25 -> 26. Added four
regression tests locking in visibility capture and the no-prefix default for `action_def`/
`action_usage` (`src/parser/action.rs`). All three gates green (see the combined gate run noted in
the entry above for the full three-increment session).

### Changed: first-class `Membership` node extended to the `RequirementUsage`-family (`RequirementDef`/`RequirementUsage`/`ConcernUsage`/`CaseDef`/`CaseUsage`/`AnalysisCaseDef`/`AnalysisCaseUsage`/`VerificationCaseDef`/`VerificationCaseUsage`/`UseCaseDef`/`UseCaseUsage`)

First of this session's three increments closing the item's confirmed remaining scope (see the
"still not covered" list at the bottom of the previous `ConnectionDef`/`ConnectionUsageMember`
entry). Landed as one increment rather than several smaller ones because every struct in this list
shares one of two parser helpers (`parse_definition_prefix` for every `*Def`, and either a shared
payload builder or a small hand-rolled `case_like_usage_body`/`use_case_usage_tail` helper for every
`*Usage`), so the mechanical shape is identical across all eleven structs -- splitting further would
not have reduced risk, only increased the number of gate cycles for no benefit. Confirmed the
family's exact membership by reading `src/ast/requirement.rs` in full first, per the task's own
instruction not to guess: it covers `RequirementDef`/`RequirementUsage` (`src/parser/requirement.rs`),
`ConcernUsage` (same file -- no separate `ConcernDef` struct exists in this AST; see below),
`CaseDef`/`CaseUsage`/`AnalysisCaseDef`/`AnalysisCaseUsage`/`VerificationCaseDef`/
`VerificationCaseUsage` (`src/parser/case.rs`), and `UseCaseDef`/`UseCaseUsage`
(`src/parser/usecase.rs`). All eleven get a non-optional `membership: Membership` field, `kind:
OwningMembership` on the five `*Def` structs and `kind: FeatureMembership` on the six `*Usage`
structs.

**Found the same genuine parsing gap a fourth time, across all five `*Def` parsers and four of the
six `*Usage` parsers.** None of `requirement_def`, `case_def`, `analysis_case_def`,
`verification_case_def`, or `use_case_def` accepted a visibility prefix before this increment
(confirmed the same way as every prior entry: `package P { private requirement def Foo; }` and
siblings all fell through to recovery pre-change) -- fixed via `.with_captured_visibility()` on each
one's `DefinitionPrefixOptions`. The hand-rolled `case_usage`/`analysis_case_usage`/
`verification_case_usage`/`use_case_usage`/`concern_usage` had the same gap -- fixed via a direct
`lex::visibility_prefix` call at each one's start, ordered before the optional `abstract` keyword
(matching this rollout's established prefix ordering: visibility, then abstract, then the type
keyword).

Two structural wrinkles specific to this family, both handled the same way this rollout has handled
every ad hoc/shared-payload site so far (real visibility only at the true member-position parser,
`visibility: None` everywhere else):
- `RequirementUsage`'s payload is built by a single shared function
  (`parse_requirement_usage_payload_with_abstract`) called from three places: the member-position
  `requirement_usage` parser (captures real visibility, overrides the payload's `membership` field
  after the call), `verify_requirement`'s `verify requirement ...` form, and `usecase.rs`'s
  `objective { requirement ... }` form (neither of the latter two has visibility grammar of its
  own, so the payload always builds `visibility: None` and those two callers don't override it).
- `CaseUsage` is similarly shared by `AnalysisCaseUsage`/`VerificationCaseUsage`, which each parse
  their own visibility prefix independently (all three keywords are mutually exclusive
  alternatives, not a shared entry point) and pass the resulting `Membership` into
  `case_like_usage_body` as a parameter rather than re-deriving it. `UseCaseUsage` similarly has two
  construction call sites: the true member-position `use_case_usage` (captures real visibility) and
  `use_case_usage_in_body` (only reachable via the `then use case ...` control-flow production in
  `then_use_case_usage`, which has no visibility grammar of its own -- `visibility: None`).
- `ConcernUsage` has no separate `ConcernDef` struct in this AST even though the BNF has a distinct
  `ConcernDefinition` production -- `concern_usage` already parses both the `concern` and `concern
  def` textual forms into the one `ConcernUsage` struct, a pre-existing design predating this item
  and out of scope to change here; it gets `kind: FeatureMembership` regardless of which textual
  form matched.

No test-fixture struct-literal ripple (all eleven structs have zero hand-built literal construction
sites in the test suite; every consumer in `src/ast/mod.rs`'s normalize matches uses the whole-value
`n.value.clone()` path). `PARSE_AST_VERSION` bumped 24 -> 25. Added ten regression tests locking in
visibility capture and the no-prefix default across the family (`src/parser/requirement.rs`,
`src/parser/case.rs`, `src/parser/usecase.rs`), including one confirming the `verify requirement
...` payload site stays `visibility: None`.

**Combined gate run for all three of this session's increments** (`RequirementUsage`-family above,
`ActionDef`/`ActionUsage`, and `AliasDef`/`Import` below): `cargo build --all-targets`, `cargo test`
(189 lib tests + full integration suite, all green), `cargo clippy -- -W clippy::all` (same 5
pre-existing `large_enum_variant`/`type_complexity` warnings as every prior entry, zero new), and
the full `SYSML_V2_RELEASE_DIR` validation gate (25/25, including `full_library_suite`, which
exercises every `requirement`/`case`/`action`/`import`/`alias` declaration in the real Kernel and
Systems libraries) are green.

### Item 4b final sweep: remaining member-bearing struct families not covered by this rollout

Per the task's closing instruction, swept the whole `src/ast` tree for every `*Def`/`*Usage`-shaped
struct and cross-checked which already have a `membership` field after this session's three
increments land. **This rollout (Item 4b) is not fully complete** -- the sweep found substantially
more member-bearing struct families than the task anticipated when framing this session as the
"last increment." The following are explicitly *not* covered, left as a documented follow-up rather
than guessed at or rushed through with reduced gate discipline:

- **Same mechanical shape as every struct this rollout has already covered** (a `*Def` parser
  routed through `parse_definition_prefix`, needing only `.with_captured_visibility()`, plus a
  sibling `*Usage` parser needing a `lex::visibility_prefix` call at its start) -- expected to be
  cheap, high-confidence follow-up increments of the identical shape as every entry in this
  changelog, deferred purely because this session's scope was bounded to the three items the task
  explicitly named plus this sweep, not because of any grammar or design obstacle:
  `FlowDef`/`FlowUsage` (`src/parser/flow.rs`), `AllocationDef`/`AllocationUsage`
  (`src/parser/allocation.rs`), `StateDef`/`StateUsage` (`src/parser/state.rs`), `IndividualDef`
  (`src/parser/individual.rs`), `InterfaceDef` (`src/parser/interface.rs`), `MetadataDef`/
  `MetadataUsage` (`src/parser/metadata.rs`), `EnumDef`/`EnumerationUsage`
  (`src/parser/enumeration.rs` -- `EnumerationUsage` already has `is_end` from the modifier-audit
  item but no `membership`), `OccurrenceDef`/`OccurrenceUsage`/`SuccessionUsage`
  (`src/parser/occurrence.rs`/`occurrence_body.rs`), `ConstraintDef` (`src/parser/constraint.rs`),
  `CalcDef`/`CalcUsage` (`src/parser/constraint.rs`), and the view family `ViewDef`/`ViewUsage`/
  `ViewpointDef`/`ViewpointUsage`/`RenderingDef`/`RenderingUsage`/`ViewRenderingUsage`
  (`src/parser/view.rs`).
- **`VariantUsage`** (`src/ast/structure.rs`) -- a variant member inside a `variation part def`
  body; not yet checked against the BNF for whether `VariantUsageElement` legally carries its own
  `MemberPrefix` distinct from the nested usage it wraps. Left unverified rather than guessed at;
  should be confirmed against the grammar before either wiring it in or excluding it for real
  (grammar-backed) reasons.
- **`MetadataKeywordUsage`** (`src/ast/structure.rs`) -- the `#keyword` annotation shorthand
  attached to an element; likely closer to an annotation than a first-class member, but not
  confirmed against the BNF's `MetadataUsage`/annotation productions in this session. Same
  "unverified, not excluded for a confirmed reason" caveat as `VariantUsage`.
- **`ActorUsage`** (`src/ast/requirement.rs`, `usecase.rs`'s `actor_usage`, distinct from the
  larger `RequirementActorDecl`/`ActorDecl` already in this AST) -- a use-case-body actor
  declaration; same "same mechanical shape, not yet verified against the BNF's `MemberPrefix`
  applicability at this exact production" status as the two above.
- **Confirmed *not* real memberships, no field needed**: `ThenUseCaseUsage` (`src/ast/
  requirement.rs`) wraps an already-`membership`-bearing `UseCaseUsage` inside the `then use case
  ...` control-flow clause -- it is the clause, not a member, and its wrapped `UseCaseUsage` already
  carries `visibility: None` at that position (see the `RequirementUsage`-family entry above).
  `Objective` (`src/ast/requirement.rs`) similarly wraps an already-`membership`-bearing
  `RequirementUsage` and is not itself a member.

This list should be treated as the new scope-boundary state for any future continuation of Item 4b,
replacing the shorter "still not covered" lists in every entry below.

### Fixed: `PARSE_AST_VERSION` was left at 20 instead of 21 after the Part extension

Found while starting the next `Membership` continuation: the "extended to `PartDef`/`PartUsage`"
entry below documents `PARSE_AST_VERSION` bumped 20 -> 21, but the actual constant in `src/lib.rs`
was still 20 (only the prior 19 -> 20 bump for the `AttributeDef`/`AttributeUsage` entry had
actually landed in code) -- a genuine documentation/code drift in this still-uncommitted working
tree, not a re-interpretation of either entry's intent. Corrected `PARSE_AST_VERSION` to 21 to match
what the Part entry always claimed, before bumping further for this session's own work below.

### Changed: first-class `Membership` node extended to `ConnectionDef`/`ConnectionUsageMember`

Continues the `Membership` rollout (see `ItemDef`/`ItemUsage` above) to `ConnectionDef` and
`ConnectionUsageMember` (`src/ast/structure.rs`). Note the plan's original "`ConnectionUsage`" name
does not exist in the current AST -- the struct backing a nested `connection` usage member (inside a
part definition body or at package level) is `ConnectionUsageMember`, confirmed against
`src/ast/structure.rs` before assuming otherwise (its parser is `connection_usage_member`, shared
between `src/parser/part/body.rs`'s part-body dispatch and `src/parser/package.rs`'s package-level
dispatch -- both call sites automatically pick up the new field with no changes needed of their own,
since they only invoke the shared parser function). Same mechanical shape as every prior entry:
non-optional `membership: Membership`, `kind: OwningMembership` on `ConnectionDef`, `kind:
FeatureMembership` on `ConnectionUsageMember`.

**Found the same genuine parsing gap a third time, in both directions again.** Neither
`connection_def`/`connection_def_required` nor `connection_usage_member` accepted a visibility
prefix before this increment -- confirmed the same way as `PortDef`/`ItemDef`: `package P { private
connection def Foo; }` and `package P { part p: T { private connection c: C; } }` both failed to
parse as visibility-prefixed declarations pre-change. Fixed using the same `.with_captured_visibility()`
infrastructure from the `PortDef`/`PortUsage` increment: both `connection_def` (which also sets
`.with_hash_annotation()` for its def-less `#annotation` form) and `connection_def_required` now set
`.with_captured_visibility()` on their `DefinitionPrefixOptions`; `connection_usage_member` (a
bespoke hand-rolled parser, not routed through `parse_definition_prefix`) calls the shared
`lex::visibility_prefix` directly at its start, same as `port_usage`/`item_usage`/`part_usage`.

Unlike `PortDef`/`PortUsage`, and matching `ItemDef`/`ItemUsage`, **no test-fixture struct-literal
ripple was needed** -- `ConnectionDef`/`ConnectionUsageMember` have zero hand-built literal
construction sites anywhere in the test suite (every existing consumer only pattern-matches and
clones the whole value via `n.value.clone()` in `src/ast/mod.rs`'s normalize matches; only the
dedicated `normalize_connection_def` helper needed a one-line `membership: c.membership.clone()`
addition, since `ConnectionUsageMember`-bearing enum variants use the direct-clone path with no
dedicated normalize function). `cargo test` and the full `SYSML_V2_RELEASE_DIR` validation gate
(25/25) both stayed green with **zero validation snapshot regeneration required**.

`PARSE_AST_VERSION` bumped 23 -> 24. Added seven regression tests locking in visibility capture
(`private`/`protected`/`public` and the no-prefix default) for `connection_def` and
`connection_usage_member` (`src/parser/connection.rs`). All three gates green: `cargo test`, `cargo
clippy -- -W clippy::all` (same pre-existing warnings only, zero new), and the full
`SYSML_V2_RELEASE_DIR` validation gate (25/25).

Updates the scope-boundary list: `ConnectionDef`/`ConnectionUsageMember` are no longer in the "not
yet covered" set. Still not covered: `RequirementUsage`-family, `ActionDef`/`ActionUsage`, etc., and
`AliasDef`/`Import` -- same reasoning as before. This closes the three struct families explicitly
scoped for this continuation (`PortDef`/`PortUsage`, `ItemDef`/`ItemUsage`,
`ConnectionDef`/`ConnectionUsageMember`); further struct families remain a documented follow-up per
the same "start narrow, extend as needed" discipline this whole `Membership` rollout has used.

### Changed: first-class `Membership` node extended to `ItemDef`/`ItemUsage`

Continues the `Membership` rollout (see `PortDef`/`PortUsage` above) to `ItemDef`
(`src/ast/structure.rs`) and `ItemUsage` (`src/ast/requirement.rs` -- note the struct lives outside
`structure.rs` despite the "Item" name; its parsers are in `src/parser/item.rs` regardless). Same
mechanical shape as every prior entry in this rollout: non-optional `membership: Membership`, `kind:
OwningMembership` on `ItemDef`, `kind: FeatureMembership` on `ItemUsage`.

**Found the same genuine parsing gap again, in both directions.** Neither `item_def`/
`item_def_required` (routed through `parse_definition_prefix`, previously with no visibility option
set) nor the hand-rolled `item_usage` accepted a visibility prefix before this increment --
confirmed the same way as `PortDef`/`PortUsage`: `package P { private item def Foo; }` and `package P
{ part p: T { private item q: Q; } }` both failed to parse as visibility-prefixed declarations
pre-change. Fixed using the exact infrastructure the `PortDef`/`PortUsage` increment introduced:
`item_def`'s options now set `.with_captured_visibility()` (the `VisibilityPrefix::Captured` mode
added last increment), and `item_usage` calls the shared `lex::visibility_prefix` directly at its
start, same as `port_usage`/`part_usage`. `directed_item_usage` (the `in`/`out`/`inout item` form
used in port definition bodies) needed no separate change -- it delegates to `item_usage` internally
and inherits its `membership` field automatically; visibility must precede the direction keyword,
same ordering constraint as every other prefix-stacked usage parser in this crate.

Unlike the `PortDef`/`PortUsage` increment, **no test-fixture struct-literal ripple** was needed:
`ItemDef`/`ItemUsage` have zero hand-built literal construction sites in
`tests/validation/parts_interconnection_2a.rs` or anywhere else in the test suite (every existing
consumer only pattern-matches and clones the whole value via `n.value.clone()` in `src/ast/mod.rs`'s
normalize matches, which needed no changes since neither struct had spans to null out). `cargo test`
and the full `SYSML_V2_RELEASE_DIR` validation gate (25/25) both stayed green with **zero validation
snapshot regeneration required** -- the Systems/Full Library sources exercised by that gate contain
no `item`/`item def` declaration whose `Debug` output the gate checks byte-for-byte against a stored
snapshot.

`PARSE_AST_VERSION` bumped 22 -> 23. Added six regression tests locking in visibility capture
(`private`/`protected`/`public` and the no-prefix default) for both `item_def` and `item_usage`
(`src/parser/item.rs`). All three gates green: `cargo test`, `cargo clippy -- -W clippy::all` (same
pre-existing warnings only, zero new), and the full `SYSML_V2_RELEASE_DIR` validation gate (25/25).

Updates the scope-boundary list: `ItemDef`/`ItemUsage` are no longer in the "not yet covered" set.
Still not covered: `ConnectionDef`/`ConnectionUsageMember`, `RequirementUsage`-family,
`ActionDef`/`ActionUsage`, etc., and `AliasDef`/`Import` -- same reasoning as before.

### Changed: first-class `Membership` node extended to `PortDef`/`PortUsage`

Continues the `Membership` rollout (see the `PartDef`/`PartUsage` entry below) to `PortDef`/
`PortUsage` (`src/ast/structure.rs`, `src/parser/port.rs`). Same mechanical shape: a non-optional
`membership: Membership` field, `kind: OwningMembership` on `PortDef`, `kind: FeatureMembership` on
`PortUsage`, visibility captured at the point each parser previously matched-and-discarded (in
`PortUsage`'s case, "previously" means "never matched at all" -- see the gap below) a
`private`/`protected`/`public` prefix.

**Found the same genuine parsing gap `PartDef` had, in two places this time.** Neither
`port_def`/`port_def_required` nor `port_usage` accepted a visibility prefix before this increment
-- confirmed by probing `package P { private port def Foo; }` and `package P { part p: T { private
port q: Q; } }` against the pre-change parser: both fell through to recovery instead of parsing as a
visibility-prefixed `PortDef`/`PortUsage`. Fixed as part of wiring `Membership` in, matching the
`PartDef` precedent (BNF `DefinitionMember`/`UsageMember` productions both legally permit a
visibility prefix wherever a `*Def`/`*Usage` is legal).

`port_usage` (a bespoke hand-rolled parser, not routed through `parse_definition_prefix`) now calls
the shared `lex::visibility_prefix` directly at its very start, same as `part_usage`. `port_def`/
`port_def_required` (both routed through the shared `parse_definition_prefix`/
`DefinitionPrefixOptions` helper, unlike `part_def`) needed a different mechanism: the existing
`VisibilityPrefix::OptionalPrivate` mode on `DefinitionPrefixOptions` only matched-and-discarded a
bare `private` (kept for its two pre-existing `constraint`/`calc` call sites, unchanged here), so a
new `VisibilityPrefix::Captured` mode plus `DefinitionPrefixOptions::with_captured_visibility()`
builder were added (`src/parser/definition_prefix.rs`) that call `lex::visibility_prefix` and thread
the result through two new `DefinitionPrefixResult` fields (`visibility`, `visibility_span`) --
`None`/zero-span for every other existing call site that doesn't opt in, so this is additive and
does not change behavior for `constraint`/`calc`/`state`/`flow`/etc. `port_def`'s options now set
`.with_captured_visibility()` for both the `def`-optional and `def`-required entry points.

Fixed all 24 downstream `PortDef`/`PortUsage` struct-literal construction sites in
`tests/validation/parts_interconnection_2a.rs` (2 `PortDef`, 22 `PortUsage`, including two nested
inside another `PortUsage`'s body that a mechanical brace-matching pass initially missed and were
fixed by hand). `PARSE_AST_VERSION` bumped 21 -> 22. Added six regression tests locking in
visibility capture (`private`/`protected`/`public` and the no-prefix default) for both `port_def`
and `port_usage` (`src/parser/port.rs`). Regenerated the `functional_allocation_4a` validation
snapshot whose `Debug` output changed shape from the new field. All three gates green: `cargo test`,
`cargo clippy -- -W clippy::all` (same pre-existing warnings only, zero new), and the full
`SYSML_V2_RELEASE_DIR` validation gate (25/25).

Updates the scope-boundary list: `PortDef`/`PortUsage` are no longer in the "not yet covered" set.
Still not covered: `ConnectionDef`/`ConnectionUsageMember`, `ItemDef`/`ItemUsage`,
`RequirementUsage`-family, `ActionDef`/`ActionUsage`, etc., and `AliasDef`/`Import` -- same reasoning
as before.

### Changed: first-class `Membership` node extended to `PartDef`/`PartUsage`

Continues the "first-class `Membership` node" item above from `AttributeDef`/`AttributeUsage` to
`PartDef`/`PartUsage`, following the exact mechanical shape that entry's scope-boundary note
predicted: added the `membership: Membership` field (`OwningMembership` on `PartDef`,
`FeatureMembership` on `PartUsage`), captured via `lex::visibility_prefix` (`src/parser/part/def.rs`,
`src/parser/part/prelude.rs`, `src/parser/part/usage.rs`).

**Found a genuine parsing gap, not just discarded data, while doing this**: unlike
`attribute_def`/`attribute_usage`, `part_def` never parsed a `private`/`protected`/`public` prefix
at all. Confirmed by probing `package P { private part def Foo; }` against the pre-change parser: it
fell through to `ExtendedLibraryDecl` recovery instead of parsing as a visibility-prefixed
`PartDef`. The BNF (`DefinitionMember : OwningMembership = MemberPrefix ownedRelatedElement +=
DefinitionElement`) legally permits a visibility prefix before any definition, including
`PartDefinition`, so this is now fixed as part of wiring `Membership` in, not scoped out.

Fixed all 17 downstream `PartDef`/`PartUsage` struct-literal construction sites in
`tests/validation/parts_interconnection_2a.rs` (a hand-built expected-AST fixture) that needed the
new field. `PARSE_AST_VERSION` bumped 20 → 21. Regenerated the `parts_tree_1a` and
`functional_allocation_4a` validation snapshots for the new field's `Debug` shape. All three gates
green: `cargo test`, `cargo clippy -- -W clippy::all` (zero new warnings), and the full
`SYSML_V2_RELEASE_DIR` validation gate (25/25, including the full/systems library suites).

Updates the previous entry's scope-boundary list: `PartDef`/`PartUsage` are no longer in the "not
yet covered" set. Still not covered: `PortDef`/`PortUsage`, `ConnectionDef`/`ConnectionUsage`,
`ItemDef`/`ItemUsage`, `RequirementUsage`-family, `ActionDef`/`ActionUsage`, etc., and
`AliasDef`/`Import` — same reasoning as before.

### Changed: typed `FeatureValue` on `AttributeDef`/`AttributeUsage`/`PartUsage`/`RefDecl`

Closes gaps-doc item 1 ("Parser work still required" backlog, post-PAR-006). `value:
Option<Node<Expression>>` on these four structs discarded which of the BNF `FeatureValue`
production's five legal forms had actually matched -- bare `= expr` (bind), bare `:= expr`
(assign), `default = expr`, `default := expr`, and bare `default expr` -- even though
`value_part` (`attribute.rs`) and `usage_value_part` (`part/usage.rs`) already syntactically
distinguished all five via `alt()` before calling `expression()` and throwing that information
away. The field is now `Option<Node<FeatureValue>>`, where a new `FeatureValue`
(`src/ast/feature_value.rs`) carries `kind: FeatureValueKind` (`Bind` for `=`, `Assign` for `:=`;
bare `default expr` is `Bind`, matching `=`'s semantics), `is_default: bool` (independent of
`kind` -- `default =`, `default :=`, and bare `default expr` all set this), and the wrapped
`expression: Node<Expression>`.

The two near-duplicate parsers were collapsed into one shared `feature_value_part`
(`src/parser/feature_value.rs`), used by both `attribute.rs` and `part::usage`. A few other call
sites construct a `RefDecl`/`AttributeDef`/`AttributeUsage`'s `value` from a bare expression parsed
by a *different*, `=`-only grammar production (the `subsets target = expr` shorthand's optional
value, and the ad hoc `ref` value parses in `action.rs`/`state.rs`/`part/usage.rs`'s
`part_ref_usage` that predate this shared type): these now go through a new
`wrap_bind_expression` helper that packages the bare expression as a non-`default`
`FeatureValueKind::Bind` `FeatureValue`, since `=` is the only operator those productions ever
accepted.

Fixed a real, previously-undetected parsing bug found while writing this item's regression test:
`usage::optional_typings` treated any `:`-prefixed lookahead as the start of a `: Type` typing
clause unless it was `:>` or `:>>`, but never excluded `:=` -- so `attribute foo := 1;` inside a
usage context (e.g. a part definition body) mis-parsed the `:` of `:=` as a typing colon, failed
to find a valid type name after it, and the whole attribute usage fell through to error recovery.
`optional_typings` now also excludes `:=` from that lookahead, matching its existing `:>`/`:>>`
exclusions.

`PARSE_AST_VERSION` bumped 16 -> 17 for the breaking AST-schema change. Added a regression test
covering all five `FeatureValue` forms across all four in-scope structs (`tests/parser/
structure.rs`), including the `:=`-in-part-def-body fix above. Regenerated the `parts_tree_1a`
validation snapshot whose `Debug` output changed shape from this field-type change; full
`cargo test`, `cargo clippy -- -W clippy::all` (zero new warnings/errors), and the
`SYSML_V2_RELEASE_DIR` validation gate (including full/systems library suites) are green.

### Changed: structured relationship targets on `TypingRelationship`/`SubsettingRelationship`

Closes gaps-doc item 2 ("Parser work still required" backlog, post-PAR-006).
`TypingRelationship::target` and `SubsettingRelationship::target` were plain `String`s built by
two lossy joins: `.`-dotted feature-chain segments and `::`-qualified namespace/type segments were
joined into one string with no way to tell them apart afterward (`specialization_target` in
`usage.rs`), and comma-separated multi-target clauses (e.g. `:> Base, Other`) were joined with
`", "`, losing the fact there were multiple distinct targets (`specialization_targets`). Both
fields are now `Vec<Node<RelationshipTarget>>`, where a new `RelationshipTarget` (`src/ast/
relationship_target.rs`) holds an ordered `Vec<RelationshipTargetSegment>`, each segment carrying
its own `name` and an `Option<SegmentSeparator>` (`ColonColon` or `Dot`) recording how it joins to
the previous segment. `first_target()`/`target_display()` convenience methods on both relationship
structs keep the overwhelmingly common single-target case simple for existing callers that only
need the display string.

This is a new, narrow type deliberately kept separate from the existing `FeatureChain`
(`src/ast/feature_chain.rs`, wired into `Expression::FeatureChainRef` for expression-level dot-chain
parsing) -- `FeatureChain` intentionally excludes `::`-qualification and other expression-postfix
concerns, and widening it would have pulled relationship-target parsing into `expr.rs`'s complexity
for no benefit.

Parser changes: `usage.rs`'s `specialization_target`/`specialization_targets` (subsetting-family
`:>`/`:>>`/`::>`/`=>` targets, dot-chains allowed), `typings`/`conjugated_qualified_name` (`:`/
`defined by` targets, no dot-chains), and `specialization.rs`'s
`parse_optional_definition_specialization`/`typing_target_from_header`/
`specializes_from_header_text` (definition-level `:>`/`specializes` and the package-level bare
`: Type` header fallback from the entry above) all now build `RelationshipTarget` segments via a
new `lex::qualified_name_segments` instead of losing structure through `qualified_name`'s joined
`String`. Every `TypingRelationship`/`SubsettingRelationship` construction site from the prior
PAR-004 work (`attribute.rs`, `part/body.rs`) was updated to match. Plain `type_name: String`
display fields on usage structs (e.g. `PartUsage.type_name`, unrelated to these two relationship
types) keep their prior joined-string behavior via a new `targets_display_string` helper, so this
change is AST-shape-only for those fields -- no parsing behavior changed for them.
`PARSE_AST_VERSION` bumped 15 -> 16 for the breaking AST-schema change.

Added regression tests locking in that a multi-target `:>` clause stays as two distinct
`RelationshipTarget`s rather than one joined string, and that `Vehicle::mass.value`'s `::` and `.`
joins stay distinguishable in the segment list (`src/parser/usage.rs`). Regenerated the two
`sysml-v2-release` validation snapshots (`parts_tree_1a`, `functional_allocation_4a`) whose
`Debug` output changed shape from this field-type change; both were confirmed to still parse
correctly against the real Systems Library and Full Library gates
(`SYSML_V2_RELEASE_DIR=... cargo test --test validation -- --include-ignored`).

### Fixed: package-level bare `: Type` header silently dropped the type reference

Found while scoping PAR-002 work (flagged there as out of scope, fixed separately): the shared
`parse_definition_prefix`/`parse_optional_definition_header_after_identification` header parsing
used by the package-level, `def`-optional `port`, `item`, `connection`, and `constraint`/`calc`
def parsers (kept `DefKeywordMode::Optional` per `definition_prefix.rs`'s module doc, to accept
the real Systems Library's bare `def`-less namespace-level forms) captured a `:>`/`specializes`
clause when present but otherwise discarded the entire `: Type` header text once
`specializes_from_header_text` found no `:>`, with nothing downstream to catch the loss. A
package-level declaration with only a plain type, e.g. `port p1: MyPortType;`, parsed successfully
but produced `PortDef { specializes: None, .. }` -- the `MyPortType` reference vanished with no
diagnostic.

`specialization.rs::parse_optional_definition_header_after_identification` now falls back to
parsing the bare type as a `Typing`-kind `TypingRelationship` (via a new `typing_target_from_header`
helper reusing the existing `qualified_name` lexer, handling the `~`-conjugated form too) when no
`:>`/`specializes` clause is present, instead of returning `None`. This reuses the existing
`specializes: Option<Node<TypingRelationship>>` field already on `PortDef`/`ItemDef`/
`ConnectionDef`/etc. -- `TypingRelationship::kind` already distinguished `Typing` from
`Subclassification` (PAR-004 item 1) but nothing previously populated a package-level `Typing`
variant through this path, so no AST schema change or `PARSE_AST_VERSION` bump was needed. The
existing combined library-shorthand form (`: Type[mult] nonunique :> Base`) is unaffected: when a
`:>`/`specializes` clause is present it still wins, matching prior (tested) behavior.

### Changed: modifier-completeness audit -- `derived`/`constant`/direction swept onto `PartUsage`/`PortUsage`, `end` added

Closes gaps-doc item 3 ("Parser work still required" backlog, post-PAR-006), a completeness audit
of eight usage-prefix modifier concepts against `sysml-v2-release/bnf/*.kebnf`. Re-verified each
finding against the current codebase (post Items 1/2) before writing any code:

- **`unique`/`readonly`**: confirmed no textual grammar production anywhere in
  `sysml-v2-release/bnf/*.kebnf` (only `nonunique` exists as a real keyword; `readonly` appears
  only in the separate graphical-notation BNF, `SysML-graphical-bnf.kgbnf`, not the textual one).
  Out of scope, matching the PAR-003b precedent of not inventing fields with no textual syntax.
- **`variable`**: re-checked the grammar directly -- no `variable`/`'var'` keyword production
  exists anywhere in `SysML-textual-bnf.kebnf`. KerML's own `EndFeaturePrefix`/`BasicFeaturePrefix`
  productions (`KerML-textual-bnf.kebnf`) do have `isVariable ?= 'var'`/`'const' { isVariable =
  true }`, but this parser targets the SysML textual notation (`SysML-textual-bnf.kebnf`'s
  `RefPrefix` uses `'derived'`/`'constant'` instead, already typed as `is_derived`/`is_constant`),
  not KerML's textual notation. Out of scope; no field added.
- **Reference/composite ownership**: confirmed `RefDecl` already structurally captures this
  distinction -- every body enum that can contain a `ref`-declared feature carries a dedicated
  `Ref(Node<RefDecl>)` (or `RefDecl(Node<RefDecl>)`) variant, populated by a separate `ref`-keyword
  parser path (`part_ref_usage`, `action_ref_decl`, `connection.rs`/`interface.rs`/`state.rs`'s
  `ref_decl`/`state_ref`), coexisting alongside the ordinary `PartUsage`/`ItemUsage`/etc. member
  variants in the same enums. Which variant a member parses into already *is* the reference-vs-
  composite distinction; the BNF's only `isComposite` production is on unrelated action control-node
  keywords (`merge`/`decide`/`join`/`fork`), not feature ownership. Not a gap; no field added.
- **`derived`/`constant`**: were typed as `is_derived`/`is_constant: bool` on `AttributeUsage` only.
  BNF `RefPrefix` (§8.2.2.6.2) is reachable through `OccurrenceUsagePrefix -> BasicUsagePrefix ->
  RefPrefix` for every occurrence-based usage kind, confirmed for `PartUsage`
  (`PartUsage = OccurrenceUsagePrefix 'part' Usage`) and `PortUsage`
  (`PortUsage = OccurrenceUsagePrefix 'port' Usage`) specifically. Added `is_derived`/`is_constant:
  bool` to both, parsed via the same `derived`/`constant` keyword-prefix pattern `AttributeUsage`
  already used (`part/usage.rs::part_usage`, `port.rs::port_usage`). `ItemUsage` already had
  `direction: Option<InOut>` from a prior increment and needed no change.
- **`direction`**: swept `Option<InOut>` onto `PartUsage`/`PortUsage` alongside `derived`/`constant`
  above (same `RefPrefix` production carries all three), reusing the existing
  `attribute::direction_prefix` parser. `AttributeUsage`, `PerformInOutBinding`, `ItemUsage`, and
  the `RequirementUsage`-family already had it from prior increments.
- **`end`** (`isEnd ?= 'end'`): confirmed a genuine gap. BNF `UnextendedUsagePrefix : Usage =
  EndUsagePrefix | BasicUsagePrefix` makes `end` and `RefPrefix`'s `derived`/`constant`/direction
  mutually exclusive alternatives (not combinable), reachable only through the full `UsagePrefix`
  production, used by exactly four usage kinds: `AttributeUsage`, `EnumerationUsage`,
  `BindingConnectorAsUsage`, `SuccessionAsUsage`. Added `is_end: bool` to `AttributeUsage`
  (`structure.rs`) and `EnumerationUsage` (`requirement.rs`), parsed in `attribute_usage` and
  `enum_usage` respectively as a mutually-exclusive alternative to the `derived`/`constant` prefix
  (matching the BNF's `isEnd` XOR `RefPrefix` structure). Left `Bind`/`SuccessionUsage` (the AST
  types for `BindingConnectorAsUsage`/`SuccessionAsUsage`) unchanged: no occurrence of a
  keyword-prefixed `end bind`/`end succession`/anonymous `end`-prefixed succession was found
  anywhere in the Systems Library, Kernel Library, or example sources the validation gate exercises,
  and their prefix-parsing entry points are more structurally invasive to extend; flagged as a
  narrower follow-up if a real model ever needs it, rather than speculatively wired now.
  Separately confirmed this is unrelated to the existing `EndDecl`/`end_decl` construct
  (`connection.rs`/`interface.rs`), which models a different grammar production entirely (a
  named connector-end declaration, `end name : Type;` / `DefaultInterfaceEnd`) that this parser
  already handles.

`PARSE_AST_VERSION` bumped 17 -> 18 for the breaking AST-schema change (new fields on
`PartUsage`/`PortUsage`/`AttributeUsage`/`EnumerationUsage`). Added regression tests covering all
new fields and their defaults (`src/parser/attribute.rs`, `tests/parser/structure.rs`).
Regenerated the two `sysml-v2-release` validation snapshots (`parts_tree_1a`,
`functional_allocation_4a`) whose `Debug` output changed shape from the new fields; full
`cargo test`, `cargo clippy -- -W clippy::all` (zero new warnings), and the `SYSML_V2_RELEASE_DIR`
validation gate (including the full/systems library suites) are green.

### Changed: structured `AliasDef.target`

Closes gaps-doc item 4a ("Parser work still required" backlog, post-PAR-006). `AliasDef.target`
(the `for` clause of `alias m for ISQ::mass;`) had the same lossy-textual-target problem item 2
already fixed for `TypingRelationship`/`SubsettingRelationship`: it was a plain `String` built by
`qualified_name`'s `::`-join, with no span and no way to distinguish `::`-qualified segments from
one another once joined. It's now a single `RelationshipTarget`
(`src/ast/relationship_target.rs`, introduced by item 2), holding the same ordered
`Vec<RelationshipTargetSegment>`/`Option<SegmentSeparator>` shape plus its own span. Unlike
`TypingRelationship`/`SubsettingRelationship::target`, this is a bare `RelationshipTarget`, not a
`Vec<Node<RelationshipTarget>>` -- an alias target is always exactly one qualified name per the
BNF's `memberElement = [QualifiedName]` (no comma-separated multi-target concept exists for
`alias ... for`), so the plural wrapper item 2 introduced for `:`/`:>` clauses doesn't apply here.

`src/parser/alias.rs::alias_def` now builds the target via the existing `lex::qualified_name_segments`
(the same segment-building helper item 2's `specialization_target`/`conjugated_qualified_name`
already use) instead of the joined-`String` `qualified_name`, and records the target's own span
around just the qualified-name text (not the whole `alias ... ;` clause).

`PARSE_AST_VERSION` bumped 18 -> 19 for the breaking AST-schema change. Added regression tests
covering both a `::`-qualified alias target (`alias m for ISQ::mass;`) and a bare single-segment
name (`tests/parser/structure.rs`). Regenerated the `function_based_behavior_3a` validation
snapshot (the Systems Library's `alias Torque for ISQ::TorqueValue;` re-export shorthand) whose
`Debug` output changed shape from this field-type change; full `cargo test`,
`cargo clippy -- -W clippy::all` (zero new warnings), and the `SYSML_V2_RELEASE_DIR` validation
gate (including the full/systems library suites) are green.

### Changed: first-class `Membership` node (scoped start, `AttributeDef`/`AttributeUsage` only)

Closes gaps-doc item 4b ("Parser work still required" backlog, post-PAR-006) -- the confirmed-scope
"full architectural treatment" alternative rather than a minimal field patch, but landed as its own
multi-increment sub-effort per the plan, following the same "start narrow, extend as needed"
discipline PAR-004 used for `TypingRelationship`/`SubsettingRelationship`/`ConnectionEnd`/
`InterfaceEnd`. Every def/usage element previously lived as a bare, undifferentiated child in its
owning body's `Vec<Node<XBodyElement>>` -- ownership, visibility, and membership *kind* were not
represented independently of which enum variant the element happened to parse into, and an explicit
`private`/`protected`/`public` prefix (matched by `attribute_def`/`attribute_usage` for years, per
this crate's history with `DefinitionPrefixOptions::with_private()`-style prefix parsing) was
consumed and thrown away rather than captured anywhere.

New types (`src/ast/membership.rs`): `MembershipKind` (`OwningMembership` -- a nested `*Def` that
becomes a new named member of its owning namespace; `FeatureMembership` -- a nested `*Usage` that
contributes a feature to its owning type; `Import` and `Alias`, reserved for the still-unmigrated
`Import`/`AliasDef` struct families noted below, not yet constructed by any parser) and
`Membership { kind, visibility: Option<Visibility>, span }`, reusing the existing `Visibility` enum
(`src/ast/common.rs`). `Membership::owning`/`Membership::feature` are convenience constructors for
the two kinds actually wired up this increment.

**Design decision -- direct field, not a generic wrapper.** Of the two shapes considered (wrap every
body-enum element in a generic `Member<T> { membership, element }`, vs. add a `membership: Membership`
field directly to the shared element structs), this lands the direct-field approach: wrapping every
`Vec<Node<XBodyElement>>` in the crate would have been the single largest mechanical change in this
entire backlog for uniformity most consumers don't need, whereas a direct field lets each struct
family be migrated independently and matches the deliberately incremental style every prior PAR/gaps
item in this backlog has used.

**Scope landed this increment: `AttributeDef` and `AttributeUsage` only** (`structure.rs`) -- chosen
as the highest-traffic starting family, matching Item 1/4a's own working area and PAR-004's precedent
of starting with the field shared by the most structs before extending outward. `AttributeDef` gets a
non-optional `membership: Membership` with `kind: OwningMembership`; `AttributeUsage` gets the same
with `kind: FeatureMembership`. Every one of `AttributeUsage`'s four construction sites in
`attribute.rs` (`attribute_usage` -- the primary parser, now the only one that actually parses a
visibility prefix; `attribute_feature_binding`, `metadata_binding`, and `attribute_usage_shorthand`
-- three ad hoc shapes with no visibility syntax of their own) now build a `Membership`, the latter
three always with `visibility: None` since their grammar productions have no visibility prefix to
capture. `attribute_def`/`attribute_usage`'s inline `alt((tag("private"), tag("protected"),
tag("public")))` visibility-matching (previously duplicated in each function, and in `filter_member`/
`import`/`package.rs`) was consolidated into a new shared `lex::visibility_prefix` helper returning
`(Span, Option<Visibility>)`, used by both.

**Explicitly NOT covered by this increment** -- documented here as the scope boundary for a future
continuation, per this backlog's "deviation noted, scoped explicitly" precedent:
- Every other member-bearing `*Def`/`*Usage` struct family (`PortDef`/
  `PortUsage`, `ConnectionDef`/`ConnectionUsage`, `ItemDef`/`ItemUsage`, `RequirementUsage`-family,
  `ActionDef`/`ActionUsage`, etc.) -- none of these got a `membership` field yet as of this
  increment (`PartDef`/`PartUsage` were extended in the very next entry above). Their body-enum
  parsers largely already have the same discarded `private`/`protected`/`public`-prefix pattern
  `attribute_def`/`attribute_usage` had (many share `DefinitionPrefixOptions`/ad hoc `alt()` prefixes
  the same way), so the mechanical shape of the follow-up is expected to closely mirror this
  increment's `AttributeDef`/`AttributeUsage` work -- add the field, call `lex::visibility_prefix` at
  the same point the def/usage prefix is currently matched-and-discarded, wire the `Membership::owning`/
  `Membership::feature` constructor at each construction site, fix the `src/ast/mod.rs` normalize-match
  ripple, regenerate any validation snapshots whose `Debug` output shape changes.
- `AliasDef`/`Import` -- `MembershipKind::Alias`/`MembershipKind::Import` variants exist (since the
  gaps doc explicitly calls out aliasing as itself a membership form) but nothing constructs them yet;
  `Import` already has its own `visibility: Option<Visibility>` field from before this item, so wiring
  a `Membership` there would need to either replace or wrap that existing field -- left as a follow-up
  design decision rather than guessed at here.
- The generic `Member<T>` wrapper alternative was not built at all (see the design decision above);
  if a future increment finds the per-struct-family approach doesn't scale (e.g. some body-enum
  `Other`/`Error`/`Doc` variant genuinely needs membership info), reconsider then rather than building
  the wrapper speculatively now.
- No new `MembershipKind` variants beyond the four above were added speculatively; per this backlog's
  `unique`/`readonly`/`variable` out-of-scope precedent (CHANGELOG 0.35.0's item 3 entry), further
  kinds (e.g. a dedicated parameter/variant/return-parameter membership) should only be added once a
  real parser site needs to distinguish them.

`PARSE_AST_VERSION` bumped 19 -> 20 for the breaking AST-schema change (new non-optional field on
`AttributeDef`/`AttributeUsage`). Added four regression tests locking in visibility capture (`private`/
`protected`/`public` prefixes and the no-prefix default) and the `OwningMembership`/`FeatureMembership`
kind split for `attribute_def`/`attribute_usage` (`src/parser/attribute.rs`). Regenerated the two
`sysml-v2-release` validation snapshots (`parts_tree_1a`, `function_based_behavior_3a`) whose `Debug`
output changed shape from the new field; full `cargo test`, `cargo clippy -- -W clippy::all` (the same
5 pre-existing `large_enum_variant`/`type_complexity` warnings as before this change, confirmed via a
throwaway `git worktree` checkout of the pre-change commit -- zero *new* warnings), and the
`SYSML_V2_RELEASE_DIR` validation gate (including the full/systems library suites) are green.

## [0.35.0] - 2026-07-15

Closes the gaps-doc PAR-002..006 backlog (definitions/usages in every owning context, typed
declaration modifiers, typed relationship AST nodes, complete expression AST, non-semantic
recovery), on top of 0.34.0's PAR-001 fix. Along the way this also found and fixed 7 real parser
bugs beyond the original scope of each PAR item — 5 def/usage-ambiguity bugs in `flow`, `port`,
`calc`, and `connection` (×2 contexts) parsers that lacked the PAR-001-style `def_required()`
guard, and one case where named invocation arguments (`F(x = a)`, legal SysML v2 syntax) had no
parse path at all and silently dropped the enclosing declaration into recovery. `PARSE_AST_VERSION`
moved from 8 (at 0.34.0) to 15 across this backlog's many breaking AST-schema changes; see the
individual entries below for what changed and why.

### PAR-006b: final disambiguation/recovery audit

Closing audit pass for PAR-006 (make recovery non-semantic), on top of the foundation
`def_required()` documentation landed as PAR-006a. Systematically grepped every `alt(...)`
body-element dispatch site across `src/parser/*.rs` and `src/parser/part/*.rs` for `*_def`/
`*_usage` pairs sharing a keyword, and every `Other`/opaque-capture recovery variant, to confirm
none silently misclassify or silently swallow input.

- **Investigated a possible new PAR-001-class gap, found and documented a real (but different)
  issue**: `package.rs::try_package_body_structure` dispatches `connection_def` then
  `connection_usage_member` at package level, and `connection_def`'s own doc comment claimed
  "nothing else shares the `connection` keyword" at package level -- stale as of PAR-002, which
  added that `connection_usage_member` dispatch. Tried making `connection_def` require `def`
  unless hash-annotated (new `DefKeywordMode::RequiredUnlessAnnotated` in
  `definition_prefix.rs`, since reverted) to close the apparent gap. This broke the
  `SYSML_V2_RELEASE_DIR` gate (`test_systems_library_node_types_no_extended`/
  `test_full_library_node_types_no_extended`): the real Systems Library uses bare, `def`-less
  `connection` declarations with `abstract`/multiplicity/`nonunique`/leading `:>` subsets (e.g.
  `abstract connection connections: Connection[0..*] nonunique :> linkObjects, parts { ... }` in
  `Systems Library/Connections.sysml`) that `connection_usage_member`'s narrower grammar can't
  parse, so they fell all the way through to `ExtendedLibraryDecl` instead of `ConnectionDef` --
  worse than the status quo, not a fix. Root cause: `connection_def`'s generic header parsing
  (`parse_optional_definition_header_after_identification`, a text-scan for `: Type[mult]
  nonunique :> target`) is already a grammar superset of `connection_usage_member` for every
  practical package-level input, so `connection_usage_member`'s package-level dispatch arm is a
  narrow fallback, not a competing classification -- there is no live misclassification bug.
  Documented this finding on `connection.rs::connection_def` and in `definition_prefix.rs`'s
  module doc instead of forcing a guard; same precedent class as the `port`/`calc`/`constraint`
  "tried and reverted" note from CHANGELOG 0.33.0.
- **Confirmed every other `def_required()` site already covers its ambiguous pair**: reviewed
  `action`, `allocation`, `case`/`analysis`/`verification`/use-case, `enum`, `flow`,
  `individual`, `interface`, `item`, `metadata`, `occurrence`, `port` (nested-body via
  `port_def_required`), `requirement`, `state`, and `view`/`viewpoint`/`rendering` dispatch sites
  across `action.rs`, `connection.rs`, `interface.rs`, `part/body.rs`, `part/usage.rs`,
  `package.rs`, `port.rs`, `requirement.rs`, `state.rs`, `usecase.rs`, `view.rs` -- all correctly
  guarded, none missing the pattern documented in `definition_prefix.rs`. `attribute_def`'s
  bespoke `disambiguate_from_usage` parameter (the original PAR-001 mechanism, predates
  `def_required()`) is likewise applied everywhere `attribute_usage` is a dispatch sibling.
- **Confirmed recovery `Other`/opaque-capture variants are a consistent, bounded pattern, not
  silent catch-alls**: `RequirementDefBodyElement::Other`, `UseCaseDefBodyElement::Other`,
  `StateDefBodyElement::Other`, `CalcDefBodyElement::Other`/`ConstraintDefBodyElement::Other`,
  `ViewDefBodyElement::Other`/`ViewBodyElement::Other`, and `PartDefBodyElement::Other` all use
  the same gate: `build_recovery_error_node`'s diagnostic classifier decides whether content
  looks like a genuine syntax error (known diagnostic codes like `missing_member_name`/
  `missing_type_reference`/`unexpected_keyword_in_scope` -> `Error(ParseErrorNode)` with a real
  diagnostic) versus an unrecognized-but-plausible, not-yet-modeled library construct (falls
  through to `Other(preview)`, a bounded text snippet, not an unbounded silent accept). This is
  the same design already reviewed once by PAR-006a (which found and removed the one genuinely
  dead/unreachable `Other` variant, `PortBodyElement::Other`) -- no further dead code or silent
  catch-alls found this pass.
- Added a regression test in `src/parser/connection.rs`
  (`connection_def_accepts_the_bare_abstract_multiplicity_nonunique_subsets_form_that_makes_
  def_required_unsafe`) that locks in the specific real-library shape
  (`abstract connection connections: Connection[0..*] nonunique :> linkObjects, parts { ... }`)
  that made the `def_required_unless_annotated` attempt above unsafe, so a future attempt to
  tighten `connection_def` gets an immediate, specific failure pointing back at this note instead
  of only failing the much slower full-library gate.
- Gate: `cargo test` and `SYSML_V2_RELEASE_DIR` `cargo test --test validation --
  --include-ignored` both green. No functional parser change landed this increment -- the
  `def_required_unless_annotated` fix attempt was reverted once it broke the gate; only the audit
  documentation and the new regression test describing the audited, already-correct behavior were
  kept.

### PAR-005: complete the expression AST

Six items against `src/ast/core.rs`'s `Expression` enum and `src/parser/expr.rs`
(`PARSE_AST_VERSION` bumped 14 -> 15 for the schema change).

- **Item 1 (constructor expressions) + item 3 (feature-chain expressions), landed together**:
  `new Type(...)` is now `Expression::Constructor { type_name: String, args: Vec<Argument> }`
  instead of the synthetic `FeatureRef("new TypeName")` string. `type_name` stays a plain
  qualified-name `String` (consistent with `TypeCheck`/`MetaCast`/`Classification`, which already
  represent qualified type references this way -- a constructor names a *type*, not a dot
  feature-access path). Separately, `path_expression` (`bind`/`connect`/`allocate`/interface-
  connect endpoints) now produces `Expression::FeatureChainRef(FeatureChain)` for genuine
  multi-segment dotted chains (e.g. `engine.fuelCmdPort.flowRate`, `rearAxleAssembly.leftWheel
  .wheelToRoadPort` from `2a-Parts Interconnection.sysml`), adopting the standalone `FeatureChain`
  type PAR-004 item 6 built for exactly this and left unwired. A single segment stays
  `FeatureRef`. The general `expression()` grammar's postfix `.` chaining (ordinary value
  expressions, e.g. inside a calc/constraint body) intentionally still folds into nested
  `MemberAccess` -- it's interleaved with `(...)`/`#(...)`/`::`/`meta`/`->op(...)` postfix
  operators a pure feature chain doesn't carry, so widening it further is out of this item's scope.
  **Bug found and fixed along the way**: `FeatureChain` derived `PartialEq` including its `span`
  field, unlike every other span-bearing AST type in this crate (`Node<T>`, `Multiplicity`,
  `TypingRelationship` all have custom `PartialEq` that ignores span so hand-built expected ASTs
  in tests don't need real source spans). Left as derived, it broke the `2a-Parts Interconnection`
  validation fixture the moment a real span reached the field. Fixed with the same custom-impl
  convention (`src/ast/feature_chain.rs`).
- **Item 2 (collection operators)**: `base->op(...)` (`->collect`, `->select`, `->size()`,
  `->includes()`, etc.) is now `Expression::CollectionOp { op: CollectionOperator, base, args }`
  instead of desugaring into an untyped `MemberAccess` + `Invocation` pair. `CollectionOperator`
  covers the names actually seen in the SysML v2 release tree (`collect`, `select`, `selectOne`,
  `size`, `isEmpty`, `notEmpty`, `includes`, `including`, `excludes`, `excluding`, `excludingAt`,
  `excludingOnce`, `equals`, `forAll`, `exists`, `sum`, `sort`, `filter`, `reduce`) plus `Other
  (String)` so no arrow-invoked name is ever lost or misclassified. A bare `->name` with no
  trailing `(...)` (rare) still falls back to plain `MemberAccess`, matching prior behavior for
  that shape.
- **Item 5 (argument relationships), real bug found and fixed**: `ArgumentList` in the KerML BNF
  (8.2.5.8.3) is `PositionalArgumentList | NamedArgumentList` -- named arguments
  (`NAME '=' ArgumentValue`) are legal, real syntax, not just positional. Confirmed against actual
  Systems Library / example usage (`new RiskLevel(probability = LevelEnum::low)` in
  `RiskMetadata.sysml`; `F(q = 1, p = a)` in `ParameterTest.sysml`; `new IgnitionCmd
  (ignitionOnOff=IgnitionOnOff::on)` in the Simple Vehicle Model). Before this change, `expression()`
  had no way to parse `name = value` inside `(...)` (`=` isn't a binary operator token), so every
  one of these real constructs silently fell into parser recovery for the entire enclosing
  declaration -- a data-loss bug, not just a missing-feature gap. Fixed by giving
  `Expression::Invocation`/`Expression::Constructor`/`Expression::CollectionOp` a shared
  `args: Vec<Argument>` (`Argument { name: Option<String>, value: Node<Expression> }`) and a new
  `argument_list_tail` parser that tries `NAME '='` (rejecting `==`/`===` so equality expressions
  are never misread as a named argument) before falling back to a positional `expression()`.
  Return-parameter relationships (`ReturnRef.return_expression`, `src/ast/requirement.rs`) stay
  scoped to requirement/verification bodies as before -- generalizing a return-parameter concept
  onto every invocation is a materially larger design change, out of this item's scope.
- **Item 4 (metadata access), real grammar gap, not previously covered by `Classification`/
  `MetaCast`**: checked `KerML-textual-bnf.kebnf` before assuming a gap existed (per the PAR-003b
  precedent of not inventing nodes for non-existent productions) and found
  `MetadataAccessExpression = ownedRelationship += ElementReferenceMember '.' 'metadata'` is a
  real, separate production -- distinct from `@Metaclass` (`Classification`, tests classification)
  and `expr meta Metaclass` (`MetaCast`, reflective cast). Added `Expression::MetadataAccess
  (Box<Node<Expression>>)`, parsed as a `.metadata` postfix suffix (literal keyword, not a
  general member name) in `postfix()`.
- **Item 6 (parenthesized marker)**: `Expression::Parenthesized(Box<Node<Expression>>)`. Chose the
  wrapping-variant design over a bool flag threaded onto every variant since the blast radius was
  small and contained (only `parenthesized()` in `expr.rs` produces it, and the two exhaustive
  matches on `Expression` in the whole crate -- `src/ast/mod.rs::normalize_expression_node` and
  `expr.rs` itself -- were the only places needing a new arm). `Expression::Tuple` (multi-element
  parenthesized sequences) doesn't get this wrapper -- it's inherently only expressible with
  parens, so there's nothing extra to mark.
- **All six items ripple-checked against the two exhaustive `Expression` matches in the crate**
  (`src/ast/mod.rs::normalize_expression_node`, `src/parser/expr.rs`'s own tests) -- both are the
  full extent of exhaustive matching on `Expression` outside `expr.rs`; the handful of other
  `Expression::` references across the crate (`action.rs`, `bnf_surface.rs`, `payload.rs`,
  `requirement.rs`) only construct or single-arm-match specific variants and needed no changes.
- New public exports: `Argument`, `CollectionOperator`, `FeatureChain` added to the crate root
  `pub use` list in `src/lib.rs` (previously `FeatureChain` wasn't re-exported at all, since
  PAR-004 built it but deliberately left it unwired).

### PAR-003b (item 1 of 4): typed `ordered`/`nonunique`/`derived`/`constant` on attributes

- **`AttributeDef`/`AttributeUsage` gain typed `ordered: bool` / `nonunique: bool` fields**: the
  `MultiplicityPart` modifiers (BNF §8.2.2.6.6) were previously consumed and thrown away by
  `ignored_feature_modifiers` at all 8 call sites across `src/parser/attribute.rs` (attribute def,
  attribute usage, the `:>>`/`:>` feature-binding shape, and the metadata binding shape). Replaced
  with `feature_modifiers`, which returns a small `FeatureModifiers { ordered, nonunique }` struct
  that callers OR-merge across the (up to three) positions a modifier can legally appear in a
  single declaration, so a later empty match can't silently clear an earlier `true`. Confirmed via
  the validation gate that the Systems Library fixtures actually exercise `ordered` (4 real
  occurrences surfaced in `tests/validation/snapshots/parts_tree_1a.txt` once regenerated).
- **`AttributeUsage` gains typed `is_derived: bool` / `is_constant: bool`**: `attribute_usage` now
  parses the `derived`/`constant` keywords from `RefPrefix` (BNF §8.2.2.6.2) before the `attribute`
  keyword. These are usage-only per the grammar -- `AttributeDefinition` uses `DefinitionPrefix`,
  which has no `derived`/`constant` production, so `AttributeDef` does not get these fields.
- **Scope judgment -- `readonly`, `variable`, `sufficient` are not textual keywords**: checked the
  reserved-word list and every relevant production in `sysml-v2-release/bnf/SysML-textual-bnf.kebnf`
  (and the KerML grammar it builds on). `readonly` never appears as a keyword anywhere in the SysML
  or KerML textual grammars -- `Feature::isReadOnly` is a computed/semantic property, not something
  a `def`/`derived`/`constant`-style prefix keyword expresses syntactically, so there is nothing
  for this parser to capture. `variable` likewise has no SysML textual production (KerML's
  `'var'`/`'const'` pair only appears in an unrelated parameter-declaration context that SysML
  doesn't use). `sufficient` corresponds to KerML's `all` keyword on `ClassifierDeclaration`, but
  `DefinitionDeclaration` in the SysML grammar (`Identification SubclassificationPart?`) never
  includes it -- SysML defs have no textual "sufficient" marker. All three are out of this parser's
  scope; noted here rather than inventing fields with no parseable syntax behind them.

### PAR-003b (items 3-4): effective name and source-range assessment (no code change)

- **"Effective name" is out of parser scope, confirmed by design, not by omission**: KerML defines
  `Element::effectiveName` as the name a feature *without its own declared name* inherits from what
  it redefines -- resolving it requires walking the redefinition/conjugation chain across
  potentially other files/packages, which is exactly the kind of cross-reference resolution this
  crate deliberately does not do (it is a syntax parser producing one file's AST, not a resolver).
  `Identification` (`src/ast/common.rs`) already exposes the two concepts that *are* syntactic --
  `name` and `short_name` -- and that's the full extent of what a single parse can determine.
  Recorded here explicitly per the task's own guidance not to force invented scope.
- **Source ranges for `subsets`/`references`/`crosses` targets: already closed by PAR-004**.
  `SubsettingRelationship` (`src/ast/core.rs`) has carried a real `span` field (covering the
  operator/keyword through the target) since PAR-004 item 2, and `references`/`crosses` on
  `AttributeUsage` are typed as `Option<Node<SubsettingRelationship>>`, so they inherit the same
  span coverage as `subsets`. No further work needed for relationship-target spans.
- **Keyword-only spans (`abstract`, `ordered`, `direction`, etc.) intentionally left as bools**:
  these are captured as plain `bool`/enum fields (e.g. `PartUsage.ordered: bool`,
  `AttributeUsage.direction: Option<InOut>`) with no separate span for the keyword token itself.
  Judgment call: adding a dedicated `Span` next to every single-keyword flag across the AST would
  be a wide, low-value restructuring (these tokens are typically one word, immediately adjacent to
  already-spanned constructs, and no consumer has asked for standalone keyword highlighting) for
  marginal value versus the relationship/target spans that matter for semantic tokens and
  diagnostics. Left as-is; flagging here rather than silently skipping.

### PAR-002 (increment 1 of N): nested `def` kinds in `PartDefBodyElement`

- **Added `StateDef`, `MetadataDef`, `MetadataUsage`, `FlowDef`, `RequirementDef`,
  `OccurrenceDef` to `PartDefBodyElement`** (`src/ast/structure.rs`), wired into
  `part_def_body_element` (`src/parser/part/body.rs`) using the existing standalone
  `state_def`/`metadata_def`/`metadata_usage`/`flow_def`/`requirement_def`/`occurrence_def`
  parsers -- no new parsers needed, these already existed and were already reachable at package
  level (`PackageBodyElement` already had all six), just not nested inside a part definition body.
  Before this, `state def`, `metadata def`, `flow def`, `requirement def`, and `occurrence def`
  written inside a `part def { ... }` body could only be reached indirectly (`exhibit state` for
  state, or not at all for the others) or fell through to `Other`/`Error`.
- **`def_required()` reuse, no new guard needed**: `state_def`, `requirement_def`, and
  `occurrence_def` already call `DefinitionPrefixOptions::def_required()` internally (per
  PAR-006a), so wiring them in required no extra disambiguation work -- a bare (`def`-less)
  declaration always still falls through to the sibling usage arm.
- **Found and fixed a real ambiguity bug while wiring `flow_def`**: `flow_usage_member` (used for
  bare `FlowUsage` dispatch) has no guard against the `def` keyword and was misparsing `flow def
  DataFlow;` as `FlowUsage { name: "def" }` when tried first. Fixed by reordering `flow_def` ahead
  of `flow_usage_member` in `part_def_body_element`'s `alt`, matching the order package-level
  dispatch (`try_package_body_structure` in `src/parser/package.rs`) already used. Caught by the
  new `part_def_body_accepts_nested_flow_def` test before it could regress silently -- exactly the
  PAR-001-class bug this backlog's process discipline is meant to catch.
- **PAR-002 acceptance-criterion tests added**: `state_def_is_same_variant_kind_at_package_level_and_nested_in_part`
  and `requirement_def_is_same_variant_kind_at_package_level_and_nested_in_part`
  (`src/parser/part/body.rs`) parse the same snippet at package level and nested in a part body,
  asserting the same AST variant kind both times.
- **Scope remaining for `PartDefBodyElement`** (not done in this increment, see gaps doc /
  PAR-002 task description): `ConnectionDef` (only `ConnectionUsageMember`, a usage shape, exists
  today), `AllocationDef`/`AllocationUsage`, `ViewDef`/`ViewUsage`, `ViewpointDef`/`ViewpointUsage`,
  `RenderingDef`/`RenderingUsage`, `CaseDef`/`CaseUsage`, `UseCaseDef`/`UseCaseUsage`,
  `AnalysisCaseDef`/`AnalysisCaseUsage`, `VerificationCaseDef`/`VerificationCaseUsage`, `PortDef`
  (has `PortUsage` only -- needs a new `port_def_required()` variant since the standalone `port_def`
  deliberately keeps `def` optional, see its own doc comment, so it cannot be wired in as-is without
  reintroducing a PAR-001-class ambiguity against `port_usage`), `CalcDef` (same issue: `calc_def`
  deliberately keeps `def` optional at namespace level, needs a `calc_def_required()` variant before
  it can be safely nested). `PartUsageBodyElement` (zero Def-kind variants), `PortDefBodyElement`/
  `PortBodyElement`, `InterfaceDefBodyElement`/`ConnectionDefBodyElement` are still unaddressed.
  `PackageBodyElement`'s own gaps (standalone `AttributeUsage`/`ItemUsage`/`PortUsage`,
  `ConnectionUsage`, `RefDecl` usage, `EnumerationUsage`) are also still unaddressed.

### PAR-002 (increment 2 of N): remaining `PartDefBodyElement` def/usage pairs

- **Added `ConnectionDef`, `PortDef`, `CalcDef`, `AllocationDef`/`AllocationUsage`,
  `ViewDef`/`ViewUsage`, `ViewpointDef`/`ViewpointUsage`, `RenderingDef`/`RenderingUsage`,
  `CaseDef`/`CaseUsage`, `UseCaseDef`/`UseCaseUsage`, `AnalysisCaseDef`/`AnalysisCaseUsage`,
  `VerificationCaseDef`/`VerificationCaseUsage` to `PartDefBodyElement`** (`src/ast/structure.rs`),
  wired into `part_def_body_element` (`src/parser/part/body.rs`).
- **Built three new `_required()` def parsers before wiring, per the flagged risk**:
  `port_def_required` (`src/parser/port.rs`), `calc_def_required` (`src/parser/constraint.rs`),
  and `connection_def_required` (`src/parser/connection.rs`), each a thin wrapper around a shared
  `parse_*_def(input, options)` helper that the existing `port_def`/`calc_def`/`connection_def`
  (still `def`-optional, unchanged, still used at package level) now also delegate to. Mirrors
  `interface_def_required`/`item_def_required` in shape. `connection_def_required` intentionally
  does not support the hash-annotation def-less form that `connection_def` does -- nothing in the
  nested-part-body grammar needs that combination today.
- **Found and fixed three more real ambiguity bugs of the same class as `flow_def` in increment
  1**, each caught by a `_not_misparsed_as_*` unit test before landing: `port_usage`,
  `calc_usage`, and `connection_usage_member` all call a bare `name`/`identification` parse
  immediately after their keyword with no guard against the literal token `def`, so stacking the
  new `_def_required` parsers *after* them (the naive wiring) would have made `port def Foo;`,
  `calc def Foo;`, and `connection def Foo;` silently misparse as usages named `"def"`. Fixed by
  ordering every new def parser *before* its usage sibling in `part_def_body_element`'s `alt`,
  matching the precedent already set for `flow_def`/`flow_usage_member`. `allocation_usage`,
  `view_usage`, `viewpoint_usage`, and `rendering_usage` have the identical bare-`name` shape and
  were ordered def-before-usage from the start to avoid the same risk proactively.
- **`case`/`analysis`/`verification`/`use case` needed no new guard**: their `_def` parsers already
  use `DefinitionPrefixOptions::def_required()` (per PAR-006a), and their `_usage` counterparts go
  through `case_like_usage_body`/`use_case_usage_tail`, not a bare `name` call, so no ordering fix
  was needed for this family (still placed def-before-usage for consistency).
- **PAR-002 acceptance-criterion tests added**: `connection_def_is_same_variant_kind_at_package_level_and_nested_in_part`
  and `case_def_is_same_variant_kind_at_package_level_and_nested_in_part`
  (`src/parser/part/body.rs`), covering the two families most at risk of the ambiguity bug class
  found in this increment.
- **Scope remaining, updated**: `PartUsageBodyElement` (zero Def-kind variants), `PortDefBodyElement`/
  `PortBodyElement`, `InterfaceDefBodyElement`/`ConnectionDefBodyElement` still unaddressed.
  `PackageBodyElement`'s gaps (standalone `AttributeUsage`/`ItemUsage`/`PortUsage`,
  `ConnectionUsage`, `RefDecl` usage, `EnumerationUsage`) still unaddressed. `PartDefBodyElement`
  itself is now believed complete against the gaps-doc's original list for that enum.

### PAR-002 (increment 3 of N): standalone usages in `PackageBodyElement`

- **Added `AttributeUsage`, `ItemUsage`, `PortUsage`, `ConnectionUsage`, `Ref`, `EnumerationUsage`
  to `PackageBodyElement`** (`src/ast/package.rs`), wired into `try_package_body_annotations`/
  `try_package_body_structure`/`try_package_body_behavior` (`src/parser/package.rs`), each tried
  immediately after its existing `def`-optional definition counterpart (`attribute_def(.., false)`,
  `port_def`, `item_def`, `connection_def`, `enum_def`). `Ref` reuses `part_ref_usage` (now
  re-exported as `crate::parser::part::part_ref_usage`/`connection_usage_member`), which despite
  the name has no part-specific grammar. Confirmed via BNF: `PackageMember` explicitly allows
  `DefinitionElement | UsageElement`, so bare usages are legal package content, not just
  definitions -- contrary to `attribute_def`'s old doc comment claiming "only definitions are
  legal" at package level.
- **Important finding, confirmed empirically via the validation gate and targeted unit tests**:
  for the four keywords whose standalone `_def` parser is deliberately `def`-optional at package
  level (`attribute`, `port`, `item`, `connection`), that `_def` parser's header grammar
  (`parse_optional_definition_header_after_identification` in `src/parser/specialization.rs`)
  already swallows a plain `name: Type;` shape as part of its own (loosely-validated) header text,
  so ordinary typed usages like `port p1: MyPortType;` are captured as the `Def` variant before
  the new `Usage` variant is ever reached -- not a regression introduced here (this parsing
  predates this increment; the validation gate stayed green with no snapshot changes, confirming
  no stdlib content shifted variant). The new `Usage` variants are real and correctly reachable,
  but only for shapes the `Def` grammar cannot parse at all: attribute's `:>>`-prefixed
  redefinition head (`attribute :>> mass = 5;`, only `attribute_usage`'s `PrefixRedefines` shape
  handles a bare `:>>` immediately after the keyword), port's `:>>`-prefixed head likewise,
  item's `subsets`/`references`/`crosses` clauses (not part of `item_def`'s header grammar), and
  connection's anonymous no-name form (`connection: LinkType;`, `connection_def`'s
  `Identification` requires a name/short_name). Verified each via a dedicated
  `package_body_accepts_standalone_*_usage` test using exactly one of these distinguishing shapes,
  not the common `name: Type;` form (which would have silently asserted against dead code).
- **Not fixed, and intentionally out of scope here**: the `Def`-swallows-typed-usage behavior
  above (losing the `: Type` reference into `specializes: None` on `port p1: MyPortType;`, e.g.)
  looks like a real pre-existing accuracy bug in `parse_optional_definition_header_after_identification`'s
  typing-colon-blob handling, but fixing it is a separate, higher-risk change (it's shared header
  logic used by many definition kinds, not `Usage`-wiring) outside this increment's scope. Flagging
  it here rather than silently working around it.
- **PAR-002 acceptance-criterion test added**: `attribute_usage_with_redefines_is_same_variant_kind_at_package_level_and_nested_in_part`
  (`src/parser/package.rs`) parses the same `:>>`-prefixed attribute usage at package level and
  nested in a part body, asserting the same AST variant kind both times.
- **`connection_usage_member` and `part_ref_usage` made `pub(crate)`** and re-exported from
  `crate::parser::part` so `src/parser/package.rs` can reuse them instead of duplicating the
  parsing logic.

### PAR-002 (increment 4 of N): nested `def` kinds in `PartUsageBodyElement`

- **`PartUsageBodyElement` had zero `Def`-kind variants before this increment.** Added `StateDef`,
  `MetadataDef`, `FlowDef`, `RequirementDef`, `OccurrenceDef`, `PortDef`, `CalcDef`,
  `ConnectionDef` (`src/ast/structure.rs`), wired into `part_usage_body_element`
  (`src/parser/part/usage.rs`) using the same parsers already wired for `PartDefBodyElement` in
  increments 1-2 (`state_def`, `metadata_def`, `flow_def`, `requirement_def`, `occurrence_def`,
  `port_def_required`, `calc_def_required`, `connection_def_required`). Justified by BNF
  `UsageBody = DefinitionBody`: a usage body legally contains nested definitions, not just nested
  usages, the same as a definition body does.
- **Same ordering discipline applied proactively**: `port_def_required`/`flow_def` placed before
  `port_usage`/`flow_usage_member` (both lack a guard against a bare `def` token, per the bugs
  found and fixed in increment 2), avoided from the start rather than caught after the fact this
  time. `calc_def_required`/`connection_def_required`/`state_def`/`metadata_def`/
  `requirement_def`/`occurrence_def` had no usage sibling already dispatched in this body, so no
  ordering risk for those.
- **nom `alt` tuple-arity limit hit and fixed**: adding 8 more branches pushed the flat `alt(...)`
  past nom's ~21-element tuple ceiling (a real compile error, not a lint). Restructured into three
  top-level nested `alt` groups (annotations/state/behavior-ish forms; the new def-kind group;
  port/ref/bind/satisfy/interface/connect/flow) rather than one flat list -- same technique
  already used in `part_def_body_element`.
- **PAR-002 acceptance-criterion test added**:
  `state_def_is_same_variant_kind_in_part_usage_body_as_in_part_def_body`
  (`src/parser/part/usage.rs`) confirms the same `state def` declaration yields `StateDef` whether
  nested in a part *usage* body or a part *definition* body.
- **Scope remaining**: `PortDefBodyElement`/`PortBodyElement`, `InterfaceDefBodyElement`/
  `ConnectionDefBodyElement` still unaddressed (item 4 of the coordinator's remaining list).
  `RequirementUsage`/other Usage-kind gaps on `PartUsageBodyElement` were out of scope for this
  increment (task asked specifically for Def-kind variants here).

### PAR-002 (increment 5 of 5, final): widen `PortDefBodyElement`/`PortBodyElement`/
`InterfaceDefBodyElement`/`ConnectionDefBodyElement`

- **`PortDefBodyElement`**: added `ItemDef` (`item_def_required`) and `EnumerationUsage`
  (`enum_usage`); it already had `AttributeDef`/`AttributeUsage`/`ItemUsage`/`PortUsage` from
  before this backlog. **`PortBodyElement`**: had zero attribute/item coverage -- added
  `AttributeUsage` (`attribute_usage`) and `ItemUsage` (`item_usage`).
- **`InterfaceDefBodyElement`** and **`ConnectionDefBodyElement`**: both had only
  `Doc`/`EndDecl`/`RefDecl`/`ConnectStmt`(+`Error` on the latter) -- no attribute/item/port
  coverage at all. Added `AttributeDef`/`AttributeUsage`, `ItemDef`/`ItemUsage`, `PortDef`/
  `PortUsage` to both, reusing the exact same parsers already wired into `PartDefBodyElement`/
  `PartUsageBodyElement`/`PackageBodyElement` in increments 1-4 (`attribute_def`/`attribute_usage`,
  `item_def_required`/`item_usage`, `port_def_required`/`port_usage`) -- no new parsers needed
  anywhere in this increment.
- **Def-before-usage discipline applied proactively** (no new bugs found this time, unlike
  increments 1-2): `item_def_required` placed before `item_usage`/`directed_item_usage`,
  `port_def_required` before `port_usage`, in every body wired this increment. `port_usage` and
  `item_usage` are the same parsers already confirmed (in increments 1-2 and this one) to lack a
  guard against a bare `def` token, so the ordering matters everywhere they're dispatched, not
  just in the bodies where the bug was originally found.
- **New cross-module imports**: `interface.rs` and `connection.rs` now import from
  `attribute.rs`/`item.rs`/`port.rs`. Checked for import cycles first (`port.rs`/`item.rs`/
  `attribute.rs` import nothing back from `interface.rs`/`connection.rs`) -- none introduced.
- **PAR-002 acceptance-criterion tests added**: one per widened file
  (`item_def_is_same_variant_kind_in_port_def_body_as_item_def_required_parser` in
  `src/parser/port.rs`, `port_def_is_same_variant_kind_in_interface_def_body_as_port_def_required_parser`
  in `src/parser/interface.rs`, `attribute_usage_is_same_variant_kind_in_connection_def_body_as_shared_parser`
  in `src/parser/connection.rs`), each confirming the shared parser accepts the identical snippet
  the new body-enum variant wraps.
- **PAR-002 is now believed complete against the gaps-doc's literal list**: every body enum named
  in the gaps doc's PAR-002 section and the coordinator's backlog (`PackageBodyElement`,
  `PartDefBodyElement`, `PartUsageBodyElement`, `PortDefBodyElement`, `PortBodyElement`,
  `InterfaceDefBodyElement`, `ConnectionDefBodyElement`) now carries the definition/usage variants
  called out as missing, wired with the `def_required()`/`_required` disambiguation guard
  established in PAR-006a everywhere a bare-`def`-vulnerable usage sibling exists in the same
  dispatch.
- **Known out-of-scope follow-up, not fixed here** (flagged for a separate task, not chased in
  this backlog per the coordinator): `parse_optional_definition_header_after_identification`
  (`src/parser/specialization.rs`) silently drops a plain `: Type` reference into an unparsed
  header blob for `def`-optional definitions (e.g. `port p1: MyPortType;` parses as `PortDef` with
  `specializes: None`, losing the type). Found and documented in PAR-002 increment 3; not this
  increment's concern.
- **Final read-through against the gaps doc's PAR-002 section caught one real gap this backlog's
  own tracking list had dropped**: `PartDefBodyElement` had `EnumerationUsage` but no `EnumDef` --
  called out explicitly in the original triage ("has ... `EnumerationUsage` but not `EnumDef`")
  but never actually landed in increments 1-2. Added `EnumDef` to both `PartDefBodyElement` and
  `PartUsageBodyElement` (`enum_def` is `def_required()`-guarded internally, so no new disambiguation
  work needed), plus a `enum_def_is_same_variant_kind_at_package_level_and_nested_in_part`
  acceptance test. With this fix, every def/usage pair from the original per-enum gap list is
  confirmed present.

### PAR-006a: recovery-guard foundation

- **Confirmed the PAR-001 disambiguation fix was already generalized**: `attribute_def`'s
  `disambiguate_from_usage` parameter (the original PAR-001 fix) has a reusable form,
  `DefinitionPrefixOptions::def_required()` in `src/parser/definition_prefix.rs`, already adopted
  by `action`, `allocation`, `case`/`analysis`/`verification`/`use case`, `enum`, `flow`,
  `individual`, `interface` (`interface_def_required`), `item` (`item_def_required`), `metadata`,
  `occurrence`, `requirement`, `state`, and `view`/`viewpoint`/`rendering`. Documented this
  explicitly in `definition_prefix.rs` so PAR-002's new body-enum wiring reuses it instead of
  hand-rolling another bespoke guard, and documented which keywords (`connection`,
  `constraint`/`calc`, `port`) deliberately keep `def` optional and why (the real Systems
  Library uses bare, `def`-less forms for those at namespace level; making `def` required there
  was tried and reverted per the 0.33.0 entry below).
- **Removed dead `PortBodyElement::Other(String)` variant**: the enum declared it, but no parser
  ever constructed it (`port_body_element` in `src/parser/port.rs` only ever produces
  `PortUsage`/`InOutDecl`/`Doc`/`Error`); recovery already falls through to a real
  `Error(Node<ParseErrorNode>)`, not this variant. This was flagged as a potential silent-fallback
  gap during PAR-006 triage but turned out to be unreachable code, not a live bug — removed rather
  than "fixed".
- Reviewed `ParseErrorNode` (`src/ast/common.rs`): its existing fields (`message`, `code`,
  `expected`, `found`, `suggestion`, `category`) are sufficient for the recovery markers needed by
  the rest of this backlog — no new fields added.

### PAR-004 (item 6 of 6): standalone `FeatureChain` type

- **Added `ast::FeatureChain`**: a dot-separated feature chain (e.g.
  `engine.fuelCmdPort.flowRate`) had no dedicated AST node distinct from a `::`-qualified name —
  only `Expression::FeatureRef`/`Expression::MemberAccess` folded chains into nested expression
  nodes with no reusable, standalone shape. New `FeatureChain { segments: Vec<String>, span: Span
  }` in `src/ast/feature_chain.rs`, with a `parser::feature_chain` parser in
  `src/parser/feature_chain.rs` built on the existing `name`/`ws_and_comments` lexer helpers.
  Deliberately **not** wired into `Expression` or `src/parser/expr.rs` — PAR-005 (complete
  expression AST) is expected to adopt this type for its own `path_expression` parsing once it
  lands, without needing to touch `expr.rs`. The parser function is unused today
  (`#[allow(dead_code)]`) until a relationship parser or PAR-005 calls it.

### PAR-004/PAR-003 (item 5 of 6): structured `Multiplicity` bounds

- **Added `ast::Multiplicity`**: multiplicity brackets (`[1..*]`, `[0..1]`, `[3]`) were stored as
  raw `Option<String>` bracket text on 9 fields across `PartUsage`, `PortUsage`, `SuccessionUsage`
  (three fields), `ItemUsage`, `EnumerationUsage`, `IncludeUseCase`, and `ReturnRef` — no
  structured access to lower/upper bounds. New `Multiplicity { lower: Option<Box<Node<Expression>>>,
  upper: Option<Box<Node<Expression>>>, span: Span }`, populated by a new
  `parser::usage::multiplicity_node` that reuses the existing `expression()` parser for bound
  expressions. A bare `[3]` sets `lower == upper == Some(3)`; `[1..*]` sets `upper = None`
  (unbounded). `Multiplicity`'s `PartialEq` ignores `span`, matching `Node<T>`'s existing
  convention, so hand-built expected ASTs in tests don't need to reproduce real spans.
- **Found and fixed a real parsing regression while building this**: an initial version handed the
  whole bracket content to `expression()` in one call. That broke `[1..*]` — `expression()`'s
  binary-operator chain commits once it matches `..` as a range comparison operator and does not
  backtrack when the right-hand side (`*`) fails to parse as a primary expression, so it hard-errors
  instead of returning just the left operand. Caught by
  `part_usage_ordered_before_colon_parses_without_recovery` in `tests/apollo_regressions.rs`, which
  parses `part engines[1..*] ordered : RocketEngine;` — a real Systems Library shape. Fixed by
  scanning for the closing `]` and an optional top-level `..` first, then handing each isolated
  bound substring to `expression()` separately, rather than the whole bracket content at once. This
  is exactly the class of regression this backlog's process discipline (run the full validation
  gate after every change, not just once at the end) is meant to catch.
- **Bumps `PARSE_AST_VERSION` to 10**: changes the schema of 9 existing struct fields
  (`Option<String>` → `Option<Node<Multiplicity>>`), invalidating parse caches built against the
  0.34.x schema.

### PAR-004 (item 1 of 6, scoped): typed `TypingRelationship` for definition-level `specializes`

- **Added `ast::TypingKind`/`ast::TypingRelationship`**: `{ target: String, kind, span: Span,
  is_conjugated: bool, is_implied: bool }`, per the design brief's combined shape (folding in
  PAR-003's conjugation concept). Replaces the `specializes: Option<String>` /
  `specializes_span: Option<Span>` field pair — previously carrying no span, kind, or conjugation
  information — with a single `specializes: Option<Node<TypingRelationship>>` field on all 21
  structs sharing that exact pair (`PartDef`, `ItemDef`, `IndividualDef`, `PortDef`,
  `InterfaceDef`, `ConnectionDef`, `MetadataDef`, `EnumDef`, `OccurrenceDef`, `ActionDef`,
  `FlowDef`, `AllocationDef`, `StateDef`, `RequirementDef`, `CaseDef`, `AnalysisCaseDef`,
  `VerificationCaseDef`, `UseCaseDef`, `ConstraintDef`, `ViewDef`, `ViewpointDef`,
  `RenderingDef`). Tractable as one change because all 21 structs source `specializes` from a
  single parser choke point (`specialization.rs`'s `parse_optional_definition_specialization`),
  which now builds the `TypingRelationship` node directly — `kind` is always
  `Subclassification` and `is_conjugated` is always `false`, matching prior behavior exactly
  (definition-level `:>`/`specializes` clauses don't accept a leading `~` today).
- **Deviation from the brief, noted for a follow-up slice**: `PartUsage`/`PortUsage`'s
  `type_name: String` (required, not `Option<String>`) doesn't match the
  `typing`/`specializes: Option<String>` pair shape the brief describes, so it was left
  untouched. `AttributeDef.typing` / `AttributeUsage.typing` — the actual `:` typing side,
  populated by a different code path (`attribute.rs` and `usage.rs`'s
  `typings()`/`conjugated_qualified_name`, which is where real `~` conjugation parsing already
  happens) — were also not migrated in this pass; conjugation there remains a literal
  `~`-prefixed string, unchanged from before this session.
- Regenerated the `function_based_behavior_3a` and `parts_tree_1a` validation snapshot fixtures
  (schema-only Debug-format diff; confirmed target text/spans/kind are correct before
  regenerating).
- Bumps `PARSE_AST_VERSION` to 11.

### PAR-004 (item 1, remaining slice) + PAR-003 (item 4): typed `:` typing and real `~` conjugation

- **`AttributeDef.typing` / `AttributeUsage.typing`** now use the same
  `Option<Node<TypingRelationship>>` shape as the `specializes` side, closing the deviation noted
  in the previous entry.
- **Real conjugation, not a folded string**: `usage::conjugated_qualified_name` used to return the
  target with a literal `~` prefix folded into the string (`"~PortConjugate"`), so nothing could
  query "is this conjugated" without re-parsing the string. It now returns `(bool, String)` — the
  `~` is stripped from the string and exposed as a real flag — and `usage::typings`/
  `optional_typings` thread that flag through as `(Span, bool, String)` instead of `(Span,
  String)`. `TypingKind::Typing` relationships built from this (`attribute.rs`'s new
  `typing_node`/`typing_relationship_node` helpers, mirroring `specialization.rs`'s
  `subclassification_node`) set `is_conjugated` from the real flag instead of always `false`.
- **No regression for untouched fields**: `optional_typings`/`typings` are also called by
  `occurrence_body.rs`, `part/usage.rs`, `port.rs`, `requirement.rs`, and `usage.rs`'s own
  `merge_usage_header` for fields that stay a plain `String`/`Option<String>` (`PartUsage`/
  `PortUsage.type_name`, `UsageHeader.type_name`) — out of scope per the previous entry's noted
  deviation. Each of those call sites now re-embeds `~name` from the `(bool, String)` pair when
  building its string, so their external behavior is byte-for-byte unchanged; only the internal
  representation moved from "folded in the string" to "tracked separately, then re-folded where a
  typed home doesn't exist yet."
- One pre-existing quirk preserved as-is (not introduced by this change): `attribute_def`'s
  leading-`:>`-subset-as-typing fallback (used when an attribute has no separate `:` typing) now
  builds a `TypingRelationship` with `kind: Subclassification` — correct, since that fallback
  really is reading a `:>`-shaped clause — but has no span to attach (this code path never tracked
  one, even before this change), so it uses `Span::dummy()`.
- Bumps `PARSE_AST_VERSION` to 12.

### PAR-004 (item 2 of 6): typed `SubsettingKind`/`SubsettingRelationship`

- **Added `ast::SubsettingKind`/`ast::SubsettingRelationship`**: `{ target: String, kind, span:
  Span, is_implied: bool }`, mirroring `TypingRelationship`'s shape and its
  `PartialEq`-ignores-`span` convention. Replaces the separate `subsets`/`redefines`/
  `references`/`crosses` `Option<String>` fields with `Option<Node<SubsettingRelationship>>` —
  kept as **separate fields per clause kind, not collapsed into one**, since not every struct
  supports all four (`ConnectionUsageMember` only has `subsets`/`redefines`; `ExhibitState` only
  has `redefines`) — on `AttributeUsage`, `PartUsage`, `PortUsage`, `ConnectionUsageMember`,
  `ExhibitState`, `OccurrenceUsage`, and `RequirementUsage` (the last two found via the same
  grep-for-the-same-shape sweep used for item 1's `specializes` fields; not in the original
  brief's explicit list, but they share the exact pattern). `PartUsage`/`PortUsage.subsets` keeps
  its existing `(target, optional value expression)` tuple shape — just the target is now a typed
  node instead of a bare string.
- Most call sites route through `usage.rs`'s `specialization_clauses` (the `subsetting`/
  `redefinition`/`reference_subsetting`/`cross_subsetting` parsers now build the relationship node
  directly, with a real span from operator through target) or `definition_prefix`'s shared
  plumbing, so the ripple was mostly mechanical. A few ad hoc `:>`/`:>>` prefix shapes parsed
  directly outside `usage.rs` (`attribute.rs`'s `attribute_feature_binding`/`metadata_binding`,
  `part/body.rs`'s `exhibit_state`/`connection_usage_member`) needed their own local
  `subsetting_relationship_node` helper and span capture.
- **Side improvement to the previous entry's noted quirk**: `attribute_def`'s leading-`:>`-subset-
  as-typing fallback used `Span::dummy()` for the resulting `TypingRelationship` because the
  leading subset clause it read from had no span of its own. Now that subsetting clauses carry
  real spans (this item), that fallback reuses the `SubsettingRelationship` node's own span
  instead of a dummy one.
- Regenerated the `functional_allocation_4a` and `parts_tree_1a` validation snapshot fixtures
  (schema-only Debug-format diff; confirmed target text, kind, and real spans before
  regenerating).
- Bumps `PARSE_AST_VERSION` to 13.

### PAR-004 (item 3 of 6): typed `ConnectionEnd`/`InterfaceEnd`

- **Added `ast::ConnectionEnd`**: `{ expression: Node<Expression>, span: Span }`, and
  `ast::InterfaceEnd` as a type alias for it. Replaces the plain `Node<Expression>` endpoints on
  `Connect.from`/`to` and `ConnectStmt.from`/`to`/`extra_ends` with `Node<ConnectionEnd>`,
  distinguishing a connector endpoint from an arbitrary standalone expression elsewhere in the
  AST.
- **`InterfaceEnd` is a type alias, not a duplicate struct**: checked `ConnectStmt`'s actual usage
  sites (`src/parser/interface.rs` and `src/parser/connection.rs`, both building a `ConnectStmt`
  from the same shared `connect_ends` parser shape) and
  `InterfaceUsage::TypedConnect`/`Connection` (`src/parser/part/usage.rs`) — an interface end
  carries nothing beyond what a generic connection end carries, both are just a path expression
  with a span. A distinct struct would have no fields of its own.
- **Deviation, noted for a follow-up**: scoped to `Connect`/`ConnectStmt` only, per the requested
  scope. `InterfaceUsage::TypedConnect`/`Connection`'s own `from`/`to` fields (also
  `Node<Expression>` today, and structurally the same "interface end" case) were left untouched —
  a natural follow-up now that `InterfaceEnd` exists, not done here to keep this change to its
  requested shape.
- `connection.rs` and `interface.rs` each define their own `connect_ends` parser (duplicated, not
  shared between the two files); both were updated in lockstep with a small
  `connection_end`/`connect_end` wrapper. `part/usage.rs`'s `connect_` (the part-usage-level
  `connect` statement, distinct from `connect_stmt`) needed its own wrapper too.
- Bumps `PARSE_AST_VERSION` to 14.

## [0.34.0] - 2026-07-15

### Fixed

- **`attribute_usage` accepts a leading visibility modifier**: 0.33.0 made `attribute_def` require
  an explicit `def` keyword whenever it's dispatched alongside `attribute_usage`, but
  `attribute_usage` itself never learned to accept `private`/`protected`/`public` the way
  `attribute_def` already did. A `def`-less, visibility-prefixed declaration — e.g. `private
  attribute zeroDegreeCelsiusInKelvin: ThermodynamicTemperatureValue = 273.15 [K];`, the shape
  used by the official Systems Library's `IntervalScale` catalog entries in
  `SysML Quantities and Units Library/SI.sysml` — correctly failed `attribute_def` (no `def`) but
  then also failed `attribute_usage` (no visibility handling) and fell through to an opaque
  recovered element instead of a proper `AttributeUsage`. `attribute_usage` now accepts and
  discards the same optional visibility prefix as `attribute_def`.

## [0.33.0] - 2026-07-13

Fixes PAR-001: `attribute def` vs. `attribute` usage were ambiguous in every body that permits
both (part, port, requirement, metadata), and the same class of bug existed for non-connector
`interface` members in part-def bodies. Both are now classified solely by an explicit, mandatory
`def` keyword in those ambiguous body contexts, rather than inferred from typing, modifiers, or
fallback parser ordering.

### Fixed

- **`attribute_def` requires `def`**: previously, `attribute_def` was tried before
  `attribute_usage` in part/port/requirement/metadata bodies and only deferred to
  `attribute_usage` for two narrow cases (`redefines`/`:>>`, or a fully untyped `attribute name =
  value`). A plain typed declaration without `def` — e.g. `attribute temperature :
  Temperature;` — fell through and was silently accepted as an `AttributeDef`. `attribute_def`
  now unconditionally requires `def` whenever it's dispatched alongside `attribute_usage`, so
  classification no longer depends on typing or value presence.
- **`attribute_usage` no longer drops a value bound in a leading `:>` (subsets) clause**: exposed
  by the fix above — `subsetting()` parses `target = value` as one unit, but `attribute_usage`'s
  merge logic kept only the target and discarded the value (e.g. `attribute v :> ISQ::speed = 0.9
  [m/s];` lost its default). The captured value is now threaded through as a fallback when no
  separate value expression follows.
- **`interface_def_required` for part-def bodies**: same bug class as `attribute_def`.
  `interface_usage` only recognizes connector forms (`connect ... to ...`), so a non-connector
  interface member without `def` (e.g. `interface foo : SomeInterfaceType;`) fell through and was
  silently accepted as an `InterfaceDef`. Part-def bodies now dispatch a new
  `interface_def_required` (mirroring the existing `item_def`/`item_def_required` split) that
  mandates `def`; a non-connector interface usage form is not yet supported by this parser and
  correctly surfaces as an explicit recovery/error element with a diagnostic instead of a false
  `InterfaceDef`. Package-level `interface_def` keeps `def` optional — the standard library uses
  bare, `def`-less `interface` usages at namespace level (e.g. `abstract interface interfaces:
  Interface[0..*] nonunique :> connections { ... }` in `Systems Library/Interfaces.sysml`) that
  this parser currently folds into `InterfaceDef`, and there's no dedicated package-level
  `interface_usage` dispatch to catch them instead.
- **`PARSE_AST_VERSION`** bumped from `7` → `8`: definition/usage classification changed for the
  cases above, so cached parses built against 0.32.x schema must be invalidated.

An earlier draft of this fix also made `port_def`, `constraint_def`, and `calc_def` require `def`
unconditionally. That broke the full SysML v2 release validation suite (`cargo test --test
validation -- --include-ignored` with `SYSML_V2_RELEASE_DIR` set): the standard library uses the
same bare, `def`-less usage pattern at namespace level for `port`, `constraint`, and `calc` (e.g.
`abstract port ports : Port[0..*] nonunique :> objects { ... }` in `Systems Library/Ports.sysml`,
and similarly in `Calculations.sysml`/`Constraints.sysml`), and none of these three have a
dedicated usage-form parser to fall back to at that level. All three were reverted to optional
`def`, same as `connection_def` (which was correctly left alone from the start — its
hash-annotation prefix, e.g. `#derivation connection { ... }`, is itself a valid definitional
marker in place of `def`).

### Added

- Regression tests: `test_part_def_body_distinguishes_attribute_def_from_usage_by_def_keyword`
  (the `Sensor` acceptance case — one `AttributeDef`, three `AttributeUsage`s for typed, untyped,
  and initialized forms without `def`) and
  `test_part_def_body_never_misclassifies_non_connector_interface_as_definition` in
  [`tests/parser/structure.rs`](tests/parser/structure.rs).

### Notes

- Verified against the full SysML v2 release validation suite (`cargo test --test validation --
  --include-ignored` with `SYSML_V2_RELEASE_DIR` pointing at a fetched
  `Systems-Modeling/SysML-v2-Release` checkout, matching `.github/workflows/ci.yml`'s `validation`
  job) in addition to the default `cargo test`. Regenerated
  [`tests/validation/snapshots/parts_tree_1a.txt`](tests/validation/snapshots/parts_tree_1a.txt)
  for the expected attribute reclassification in `01-Parts Tree/1a-Parts Tree.sysml`.

## [0.32.0] - 2026-07-06

Extends `variant` members in variation part bodies to support typed inline declarations
(SysML §7.6.7), and surfaces the leading `abstract` prefix on requirement, case, analysis,
verification, and use-case definitions/usages as `is_abstract` on the AST node (previously
accepted at parse time but discarded).

### Added

- **`VariantUsage.typed` / `VariantTypedUsage`**: typed `variant` members inside variation part
  def/usage bodies — `variant part name : Type { … }`, `variant attribute name = expr;`,
  `variant item …`, `variant port …` — in addition to the existing untyped reference form
  (`variant name;`, `typed: None`). `VariantTypedUsage` carries `Part`, `Attribute`, `Item`, or
  `Port` nested usage nodes. `variant_usage()` tries each kind parser before falling back to the
  untyped name-and-semicolon form.
- **`is_abstract` on requirement and case families**: `RequirementDef`, `RequirementUsage`,
  `CaseDef`, `CaseUsage`, `AnalysisCaseDef`, `AnalysisCaseUsage`, `VerificationCaseDef`,
  `VerificationCaseUsage`, `UseCaseDef`, and `UseCaseUsage` each gain `is_abstract: bool` (`true`
  for `abstract requirement def …`, `abstract analysis …`, etc.). Definition parsers read the
  flag from `parse_definition_prefix`; usage parsers capture a leading `abstract` keyword before
  the kind keyword.
- **Regression tests** [`tests/abstract_requirement_analysis_flags.rs`](tests/abstract_requirement_analysis_flags.rs)
  for `is_abstract` on requirement/analysis/verification/use-case defs and usages; extended
  [`tests/variation_variant_body.rs`](tests/variation_variant_body.rs) with the spec's
  `TransmissionChoices` typed `variant part` example.
- **`PARSE_AST_VERSION`** bumped from `6` → `7` to invalidate caches built against the 0.31.x
  schema.

## [0.31.0] - 2026-07-06

Fixes S42-LIM-014: `variant` members inside a `variation part def` body now parse as structured
AST nodes instead of falling through to error recovery. Part usage bodies already supported this
since 0.25.2 (`PartUsageBodyElement::VariantUsage`); part definition bodies did not.

### Added

- **`PartDefBodyElement::VariantUsage`**: `variant` *name* `;` inside a `variation part def` brace body (e.g. `variation part def NavigationSensorSuiteChoice :> SensorAssembly { variant tofImuOnly; … }`). `part_def_body_element()` dispatches via the existing `variant_usage` parser (now `pub(crate)`); `b"variant"` was already in `PART_BODY_STARTERS` from 0.25.2 so recovery did not abort, but unrecognized members were previously misclassified as recovery errors rather than owned `VariantUsage` nodes.
- **Regression test** [`tests/variation_variant_body.rs`](tests/variation_variant_body.rs) for a three-variant `variation part def` body with no recovery `Error` nodes.
- **`PARSE_AST_VERSION`** bumped from `5` → `6` to invalidate caches built against the 0.30.x schema.

## [0.30.0] - 2026-07-03

Closes all 7 open follow-ups listed in `docs/PARSER_BACKLOG_ROADMAP.md` § 5 from the 0.29.0
state/action/connector audit, plus three unrelated `doc`-comment parsing gaps found while
fixing a Babel42 Components-tab bug report.

### Added

- **`doc` support in port usage bodies**: `PortBodyElement` gains a `Doc(Node<DocComment>)` variant. Previously a `doc /* ... */` block inside a `port name : Type { ... }` *usage* body (as opposed to a `port def` body, which already supported it) failed to parse, and error recovery misclassified the doc text as a "bare feature declaration in part definition body".
- **`doc` support in `connection def` bodies**: `ConnectionDefBodyElement` gains `Doc(Node<DocComment>)`. `connection_def_body_element()` also gains an `Error(Node<ParseErrorNode>)` variant; `connection_member_body` is migrated from a hand-rolled recovery loop (whose only fallback silently discarded every member *after* an unrecognized one, not just the bad one) to the shared `parse_structured_brace_members` helper, matching `port_body_brace`.
- **`doc` support in interface usage connect bodies**: `InterfaceUsageBodyElement` gains `Doc(Node<DocComment>)`. Previously a `doc` block inside `interface ... connect ... to ... { ... }` caused the entire interface usage to be discarded as a generic recovery `Error` node one level up (`connect_body_with_elements` had no recovery path at all).
- **`if` / `while` / `terminate` control nodes**: new `IfStmt`, `WhileStmt`, `TerminateStmt` AST nodes and matching `ActionDefBodyElement`/`ActionUsageBodyElement` variants. `if`/`while` bodies are fully structured (`ActionDefBody`, reusing `action_def_body_brace` for real recursion, including nested control nodes), not the opaque `FirstMergeBody` that `decide`/`join`/`fork` use. `terminate` accepts an optional target expression (`terminate;` / `terminate someAction;`). Both `alt()` dispatchers needed nesting into a sub-`alt()` to stay under nom's 21-branch tuple limit.
- **Standalone `succession` usage**: new `SuccessionUsage` AST node (`OccurrenceBodyElement::SuccessionUsage`) for `succession (first)? source then target;` written directly in a definition/occurrence body — distinct from the action-body `FirstStmt`/`MergeStmt` control node and from `succession flow X to Y;`. Also models the multiplicity-bearing form actually used by the SysML Systems Library (`succession [seBeforeNum] first [0..1] sourceEvent then [0..1] self;` in `Flows.sysml`): `SuccessionUsage.multiplicity`/`source_multiplicity`/`target_multiplicity`. Previously swallowed as opaque `Other(String)` text via `DEFINITION_BODY_OPAQUE_STARTERS`.
- **Transition trigger `via`**: `TransitionAccept::Payload`/`Shorthand` each gain an `Option<Node<Expression>>` for a trailing `via <port>` clause (e.g. `accept TurnOn via commPort`). Previously only the `do`-effect `accept`/`send` forms supported `via`; the trigger clause (before `if`/`do`) did not.
- **`satisfy requirement <name> : <Type> by <expr>`**: `Satisfy` gains `inline_requirement: Option<InlineSatisfyRequirement>` for the fuller named/typed requirement-usage form, reusing `optional_typings()` from `usage.rs`. The bare `satisfy <ref> (by <expr>)?;` shorthand is unaffected (`inline_requirement: None`).
- **`assert constraint` / `satisfy` reachable from more scopes**: `PartDefBodyElement` gains `AssertConstraint(Node<AssertConstraintMember>)` and `Satisfy(Node<Satisfy>)` — both previously only reachable from `occurrence def` bodies (`assert constraint`) or package level (`satisfy`), not `part def` bodies. `OccurrenceBodyElement` gains `Satisfy(Node<Satisfy>)` (occurrence def bodies already had `AssertConstraint`). `b"assert"` added to `PART_BODY_STARTERS`; `b"satisfy"` added to `OCCURRENCE_BODY_STARTERS`.
- **KerML arrow-invocation operator**: `postfix()` in `expr.rs` now parses `->name` / `->name(args)` (e.g. `collection->size()`, `powerProfile->size()-1`) at the same precedence level as `.` member access, desugaring into the existing `Expression::MemberAccess`/`Invocation` shapes — no new `Expression` variant, so no downstream exhaustive matches needed updating.
- **`PARSE_AST_VERSION`** bumped from `4` → `5` to invalidate caches built against the 0.29.x schema.

### Changed

- **`AssignStmt.rhs`**: changed from `String` to `Node<Expression>`, unblocked by the arrow-invocation operator above (real models commonly write `x := collection->size();`, which previously required the raw-text fallback).
- **`ForLoop.range` raw-text fallback**: retained as a defensive net (not removed — recovery/fallback paths are kept per project convention), but the arrow-invocation operator above means it's no longer hit for the common case; doc comment updated accordingly.

## [0.29.0] - 2026-07-01

### Added

- **State `do`/`exit` actions**: `StateDefBodyElement` gains `Do(Node<DoAction>)` and `Exit(Node<ExitAction>)` variants, mirroring the existing `Entry` handling. Previously `do action ...` / `exit action ...` in a state body fell through to error recovery; only `entry` was recognised. `b"do"` / `b"exit"` added to `STATE_BODY_STARTERS`.
- **Control nodes `decide` / `join` / `fork`**: new `DecisionStmt`, `JoinStmt`, `ForkStmt` AST nodes and `ActionDefBodyElement`/`ActionUsageBodyElement` variants, parsed the same way as the existing `MergeStmt`. The action-body keyword lists previously listed the non-spec keyword `decision` instead of `decide`, and had no parser at all for `join`/`fork`.
- **Negated `assert` / `satisfy`**: `AssertConstraintMember` and `Satisfy` both gain `is_negated: bool`. `assert not constraint { ... }` and `(assert)? not satisfy X (by Y)?;` now parse; previously `not` after `assert` was unhandled and `assert`-prefixed `satisfy` had no parse path at all (only the bare `satisfy X by Y;` shorthand worked).
- **Structured transition `do` effect**: new `TransitionEffect` enum (`Perform`, `Accept`, `Send`, `Assign`, `Expression`) replaces the raw `Node<Expression>` previously stored on `Transition.effect`. Recognises `do action name : Type`, `do accept payload (: Type)? (via expr)?`, `do send payload (: Type)? (via expr)? (to expr)?`, and `do assign lhs := rhs`; falls back to a bare expression for anything else.
- **`ConnectStmt.extra_ends`**: `ConnectStmt` gains `extra_ends: Vec<Node<Expression>>` for the SysML v2 n-ary connector/interface form `connect (a, b, c, ...)`. The ordinary binary `from ... to ...` form is unaffected (`extra_ends` is empty).
- **`PARSE_AST_VERSION`** bumped from `3` → `4` to invalidate caches built against the 0.28.x schema.

### Changed

- **`ForLoop.range`**: changed from `String` to `Node<Expression>`. The `for x in <range> { ... }` range now parses as a structured expression (e.g. `1..10`); falls back to raw text only when the expression grammar can't parse the range (e.g. KerML `->` arrow-invocation syntax, not yet modelled — see `docs/PARSER_BACKLOG_ROADMAP.md` § 5).
- **`AssignStmt.lhs`**: changed from `String` to `Node<Expression>`, parsed via `path_expression` (matches spec's `AssignmentTargetParameter` feature-chain shape). `AssignStmt.rhs` remains a raw `String` (out of scope for this pass).

See `docs/PARSER_BACKLOG_ROADMAP.md` § 5 for the full list of gaps this closes and the related follow-ups intentionally left open (`if`/`while`/`terminate` control nodes, standalone `succession` usage, transition trigger `accept ... via`, full `satisfy requirement name : Type` form, `assert`/`satisfy` scope wiring, KerML arrow-invocation expressions).

## [0.28.0] - 2026-06-29

### Added

- **`MetadataAnnotation` in use-case, view-def, and calc-def bodies**: `UseCaseDefBodyElement`, `ViewDefBodyElement`, and `CalcDefBodyElement` now carry a `MetadataAnnotation` variant. `@Name` annotations in those bodies are parsed via the structured `metadata_annotation()` path (previously fell through to the generic `Annotation` fallback or were silently skipped). Recovery starters updated: `b"@"` added to `USE_CASE_BODY_STARTERS`, `VIEW_DEF_BODY_STARTERS`, and `CALC_DEF_BODY_STARTERS`.
- **`CaseReturnDecl`**: new AST node modelling `return [attribute] name [: Type] [= expr] ;` and `return :>> name [= expr] ;` in analysis and verification case bodies. Adds `UseCaseDefBodyElement::CaseReturnDecl`. Fields: `name: String`, `name_span: Option<Span>`, `type_name: Option<String>`, `is_redefine: bool`. Previously these forms fell through to `Other`.
- **Rep language diagnostics**: `textual_representation()` now parses `language STRING_VALUE` resiliently. A missing `language` keyword produces a `TextualRepresentation` node with `language_span = None`; an empty language string is flagged. `collect_errors` emits `missing_rep_language` or `invalid_rep_language` (both already in `diagnostic_catalog.rs`) for these cases. Wired in requirement body and package body collectors.
- **`PARSE_AST_VERSION`** bumped from `2` → `3` to invalidate caches built against the 0.27.x schema.
- **Recovery tests for view-def and constraint-def bodies**: `view_def_recovery_inserts_error_node_and_keeps_later_render` and `constraint_def_recovery_inserts_error_node_and_keeps_later_sibling` added to `tests/recovery_diagnostics_integration.rs`, verifying that a malformed token produces a `ParseErrorNode`, surfaces as a diagnostic, and does not abort parsing of subsequent members.

### Fixed

- **`textual_representation()` resilience**: the `language STRING_VALUE` clause is now parsed with soft-failure so a missing `language` keyword produces a structured node (with `language_span = None`) rather than a hard nom error, enabling the error collector to emit a targeted `missing_rep_language` diagnostic.

## [0.27.0] - 2026-06-29

### Added

- **`InOutDecl.value`**: `in`/`out`/`inout` parameter declarations now carry an optional `value: Option<Node<Expression>>` field. The parser recognises `= expr` default-value initialisers after the type annotation and stores them; the error-recovery path yields `None`.
- **`RefBody::Brace { elements }`**: the unit variant `RefBody::Brace` is replaced by a struct variant carrying `elements: Vec<Node<ActionDefBodyElement>>`. Action-context ref bodies (`ref action name: Type { … }`) are fully parsed into structured elements via `parse_structured_brace_members`; other ref-body contexts (state, part, interface, connection) remain opaque and produce an empty element list.
- **`Satisfy.body_elements`**: `Satisfy` gains `body_elements: Option<Vec<Node<ConstraintDefBodyElement>>>`. Braced satisfy bodies now expose their structured constraint members instead of discarding them; semicolon-terminated satisfies yield `None`.
- **`Import.target_span`**: `Import` gains `target_span: Span` recording the source span of the qualified name (excluding `::*` / `::**` suffix), enabling semantic-token providers to highlight only the name portion.
- **`PARSE_AST_VERSION`** bumped from `1` → `2` to invalidate caches built against the 0.26.x schema.

### Fixed

- **Error collection for `RefBody::Brace { elements }`**: `collect_action_def_body_errors` now recurses into `ActionDefBodyElement::RefDecl` bodies via the new `collect_ref_body_errors` helper, surfacing `ParseErrorNode`s nested inside action ref bodies as LSP diagnostics.
- **Error collection for `Satisfy.body_elements`**: `collect_part_usage_body_errors` and `collect_package_body_errors` now walk `Satisfy.body_elements` via `collect_constraint_body_element_errors`, surfacing constraint-body recovery errors inside braced satisfy statements.

## [0.26.0] - 2026-06-26

### Added

- **`serde` feature**: all AST types (`Span`, `Node<T>`, `Expression`, operators, and every `pub struct` / `pub enum` in `core`, `common`, `root`, `package`, `structure`, `behavior`, `requirement`, `view`, `kerml_fallback`) now conditionally derive `serde::Serialize` / `serde::Deserialize` when the `serde` feature is enabled. Enable with `sysml-v2-parser = { version = "0.26.0", features = ["serde"] }`. `DiagnosticCategory` and `DiagnosticSeverity` from `error` also gain the derives so that `ParseErrorNode` (embedded in the AST) round-trips cleanly.
- **`PARSE_AST_VERSION: u32` constant** (in `lib.rs`): must be incremented on any breaking AST schema change. Consumers that cache serialized parse results (e.g. an LSP parse cache keyed by content hash) embed this value in cache entries and reject stale entries on mismatch.

### Fixed

- **Diagnostic collection gaps** in `collect_errors.rs`: `ParseErrorNode`s nested inside the bodies of `CaseDef`, `CaseUsage`, `AnalysisCaseDef`, `AnalysisCaseUsage`, `VerificationCaseDef`, `VerificationCaseUsage`, `ViewpointDef`, `ViewpointUsage`, `RenderingDef`, `PortDef`, `AttributeDef`, `ItemDef`, `IndividualDef`, `MetadataDef`, `MetadataUsage`, `OccurrenceDef`, `OccurrenceUsage`, `AllocationDef`, `AllocationUsage`, `FlowDef`, and `FlowUsage` were silently dropped and never surfaced as LSP diagnostics; they now propagate correctly.
- **`collect_part_def_body_errors`**: errors inside `ExhibitState`, `RequirementUsage`, `OccurrenceUsage`, `AttributeDef`, and `AttributeUsage` members of part definition bodies now surface.
- **`collect_part_usage_body_errors`**: errors inside `OccurrenceUsage` and `AttributeUsage` members of part usage bodies now surface.
- Five new helper collectors introduced: `collect_attribute_body_errors`, `collect_definition_body_errors`, `collect_occurrence_usage_body_errors`, `collect_port_def_body_errors`, `collect_rendering_def_body_errors`.
- **Parser gaps in SysML standard library**: attribute, metadata, occurrence-definition, port-definition, view-definition, and rendering-definition bodies now recognise and capture as `Other` a range of previously-unhandled syntax forms that appear in the SysML standard library (e.g. `ref self`, `item start`, `end source`, `abstract ref port`, `derived ref item`, `binding`, `connection`, `in event occurrence`, `succession`, multi-target `:>>`, and conditional `if ? else` expressions in attribute values). All 94 standard-library files now parse without diagnostics.

## [0.25.4] - 2026-06-12

### Added

- **Generic `FlowUsage`**: unified parser for `flow`, `message`, and `succession flow` with optional name, `of` payload, `from`/`to` or shorthand `expr to expr`, and `DefinitionBody` brace bodies.
- **`FlowUsageKind`** and extended **`FlowUsage`** AST (replaces anonymous **`Flow`** struct in action bodies).
- **`FlowUsage`** wiring in `OccurrenceBodyElement`, `PartDefBodyElement`, `PartUsageBodyElement`, `UseCaseDefBodyElement`, and action bodies (via shared `flow_usage_member`).

### Changed

- **`flow_usage` / `flow_usage_member`**: supersedes named-only package parser and action-body `flow_` helper.
- **`PART_BODY_STARTERS` / `OCCURRENCE_BODY_STARTERS`**: include `flow`, `message`, `succession`.

## [0.25.3] - 2026-06-12

### Fixed

- **State transitions**: `transition <name> first <source> then <target>` no longer sets `is_initial` on named transitions; only unnamed `transition first … then …` forms are initial transitions.

## [0.25.2] - 2026-06-11

### Added

- **`PartUsage.usage_prefix`**: optional `abstract` or `variation` prefix on part usages (package-level `variation part …` and nested part usage bodies).
- **`PartUsageBodyElement::VariantUsage`** and **`VariantUsage`**: `variant` *name* `;` inside variation part usage bodies.
- **`variation` / `variant`** in `PART_BODY_STARTERS` so part usage body recovery does not abort on variation modelling members.

### Changed

- **`part_usage`**: parses leading `abstract` / `variation` before `part` (nested bodies); package-level `part_def_or_usage` propagates `usage_prefix` onto usages.
- **Validation fixtures** updated for the new `PartUsage` field (`usage_prefix: None`).

## [0.25.1] - 2026-06-11

### Added

- **`MetadataKeywordUsage`** in action and usage body element parse paths.
- **`Expression::MetaCast`**: reflective `meta` cast expressions.
- **`about` targets** on metadata annotations and metadata usages.
- **Tests** in [`spec42_diagnostics_ast.rs`](tests/spec42_diagnostics_ast.rs) for metadata keyword, `about`, and meta-cast forms.

## [0.24.0] - 2026-06-10

### Added

- **`ConstraintDefBodyElement::MetadataAnnotation`**: `@Name : Type` in constraint definition bodies (not only generic expressions).
- **`b"@"`** in `CONSTRAINT_DEF_BODY_STARTERS` for structured recovery.
- **Fixture** [`constraint-metadata-annotation.sysml`](tests/fixtures/constraint-metadata-annotation.sysml).
- **`ReturnRef.return_expression`**: structured `return <expr>;` inside verification return-ref bodies.
- **Recovery tests** for state `ref` brace bodies and part-usage bind/ref connect bodies.

### Changed

- **Opaque brace bodies** in `state.rs` (`ref` bodies) and `part/usage.rs` (bind/ref connect bodies) use structured recovery instead of silent `advance_to_closing_brace` (5 sites removed).
- **`use_case_def_body_brace`** uses `parse_structured_brace_members` (removes body-level abort); `ref :>>` and `return ref` inner bodies parse with structured recovery.
- **`return_expression_stmt`** shared between calc and return-ref bodies (was private `calc_return_expression`).

## [0.23.0] - 2026-06-10

### Added

- **`Expression::Classification`**: `@Metaclass` filter/guard forms (e.g. `@SysML::PartUsage`) no longer stored as `FeatureRef("@…")`.
- **`Expression::TypeCheck`**: `istype`, `hastype`, and `as` type tests with optional operand.
- **`Expression::Select` / `Collect`**: structured `.?` / `.**` selector expressions.
- **Typed `stakeholder name : Type;`**: extends `StakeholderMember` while keeping shorthand `stakeholder ConcernRef;`.
- **`MetadataAnnotation`** variants on `StateDefBodyElement` and `PartDefBodyElement` parse paths.
- **Fixtures** [`expression-classification.sysml`](tests/fixtures/expression-classification.sysml), [`stakeholder-typed.sysml`](tests/fixtures/stakeholder-typed.sysml).

### Changed

- **Opaque brace bodies** in `action.rs` and `requirement.rs` use structured recovery instead of silent `advance_to_closing_brace` (11 sites removed).
- **`constraint_body` / `satisfy` connect bodies** delegate to `structured_constraint_body`.
- **`metadata_ref_primary`** emits `Classification` AST nodes.

## [0.22.0] - 2026-06-09

### Added

- **`ItemUsage.direction`**: optional `InOut` on item usages parsed as `in`/`out`/`inout item …` in port def bodies.
- **`directed_item_usage`**: parser path for direction-prefixed item usages (SysML §7.6.3 / port directed-features table).
- **`PortDefBodyElement::ItemUsage`**: port def brace bodies accept directed items before legacy `in_out_decl` pins.
- **`tests/vacuuming_types_parse`**: inline fixtures for directed port items, block/line comments with braces, and optional live-corpus checks via `MBSE_VACUUM_EXAMPLE_DIR`.
- **Fixture** [`tests/fixtures/port-directed-item-inout.sysml`](tests/fixtures/port-directed-item-inout.sysml).

### Fixed

- **`in_out_decl`**: directed port pins accept `:>` subsetting (e.g. `out volume :> ISQSpaceTime::volume`), not only `:` typing.
- **EOF brace balance**: `has_unclosed_brace` / `extra_closing_brace_at_eof` ignore `{`/`}` inside `//` and `/* */` comments (fixes false `missing_closing_brace` on commented-out blocks).
- **Recovery**: `local_recovery_line_boundary` skips `//` line comments correctly; bounds check in `balanced_inline_depth`.

### Changed

- **`direction_prefix`**: `pub(crate)` in [`attribute.rs`](src/parser/attribute.rs) for reuse by item and attribute directed-usage parsers.

## [0.21.0] - 2026-06-09

### Added

- **`AttributeUsage.direction`**: optional `InOut` when parsed as `in`/`out`/`inout attribute …` in port def bodies.
- **`directed_attribute_usage`**: parser path for direction-prefixed attribute usages, including `out attribute redefines …`.

### Fixed

- Port def bodies no longer mis-parse `out attribute redefines name :> Type` as `InOutDecl` with name `redefines`.

## [0.20.0] - 2026-06-08

### Added

- **DefinitionBody parity** — `flow def`, `flow`/`message` usage, `allocation def`, and `allocation`/`allocate` usage brace bodies now parse occurrence-style members (`attribute`, `part`, `occurrence`, `assert constraint`, `doc`, …) via shared [`occurrence_body.rs`](src/parser/occurrence_body.rs), aligned with `occurrence def` bodies.

### Changed

- Doc-only flow/allocation brace bodies now surface as `DefinitionBodyElement::OccurrenceMember(Doc(...))` instead of top-level `DefinitionBodyElement::Doc`.
- Unknown statements in generic `DefinitionBody` brace bodies emit recovery `Error` nodes instead of being silently skipped.

## [0.19.0] - 2026-06-08

### Breaking

- **`ActionUsage.accept` / `ActionUsage.send`**: now `Option<PayloadClause>` (was `Option<(String, String)>`).
- **`Transition`**: added `is_initial`, `accept: Option<TransitionAccept>`; `BinaryOperator` / `UnaryOperator` replace raw operator strings in expressions.

### Added

- **`PayloadClause`**, **`TransitionAccept`**, **`FinalState`** AST members for state-machine accept/send/final syntax.
- **`MetadataKeywordUsage`** (`#keyword`) in part, state, requirement, and use-case bodies.
- **`StakeholderMember`**, **`PurposeMember`**, **`TextualRep`** in requirement/viewpoint bodies.
- **`spec42_diagnostics_ast`** integration tests and fixtures for the June 2026 parser wave.

## [0.18.0] - 2026-06-05

### Breaking

- **`MetadataDef.body`**: now `AttributeBody` (was `DefinitionBody`) so `attribute` members in metadata definitions parse structurally like item definitions.
- **`PackageBodyElement`**: added `MetadataUsage` for package-level `metadata name : Type` declarations.

### Added

- **`MetadataUsage`** AST and `metadata_usage` parser (BNF MetadataUsageDeclaration).
- **Expose feature chains**: `expose` targets accept dot-separated usage segments after the initial qualified name (SysML §7.6.6).

### Removed

- **`invalid_qualified_name_separator`** diagnostic for valid expose feature-chain notation (dots between usage segments).

## [0.17.0] - 2026-06-04

### Breaking

- **`AttributeUsage`**: added `subsets`, `references`, and `crosses` (`Option<String>`) for `:>` / `::>` / `=>` clauses on attribute usages. Manual struct literals and destructuring must include the new fields (`None` when absent).
- **`PortUsage`**: added `references` and `crosses` for the same operators on port usages.

### Added

- **P2–P4 parser debt (complete)**: plans and status in [`docs/PARSER_DEBT_P2_PLAN.md`](docs/PARSER_DEBT_P2_PLAN.md), [`docs/PARSER_DEBT_P3_PLAN.md`](docs/PARSER_DEBT_P3_PLAN.md), [`docs/PARSER_DEBT_P4_PLAN.md`](docs/PARSER_DEBT_P4_PLAN.md); P4 checklist marked done in [`docs/PARSER_TECHNICAL_DEBT.md`](docs/PARSER_TECHNICAL_DEBT.md).
- **Structured view and part definition bodies**: `view_body` and `part_def_body_brace` use `parse_structured_brace_members_with_skip` with scoped recovery (`BodyElementRecover`, `view_body_recovery`); recovery nodes are retained when recovery ends at `}` (fixes empty view bodies and missing expose diagnostics).
- **Definition headers** ([`src/parser/definition_header.rs`](src/parser/definition_header.rs)): shared header parsing; pilot use in item and view definition paths.
- **Expression surface**: `implies` below `or` / `and` in [`src/parser/expr.rs`](src/parser/expr.rs) with unit coverage.
- **Recovery and LSP**: clearer `Many0` / `Many1` messages in [`src/parser/diagnostics.rs`](src/parser/diagnostics.rs); `expose_member` rejects `.` after a qualified expose name; `tests/recovery_body_scopes.rs` adds `part_def_recovery_keeps_later_members`.
- **Module layout (internal, public API unchanged)**: `src/ast.rs` split into `src/ast/`; parser helpers split into `diagnostics.rs`, `recovery.rs`, `collect_errors.rs`, `parse.rs`; `part.rs` split into `src/parser/part/` (`mod`, `prelude`, `body`, `def`, `usage`); integration tests split under `tests/parser/`.

### Changed

- **Port and attribute usages**: wire `references` / `crosses` (and attribute `subsets`) from shared `usage_header` parsing; AST normalization updated in [`src/ast/mod.rs`](src/ast/mod.rs).
- **Validation snapshots**: `parts_tree_1a` and `functional_allocation_4a` AST snapshots refreshed for structured-body and usage-header shapes.
- **BNF compliance test paths**: part grammar references `part/def.rs` and `part/usage.rs` after the part module split.

### Fixed

- **View body recovery**: structured brace loop no longer drops the final recovery `Error` node when the skip ends at `}` (regression that hid invalid `expose` separator diagnostics).
- **Clippy**: `#[allow(clippy::large_enum_variant)]` on affected enums in [`src/ast/structure.rs`](src/ast/structure.rs) after usage AST growth.

### Migration (Spec42 and similar hosts)

1. Bump to `sysml-v2-parser` `0.17.0` (crates.io or tag `v0.17.0`).
2. Extend `AttributeUsage` / `PortUsage` construction and matches with `subsets` / `references` / `crosses` as needed (`None` when not used).
3. Re-run `cargo test`, `cargo test --test validation -- --include-ignored`, and `cargo test --test bnf_compliance` with `SYSML_V2_RELEASE_DIR` set.
4. If you snapshot AST text, refresh fixtures after bumping.

[0.17.0]: https://github.com/elan8/sysml-v2-parser/compare/v0.16.0...v0.17.0

## [0.16.0] - 2026-06-03

### Added

- **Requirement body actors**: `RequirementActorDecl` and `actor_decl` in requirement definition bodies (anonymous `actor : Type;` mirrors existing `subject : Type;`).
- **Enumeration usages in part bodies**: `EnumerationUsage`, `enum_usage` parser, and `PartDefBodyElement::EnumerationUsage` / `PartUsageBodyElement::EnumerationUsage` for `enum name : Type;` inside part definitions and usages.
- **Part definition members**: `ItemUsage` and `CalcUsage` in `part_def_body_element` (library-style `item` / `calc` usages in part defs).
- **Diagnostics taxonomy** ([`src/parser/mod.rs`](src/parser/mod.rs)): `DiagnosticCategory`, `DiagnosticSeverity` on `ParseError`; classification for invalid requirement short names (`id '…'`), bare features in part defs, invalid typing operators, and related recovery codes.
- **Editor-oriented post-processing**: cascade suppression (`recovery_cascade_suppressed`), deduplication by specificity, and `suppress_redundant_closing_brace_errors` when a line already reports an invalid `{…}` statement block.
- **Corpus-oriented checks**: `collect_implicit_attribute_in_part_def_warnings`, `collect_requirement_id_dialect_diagnostics`; Apollo regression test [`tests/apollo_regressions.rs`](tests/apollo_regressions.rs).
- **Recovery fixtures/tests**: anonymous actor in requirement, enum in part def, calc usage in part def, bare feature hint, nested part-def typed usages, requirement `id` dialect hint; glued `}package` now expected to parse cleanly.

### Changed

- **`parse_with_diagnostics`**: no longer emits `missing_statement_separator_between_members` for valid glued `}package` boundaries; stricter trailing-`}` handling at root with `unexpected_closing_brace` where appropriate.
- **Recovery**: `missing_member_name` skips anonymous `subject` / `actor` before `:` only in `"requirement body"` scope (use case `actor:` without a name still diagnosed).
- **`part_usage_body_element`**: nested `alt` to stay within nom tuple limits after new enum arm.

### Fixed

- **False positives** on spec-aligned models: `missing_member_name` on `actor : Type` in requirement bodies; `unexpected_keyword_in_scope` for `enum` in part defs; bogus separator errors at `}package`.
- **SurveillanceDrone-errors** validation expectations aligned with multi-package recovery (four root packages, three member-level errors).

### Migration (Spec42 and similar hosts)

1. Bump to `sysml-v2-parser` `0.16.0` (crates.io or tag `v0.16.0`).
2. Match on `RequirementDefBodyElement::RequirementActorDecl` (not a separate top-level `ActorDecl` in requirement bodies — use case `ActorDecl` remains distinct).
3. Handle `PartDefBodyElement::EnumerationUsage` and `PartUsageBodyElement::EnumerationUsage` in graph builders (or ignore like other usage members).
4. Remove handling for diagnostic code `missing_statement_separator_between_members` if you branched on it.
5. Re-run `cargo test` and validation fixtures after bumping.

[0.16.0]: https://github.com/elan8/sysml-v2-parser/compare/v0.15.0...v0.16.0

## [0.15.0] - 2026-06-03

### Breaking

- **`PortBody`**: removed variant `BraceWithPorts { elements: Vec<Node<PortUsage>> }`. Nested port bodies now use `PortBody::Brace { elements: Vec<Node<PortBodyElement>> }` with structured members (`PortUsage`, `InOutDecl`, `Error`, `Other`). Update exhaustive matches and any code that assumed nested ports were only `PortUsage` nodes.
- **`AttributeBody`**: brace bodies are now `AttributeBody::Brace { elements: Vec<Node<AttributeBodyElement>> }` instead of an opaque skipped brace. Members include nested attributes, doc comments, annotations, and recovery `Error` nodes.
- **`DefinitionBody` / `RenderingDefBody`**: generic definition and rendering definition brace bodies now expose structured `DefinitionBodyElement` / `RenderingDefBodyElement` lists (doc, occurrence members, recovery errors) rather than opaque skipped content for occurrence, rendering, flow, allocation, and metadata families.

### Added

- **BNF compliance gate (100% `implemented`)**: machine-readable map [`docs/bnf_coverage.map`](docs/bnf_coverage.map) and [`tests/bnf_compliance.rs`](tests/bnf_compliance.rs) classify all 640 SysML/KerML textual productions; new tests assert zero `partial` map rules and full production coverage. See [`docs/BNF_COVERAGE.md`](docs/BNF_COVERAGE.md) and [`docs/BNF_COMPLIANCE_MATRIX.md`](docs/BNF_COMPLIANCE_MATRIX.md).
- **Shared usage grammar** ([`src/parser/usage.rs`](src/parser/usage.rs)): `usage_header`, `feature_usage_header`, `specialization_clauses`, `subsetting` / `redefinition`, plus `references` (`::>`) and `crosses` (`=>`) operators; supports `defined by`, `typed by`, conjugated types (`~`), and multiple specialization clauses (last-wins where the AST stores a single target).
- **Structured body parsing** ([`src/parser/body.rs`](src/parser/body.rs)): `parse_structured_brace_members` and `advance_to_closing_brace` replace opaque `skip_until_brace_end` in many high-traffic modules (attribute, part, port, occurrence, rendering, flow, allocation, metadata, connection, interface, import, alias, enumeration, constraint, use case).
- **Expression surface** ([`src/parser/expr.rs`](src/parser/expr.rs)): `select` (`.?`), `collect` (`.**`), and parenthesized sequence expressions; precedence-aware binary/unary chain unchanged as the main `expression()` entry point.
- **BNF surface helpers** ([`src/parser/bnf_surface.rs`](src/parser/bnf_surface.rs)): shared entry points and unit tests for lexical terminals, empty productions, and usage/definition declaration fragments.
- **Lexical operators** ([`src/parser/lex.rs`](src/parser/lex.rs)): `references_operator`, `crosses_operator`, `decimal_value_text`, `string_value`, plus lexical BNF unit tests.
- **Action control nodes**: action definition bodies recognize `accept`, `decision`, `fork`, `join`, `send`, `terminate`, `while`, and `if` starters as control-node action usages.
- **CI**: workflow fetches the pinned SysML v2 release tree and runs `cargo test` with `SYSML_V2_RELEASE_DIR` so the BNF gate and default tests run against normative fixtures on every push.
- **Docs**: updated [`docs/SYSML_V2_COMPLIANCE_GAP.md`](docs/SYSML_V2_COMPLIANCE_GAP.md), [`docs/PARSER_TECHNICAL_DEBT.md`](docs/PARSER_TECHNICAL_DEBT.md), and validation README/snapshots for structured parsing regressions.

### Changed

- **Part and port definitions/usages**: brace bodies parse structured member AST with recovery (`PartDefBody`, `PortDefBody`, `PortBodyElement`) instead of swallowing inner grammar.
- **Action and state definitions**: definition-level bodies use structured member loops with `skip_statement_or_block` recovery (no `skip_until_brace_end` on promoted top-level defs guarded by `bnf_compliance`).
- **Requirement, case, view, and usage families**: migrated to shared `usage_header` / `feature_usage_header` where applicable (requirement/case/analysis/verification/action/state/view/rendering/viewpoint/use-case usages, concern usage, calc definition prefix).
- **Specialization targets**: subsetting and redefinition accept qualified names with dotted feature chains and comma-separated target lists.
- **Validation tests**: `parts_tree_1a`, `parts_interconnection_2a`, `function_based_behavior_3a`, and `functional_allocation_4a` refactored to snapshot-based checks aligned with structured AST shapes.

### Fixed

- **Port nested bodies**: `port` usages inside `port` brace bodies (e.g. left/right redefinitions) parse into `PortBodyElement::PortUsage` instead of a separate `BraceWithPorts` shape.
- **Library typing headers**: `defined by` and `typed by` accepted alongside `:` on usage headers; multiple `:>` / `:>>` / `subsets` / `redefines` clauses parse without spurious recovery on common stdlib patterns.
- **Part `ref` lines**: optional comments and formatting around `ref part` assignments tolerated in part usage bodies.

### Migration (Spec42 and similar hosts)

1. Bump the `sysml-v2-parser` dependency to `0.15.0` (or the matching git revision / path).
2. Replace `PortBody::BraceWithPorts` matches with `PortBody::Brace` and handle `PortBodyElement` (nested ports are `PortBodyElement::PortUsage`).
3. If you read attribute or generic definition brace bodies as opaque text, switch to iterating `AttributeBodyElement` / `DefinitionBodyElement` (or keep using span recovery `Error` / `Other` members for unsupported inner forms).
4. For usage typing and specialization, prefer `usage_header` semantics: `references` / `crosses` may appear in the same clause stream as subsets (stored via the shared specialization path where the AST has a single subsets slot).
5. Run `cargo test`, `cargo test --test bnf_compliance`, and `cargo test --test validation -- --include-ignored` with `SYSML_V2_RELEASE_DIR` pointing at the release BNF tree.

[0.15.0]: https://github.com/elan8/sysml-v2-parser/compare/v0.14.0...v0.15.0

## [0.14.0] - 2026-06-02

### Added

- **Qualified package identifiers**: package and namespace declarations now accept qualified names in the identification position (e.g. `package AstronomyReference::Domain { ... }`) and keep the full qualified path in the AST.
- **`ref part` assignment forms**: part usage bodies now parse `ref part` declarations with optional typing and optional value binding (e.g. `ref part centralBody = sun;`, `ref part orbitingBody : Body = earth;`) without recovery diagnostics.

### Fixed

- **Reference usage grammar coverage**: `ref part` declarations that omit explicit typing are no longer forced into a `:` parse path, aligning parser behavior with SysML v2 reference-usage notation.

### Migration (Spec42 and similar hosts)

1. Bump the `sysml-v2-parser` dependency to `0.14.0` (or the matching git revision / path).
2. If downstream code assumes `package`/`namespace` names are unqualified, update it to handle `::`-qualified identifiers in `Identification.name`.
3. Re-run parser and semantic smoke tests that cover `ref part` declarations with and without type annotations.

[0.14.0]: https://github.com/elan8/sysml-v2-parser/compare/v0.13.0...v0.14.0

## [0.13.0] - 2026-06-01

### Breaking

- **Definition subclassification on AST nodes**: many `*Def` types now include `specializes: Option<String>` and `specializes_span: Option<Span>` when a declaration uses `:>` / `specializes` or a library-style typed header before subclassification (e.g. `abstract connection name : Connection[0..*] :> linkObjects, parts`). Affected types include (among others) `ItemDef`, `IndividualDef`, `InterfaceDef`, `ConnectionDef`, `PortDef`, `RequirementDef`, `ConstraintDef`, `StateDef`, `ActionDef`, `FlowDef`, `AllocationDef`, `MetadataDef`, `OccurrenceDef`, `EnumDef`, and the case/view/use-case definition families. Any manual struct literals or exhaustive construction must set these fields (`None` when absent).

### Added

- **Shared definition prelude** ([`src/parser/definition_prefix.rs`](src/parser/definition_prefix.rs)): `parse_definition_prefix` with `DefinitionPrefixOptions` centralizes `abstract`, optional `private`, optional `#` annotation, keyword/`def`, and header-after-ident parsing for migrated definition parsers.
- **Shared opaque body terminator** ([`src/parser/body.rs`](src/parser/body.rs)): `semicolon_or_opaque_brace_body` for `;` or brace bodies whose inner content is skipped (`flow`, `allocation`, `metadata`, and related usages).
- **Header helper** ([`src/parser/specialization.rs`](src/parser/specialization.rs)): `parse_optional_definition_header_after_identification` handles direct `:>` / `specializes` and typed headers (`: Type[multiplicity] … :> bases`) after `identification`.
- **Docs**: [`docs/PARSER_TECHNICAL_DEBT.md`](docs/PARSER_TECHNICAL_DEBT.md) and [`docs/PARSER_DEBT_P1_PLAN.md`](docs/PARSER_DEBT_P1_PLAN.md) document parser duplication, P1 consolidation (complete), and follow-up P2/P3 work.

### Changed

- **Internal refactor (P1)**: eighteen `*_def` entry points (item, individual, interface, metadata, connection, constraint, port, requirement, state, occurrence, flow, allocation, case/analysis/verification, view/viewpoint/rendering, use case, enum, action) delegate their prelude to `parse_definition_prefix`. `part_def`, `calc_def`, usages, `alias_def`, and `dependency` remain on local preludes by design.
- **Numeric literals**: decimal and scientific-notation forms are parsed more consistently in expression paths.

### Fixed

- **Systems / full library gates**: declarations such as `abstract connection … : Type[…] :> …` and `private abstract constraint def …` map to dedicated definition nodes again (`ExtendedLibraryDecl` count stays at zero with `cargo test -- --include-ignored`).
- **Calc and constraint bodies**: `return` expressions in calculation definitions and constraint bodies parse without swallowing following members.
- **Definition prefix modifier order**: `private` is accepted before `abstract` (stdlib `private abstract constraint def`).

### Migration (Spec42 and similar hosts)

1. Bump the `sysml-v2-parser` dependency to `0.13.0` (or the matching git revision / path).
2. Update any manual `*Def` struct literals to include `specializes` and `specializes_span` (use `None` when not modeled).
3. When building semantics from definitions, read `specializes` / `specializes_span` for subclassification edges; typed library headers populate `specializes` from the `:>` clause after the skipped typing fragment.
4. Re-run `cargo test --test validation -- --include-ignored` after upgrading.

[0.13.0]: https://github.com/elan8/sysml-v2-parser/compare/v0.12.0...v0.13.0

## [0.12.0] - 2026-05-28

### Breaking

- **`AttributeUsage`**: added `typing: Option<String>` and `typing_span: Option<Span>` for the type after `:` or `:>` on attribute usages (e.g. `attribute totalMassKg : MassValue`). Any struct literals or manual construction of `AttributeUsage` must set these fields (use `None` when untyped).

### Fixed

- **Typed attribute usages in usage bodies**: `attribute` name followed by `:` or `:>` and a qualified type name now parses as `AttributeUsage` with `typing` populated, including inside `part` usage bodies. Previously the parser rejected this form in usage contexts (recovery / wrong classification). This matches OMG SysML v2 `AttributeUsage = UsagePrefix 'attribute' Usage`, where typing is part of the usage, not only of `attribute def`.
- **Attribute def vs usage disambiguation**: in definition bodies (`part def`, `port def`, `requirement def`), the parser tries `attribute def` before `attribute usage` so typed declaration members such as `attribute mass :> ISQ::mass` remain `AttributeDef`. Untyped value assignments (`attribute actualMass = measuredMass`), `redefines` / `:>>` forms, and prefix redefinitions (`attribute :>> propellantMass = …`) still parse as `AttributeUsage`. Package- and use-case-level attributes are unchanged (`attribute x = expr` stays `AttributeDef`). Fixes validation fixture `1a-Parts Tree.sysml` and similar library models.
- **`:>` vs `:>>` on attributes**: attribute typing no longer treats `:>>` as a `:>` prefix. Prefix-redefine usages (`attribute :>> currentTime : TimeInstantValue`) accept an optional `: Type` after the redefine target; a following `:>` is left for subsetting (e.g. `attribute :>> outlet :> electricGrid.outlets`). `attribute def` requires a declared name so bare `attribute :>> …` is not misclassified.

### Migration (Spec42 and similar hosts)

1. Bump the `sysml-v2-parser` dependency to `0.12.0` (or the matching git revision).
2. Update `AttributeUsage` struct literals to include `typing` and `typing_span`.
3. When building semantics from attribute usages, read `AttributeUsage::typing` for type edges in **usage** bodies (e.g. nested `part` usages).
4. In **definition** bodies, typed members without `def` (e.g. `attribute massActual: MassValue` in `requirement def`) continue to surface as `AttributeDef`; do not assume every typed `attribute` is an `AttributeUsage`.
5. Re-run `cargo test --test validation -- --include-ignored` after upgrading; the full validation and std-library gates should be green.

[0.12.0]: https://github.com/elan8/sysml-v2-parser/compare/v0.11.0...v0.12.0

## [0.11.0] - 2026-05-28

### Breaking

- **`UseCaseDefBodyElement`**: added new variant `AttributeDef(Node<AttributeDef>)` so that `attribute` definitions inside a `use case def` body are surfaced in the AST. Any exhaustive `match` on `UseCaseDefBodyElement` must add an arm for `AttributeDef`.

### Fixed

- **Transition names vs transition keywords**: optional transition names such as `docked` are no longer dropped when the name shares a prefix with `first`, `if`, `do`, or `then`. The parser now uses whole-keyword detection (`starts_with_keyword`) so `transition docked first docking then charging;` parses correctly.

### Migration (Spec42 and similar hosts)

1. Bump the `sysml-v2-parser` dependency to `0.11.0` (or the matching git revision).
2. If you exhaustively match on `UseCaseDefBodyElement`, add an arm for the new `AttributeDef` variant (carry-through is usually identical to the existing `AttributeDef` arms on `PartDefBodyElement` or `RequirementDefBodyElement`).

[0.11.0]: https://github.com/elan8/sysml-v2-parser/compare/v0.10.0...v0.11.0

## [0.10.0] - 2026-05-13

### Breaking

- **`PartDefBodyElement`**: added new variant `InterfaceDef(Node<InterfaceDef>)` so that nested `interface def` declarations inside a `part def` body are surfaced in the AST. Any exhaustive `match` on `PartDefBodyElement` must add an arm for `InterfaceDef`.
- **`parse_root` strict mode**: a stray trailing `}` after a well-formed root namespace is now reported as `unexpected_closing_brace` instead of being silently accepted. Inputs that previously parsed under `parse_root` but contained extra closing braces will now return an error (these inputs already produced diagnostics from `parse_with_diagnostics`).

### Added

- **Nested `interface def` in part definitions**: `part def` bodies now accept nested `interface def` (and continue to accept `interface` usages), matching the OMG SysML v2 Part 1 textual grammar which lists `InterfaceDefinition` under `DefinitionElement`. New fixtures cover the nested form and assert no recovery diagnostics.
- **Diagnostic code `invalid_bare_identifier_in_action_body` / `invalid_bare_identifier_in_state_body`**: bare identifiers in action and state bodies (e.g. `action a { batCap; maxBatCap; }`) now produce a targeted message naming valid forms (`perform`, `bind`, `in`/`out`, `entry`, `transition`, `then`, …) instead of the generic `unexpected_keyword_in_scope`.
- **Diagnostic code `recovery_cascade_suppressed`**: after three consecutive `missing_semicolon` or `recovered_*` diagnostics in the same body region, a single warning-severity summary replaces the remaining cascade entries, pointing back to the first error to fix.
- **Diagnostic code `recovered_root_body`**: when a root-level `package` / `library` / `standard package` / `namespace` body fails to parse, the recovery path emits one root-scoped error and skips to the next root element, preventing cascades across unrelated definitions in the same file.
- **Docs**: new `docs/CORPUS_MBSE_VACUUM_PARSER_SPEC42_FEEDBACK.md` capturing findings from running the parser/Spec42 stack against the public MBSE vacuum-cleaner robot example, plus a documentation index in `README.md`.

### Fixed

- **`interface` usage with no whitespace before `:`**: `interface : Foo;` (and similar forms without a space between the keyword and the colon) is now accepted.
- **`comment` annotation prefixes**: `comment` annotations tolerate arbitrary tokens between the optional name/about clauses and the opening `/* … */` comment body, matching real-world inputs that include extra metadata before the comment.
- **Part / state body recovery**: classification codes `invalid_bare_identifier_in_action_body`, `invalid_bare_identifier_in_state_body`, `unexpected_keyword_in_scope`, `missing_semicolon`, and `missing_body_or_semicolon` now produce `Other` placeholder elements in `PartDefBody` / `StateDefBody` so downstream tooling can see the skipped span.

### Reliability

- Cascade suppression and the root-body recovery error together significantly reduce diagnostic volume on large real-world corpora where a single structural error previously fanned out into hundreds of follow-up `missing_semicolon` / `recovered_*` entries.

### Migration (Spec42 and similar hosts)

1. Bump the `sysml-v2-parser` dependency to `0.10.0` (or the matching git revision).
2. If you exhaustively match on `PartDefBodyElement`, add an arm for the new `InterfaceDef` variant (carry-through is usually identical to the existing `InterfaceUsage` arm).
3. Diagnostic consumers can opt to treat `recovery_cascade_suppressed` as informational (it carries `severity: Warning`) and to display `recovered_root_body` as the primary error for affected root scopes.

[0.10.0]: https://github.com/elan8/sysml-v2-parser/compare/v0.9.0...v0.10.0

## [0.9.0] - 2026-05-04

### Breaking

- **`AttributeDef`**: added optional field `value: Option<Node<Expression>>` for default / value parts after `=`, `:=`, or `default =` on attribute definitions (e.g. `attribute n: Integer = 0;`). Update any exhaustive matches or struct literals that construct `AttributeDef`.
- **Expression `Span` for parenthesized grouping**: a single expression in parentheses `( expr )` now uses a node span covering the full `(` … `)` in the source (not only the inner expression). Tools that slice source by `Span` (e.g. joining `require constraint` text) may see different byte ranges than in 0.8.x for the same logical tree.
- **Numeric literal parsing**: `literal_only` tries `literal_real` before `literal_integer`, so decimals such as `0.9` parse as reals instead of integer `0` with a stray `.9`. Rare integer-vs-real edge cases in malformed or unusual inputs may produce a different AST than before.

### Fixed

- **Quantity literals**: bracket units such as `[m/s]` or library-style names with `::` inside `[` … `]` parse more reliably into `LiteralWithUnit`.
- **Constraint and calc brace bodies**: optional terminating `;` after each body item is accepted, so chained expressions split with `;` (e.g. `(a <= b); and (c <= d);`) map to multiple `Expression` elements instead of falling through to `Other`.
- **Recovery**: `inout` is included in constraint/calc body recovery keyword lists alongside `in` / `out`.

### Reliability

- Slightly longer preview text for `Other` placeholders in constraint/calc recovery paths (diagnostics).

### Migration (Spec42 and similar hosts)

1. Bump the `sysml-v2-parser` dependency to `0.9.0` (or the matching git revision).
2. Add `value: None` (or the parsed value) wherever you construct `AttributeDef` manually; re-run tests that assert on expression source spans inside parentheses or on joined constraint text.

**Local smoke (optional):** In a Spec42 checkout, add to `.cargo/config.toml` a `[patch."https://github.com/elan8/sysml-v2-parser"]` entry with `sysml-v2-parser = { path = "../sysml-v2-parser" }`, then run `cargo update -p sysml-v2-parser` and `cargo check -p kernel`. Remove the patch afterward unless you intend to keep developing against a local parser build.

[0.9.0]: https://github.com/elan8/sysml-v2-parser/compare/v0.8.0...v0.9.0
