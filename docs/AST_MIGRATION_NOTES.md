# AST Migration Notes

## 2026-03 library-coverage update

To support strict parsing of the full SysML/KerML library set without top-level recovery noise, the package-body AST gained one additive variant:

- `PackageBodyElement::GenericDecl(Node<GenericDecl>)`

Where:

- `GenericDecl.text` stores the original declaration text fragment that was recognized and consumed by the generic declaration fallback.

### Compatibility impact

- This is an additive enum variant, so existing exhaustive matches over `PackageBodyElement` must add a `GenericDecl` branch.
- Existing node shapes for all previously supported dedicated constructs remain unchanged.
- Diagnostics behavior changed for unsupported package-level declarations: many declarations that previously produced `recovered_package_body_element` diagnostics are now represented as `GenericDecl` and no longer emit that recovery diagnostic.

### Suggested downstream handling (Spec42)

- Treat `GenericDecl` as a non-fatal declaration node.
- For indexing/symbol extraction pipelines, either:
  - skip `GenericDecl` nodes, or
  - parse `GenericDecl.text` with downstream heuristics if partial symbol extraction is desired.
