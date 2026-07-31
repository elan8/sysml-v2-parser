//! Parser test for `02-Parts Interconnection/2a-Parts Interconnection.sysml`.

use sysml_v2_parser::ast::{
    Bind, Connect, ConnectBody, ConnectStmt, EndDecl, Expression, Identification, Import,
    InterfaceDef, InterfaceDefBody, InterfaceDefBodyElement, InterfaceUsage,
    InterfaceUsageBodyElement, Membership, Multiplicity, Node, Package, PackageBody,
    PackageBodyElement, PartDef, PartDefBody, PartDefBodyElement, PartUsage, PartUsageBody,
    PartUsageBodyElement, PortBody, PortBodyElement, PortDef, PortDefBody, PortDefBodyElement,
    PortUsage, RefBody, RefDecl, RelationshipTarget, RelationshipTargetSegment, RootElement,
    RootNamespace, SegmentSeparator, Span, SubsettingKind, SubsettingRelationship, TypingKind,
    TypingRelationship, Visibility,
};
use sysml_v2_parser::parse;

/// Bare bracket multiplicity `[n]`, e.g. `mult(2)` for `[2]` (lower == upper == n).
fn mult(v: i64) -> Node<Multiplicity> {
    let bound = Some(Box::new(n(Expression::LiteralInteger(v))));
    n(Multiplicity {
        lower: bound.clone(),
        upper: bound,
        span: Span::dummy(),
    })
}

/// Build a single [`RelationshipTarget`] from a `::`/`.`-joined display string, e.g.
/// `"Vehicle::mass.value"` -> `[Vehicle, ::mass, .value]`, mirroring how the real parser splits
/// a target into segments (all `::`-segments first, then any `.`-segments after).
fn relationship_target(text: &str) -> Node<RelationshipTarget> {
    let mut parts = text.splitn(2, '.');
    let qualified = parts.next().unwrap_or_default();
    let dotted_rest = parts.next();
    let mut segments: Vec<RelationshipTargetSegment> = qualified
        .split("::")
        .enumerate()
        .map(|(i, name)| RelationshipTargetSegment {
            name: name.to_string(),
            separator: if i == 0 {
                None
            } else {
                Some(SegmentSeparator::ColonColon)
            },
        })
        .collect();
    if let Some(rest) = dotted_rest {
        segments.extend(rest.split('.').map(|name| RelationshipTargetSegment {
            name: name.to_string(),
            separator: Some(SegmentSeparator::Dot),
        }));
    }
    n(RelationshipTarget {
        segments,
        span: Span::dummy(),
    })
}

/// `:> target` subclassification relationship, e.g. `spec("Axle")` for `:> Axle`.
fn spec(target: &str) -> Node<TypingRelationship> {
    n(TypingRelationship {
        target: vec![relationship_target(target)],
        kind: TypingKind::Subclassification,
        span: Span::dummy(),
        is_conjugated: false,
        is_implied: false,
    })
}

/// `: target` typing relationship for `PartUsage.typing`, e.g. `typed("Engine")` for `: Engine`.
fn typed(target: &str) -> Node<TypingRelationship> {
    n(TypingRelationship {
        target: vec![relationship_target(target)],
        kind: TypingKind::Typing,
        span: Span::dummy(),
        is_conjugated: false,
        is_implied: false,
    })
}

/// `:>>` / `redefines` target, e.g. `redef("cylinders")` for `redefines cylinders`.
fn redef(target: &str) -> Node<SubsettingRelationship> {
    n(SubsettingRelationship {
        target: vec![relationship_target(target)],
        kind: SubsettingKind::Redefines,
        span: Span::dummy(),
        is_implied: false,
    })
}

/// `:>` / `subsets` target, e.g. `subs("wheelToRoadPort")` for `subsets wheelToRoadPort`.
fn subs(target: &str) -> Node<SubsettingRelationship> {
    n(SubsettingRelationship {
        target: vec![relationship_target(target)],
        kind: SubsettingKind::Subsets,
        span: Span::dummy(),
        is_implied: false,
    })
}

fn id(name: &str) -> Identification {
    Identification {
        short_name: None,
        name: Some(name.to_string()),
    }
}

fn n<T>(v: T) -> Node<T> {
    Node::new(Span::dummy(), v)
}

/// This fixture's source has no `private`/`protected`/`public` prefixes anywhere, so every
/// `PartDef`/`PartUsage` membership below is the no-explicit-visibility default -- see
/// `Membership::owning`/`Membership::feature` (parser work item 4b).
fn owning_membership() -> sysml_v2_parser::ast::Membership {
    sysml_v2_parser::ast::Membership::owning(None, Span::dummy())
}

fn feature_membership() -> sysml_v2_parser::ast::Membership {
    sysml_v2_parser::ast::Membership::feature(None, Span::dummy())
}

/// Both `import`s in this fixture's source are written `public import ...`.
fn public_import_membership() -> sysml_v2_parser::ast::Membership {
    sysml_v2_parser::ast::Membership::new(
        sysml_v2_parser::ast::MembershipKind::Import,
        Some(Visibility::Public),
        Span::dummy(),
    )
}

/// Path expression from dot-separated path (e.g. "engine.fuelCmdPort"), for endpoints parsed via
/// `path_expression` (`src/parser/expr.rs`: bind/connect/allocate/interface-connect lhs/rhs). A
/// single segment stays `FeatureRef`; a genuine multi-segment dotted chain becomes
/// `FeatureChainRef` (PAR-005 item 3).
fn expr_path(path: &str) -> Node<Expression> {
    let segments: Vec<String> = path.split('.').map(str::to_string).collect();
    let expr = if segments.len() == 1 {
        Expression::FeatureRef(segments.into_iter().next().unwrap())
    } else {
        Expression::FeatureChainRef(sysml_v2_parser::ast::FeatureChain {
            segments,
            span: Span::dummy(),
        })
    };
    n(expr)
}

/// Dot-separated path built as nested `MemberAccess` (not `FeatureChainRef`), for the (rarer)
/// call sites parsed via the general `expression()` grammar rather than `path_expression` --
/// e.g. `ref :>> name = value;` (`interface_usage_ref_redef`, `src/parser/part/usage.rs`) parses
/// `value` with `expression`, whose postfix `.` chaining still folds into `MemberAccess` (PAR-005
/// only adopted `FeatureChain` for `path_expression`, not the general postfix chain -- see
/// `src/parser/expr.rs` module notes).
fn expr_member_chain(path: &str) -> Node<Expression> {
    let segments: Vec<&str> = path.split('.').collect();
    let mut expr = Expression::FeatureRef(segments[0].to_string());
    for seg in segments.iter().skip(1) {
        expr = Expression::MemberAccess(Box::new(n(expr)), (*seg).to_string());
    }
    n(expr)
}

/// Wraps a `Node<Expression>` endpoint in a `Node<ConnectionEnd>` for `Connect`/`ConnectStmt`.
fn end(expr: Node<Expression>) -> Node<sysml_v2_parser::ast::ConnectionEnd> {
    let span = expr.span.clone();
    n(sysml_v2_parser::ast::ConnectionEnd {
        expression: expr,
        multiplicity: None,
        span,
    })
}

/// Index expression base#(n).
fn expr_index(base: &str, index_val: i64) -> Node<Expression> {
    n(Expression::Index {
        base: Box::new(n(Expression::FeatureRef(base.to_string()))),
        index: Box::new(n(Expression::LiteralInteger(index_val))),
    })
}

/// Expected AST for `2a-Parts Interconnection.sysml`.
fn expected_ast() -> RootNamespace {
    RootNamespace {
        elements: vec![n(RootElement::Package(n(Package {
            identification: id("2a-Parts Interconnection"),
            body: PackageBody::Brace {
                elements: vec![
                    n(PackageBodyElement::Import(n(Import {
                        membership: public_import_membership(),
                        is_import_all: true,
                        target: "Definitions::*".to_string(),
                        target_span: Span::dummy(),
                        is_recursive: false,
                        filter_members: None,
                    }))),
                    n(PackageBodyElement::Import(n(Import {
                        membership: public_import_membership(),
                        is_import_all: true,
                        target: "Usages::*".to_string(),
                        target_span: Span::dummy(),
                        is_recursive: false,
                        filter_members: None,
                    }))),
                    n(PackageBodyElement::Package(n(definitions_package()))),
                    n(PackageBodyElement::Package(n(usages_package()))),
                ],
            },
        })))],
    }
}

fn definitions_package() -> Package {
    Package {
        identification: id("Definitions"),
        body: PackageBody::Brace {
            elements: vec![
                n(port_def_semicolon("FuelCmdPort")),
                n(port_def_semicolon("DrivePwrPort")),
                n(port_def_semicolon("ClutchPort")),
                n(port_def_semicolon("ShaftPort_a")),
                n(port_def_semicolon("ShaftPort_b")),
                n(port_def_semicolon("ShaftPort_c")),
                n(port_def_semicolon("ShaftPort_d")),
                n(port_def_semicolon("DiffPort")),
                n(port_def_semicolon("AxlePort")),
                n(port_def_semicolon("AxleToWheelPort")),
                n(port_def_semicolon("WheelToAxlePort")),
                n(port_def_semicolon("WheelToRoadPort")),
                n(port_def_vehicle_to_road()),
                n(part_def_vehicle_a()),
                n(PackageBodyElement::PartDef(n(PartDef {
                    is_individual: false,
                    definition_prefix: None,
                    identification: id("AxleAssembly"),
                    specializes: None,
                    body: PartDefBody::Semicolon,
                    membership: owning_membership(),
                }))),
                n(part_def_rear_axle_assembly()),
                n(PackageBodyElement::PartDef(n(PartDef {
                    is_individual: false,
                    definition_prefix: None,
                    identification: id("Axle"),
                    specializes: None,
                    body: PartDefBody::Semicolon,
                    membership: owning_membership(),
                }))),
                n(PackageBodyElement::PartDef(n(PartDef {
                    is_individual: false,
                    definition_prefix: None,
                    identification: id("RearAxle"),
                    specializes: Some(spec("Axle")),
                    body: PartDefBody::Semicolon,
                    membership: owning_membership(),
                }))),
                n(part_def_half_axle()),
                n(part_def_engine()),
                n(part_def_transmission()),
                n(part_def_driveshaft()),
                n(PackageBodyElement::PartDef(n(PartDef {
                    is_individual: false,
                    definition_prefix: None,
                    identification: id("Differential"),
                    specializes: None,
                    body: PartDefBody::Brace { elements: vec![] },
                    membership: owning_membership(),
                }))),
                n(PackageBodyElement::PartDef(n(PartDef {
                    is_individual: false,
                    definition_prefix: None,
                    identification: id("Wheel"),
                    specializes: None,
                    body: PartDefBody::Semicolon,
                    membership: owning_membership(),
                }))),
                n(interface_def_engine_to_transmission()),
                n(interface_def_driveshaft()),
            ],
        },
    }
}

fn port_def_semicolon(name: &str) -> PackageBodyElement {
    PackageBodyElement::PortDef(n(PortDef {
        identification: id(name),
        specializes: None,
        body: PortDefBody::Semicolon,
        membership: owning_membership(),
    }))
}

fn port_def_vehicle_to_road() -> PackageBodyElement {
    PackageBodyElement::PortDef(n(PortDef {
        identification: id("VehicleToRoadPort"),
        specializes: None,
        body: PortDefBody::Brace {
            elements: vec![n(PortDefBodyElement::PortUsage(n(PortUsage {
                is_abstract: false,
                direction: None,
                is_derived: false,
                is_constant: false,
                name: "wheelToRoadPort".to_string(),
                short_name: None,
                type_name: Some("WheelToRoadPort".to_string()),
                multiplicity: Some(mult(2)),
                subsets: None,
                redefines: None,
                references: None,
                crosses: None,
                intersects: None,
                value: None,
                body: PortBody::Semicolon,
                name_span: None,
                type_ref_span: None,
                membership: feature_membership(),
            })))],
        },
        membership: owning_membership(),
    }))
}

fn part_def_vehicle_a() -> PackageBodyElement {
    PackageBodyElement::PartDef(n(PartDef {
        is_individual: false,
        definition_prefix: None,
        identification: id("VehicleA"),
        specializes: None,
        body: PartDefBody::Brace {
            elements: vec![
                n(PartDefBodyElement::PortUsage(n(PortUsage {
                    is_abstract: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "fuelCmdPort".to_string(),
                    short_name: None,
                    type_name: Some("FuelCmdPort".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    references: None,
                    crosses: None,
                    intersects: None,
                    value: None,
                    body: PortBody::Semicolon,
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                }))),
                n(PartDefBodyElement::PortUsage(n(PortUsage {
                    is_abstract: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "vehicleToRoadPort".to_string(),
                    short_name: None,
                    type_name: Some("VehicleToRoadPort".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    references: None,
                    crosses: None,
                    intersects: None,
                    value: None,
                    body: PortBody::Semicolon,
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                }))),
            ],
        },
        membership: owning_membership(),
    }))
}

fn part_def_rear_axle_assembly() -> PackageBodyElement {
    PackageBodyElement::PartDef(n(PartDef {
        is_individual: false,
        definition_prefix: None,
        identification: id("RearAxleAssembly"),
        specializes: Some(spec("AxleAssembly")),
        body: PartDefBody::Brace {
            elements: vec![n(PartDefBodyElement::PortUsage(n(PortUsage {
                is_abstract: false,
                direction: None,
                is_derived: false,
                is_constant: false,
                name: "shaftPort_d".to_string(),
                short_name: None,
                type_name: Some("ShaftPort_d".to_string()),
                multiplicity: None,
                subsets: None,
                redefines: None,
                references: None,
                crosses: None,
                intersects: None,
                value: None,
                body: PortBody::Semicolon,
                name_span: None,
                type_ref_span: None,
                membership: feature_membership(),
            })))],
        },
        membership: owning_membership(),
    }))
}

fn part_def_half_axle() -> PackageBodyElement {
    PackageBodyElement::PartDef(n(PartDef {
        is_individual: false,
        definition_prefix: None,
        identification: id("HalfAxle"),
        specializes: None,
        body: PartDefBody::Brace {
            elements: vec![
                n(PartDefBodyElement::PortUsage(n(PortUsage {
                    is_abstract: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "axleToDiffPort".to_string(),
                    short_name: None,
                    type_name: Some("AxlePort".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    references: None,
                    crosses: None,
                    intersects: None,
                    value: None,
                    body: PortBody::Semicolon,
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                }))),
                n(PartDefBodyElement::PortUsage(n(PortUsage {
                    is_abstract: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "axleToWheelPort".to_string(),
                    short_name: None,
                    type_name: Some("AxleToWheelPort".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    references: None,
                    crosses: None,
                    intersects: None,
                    value: None,
                    body: PortBody::Semicolon,
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                }))),
            ],
        },
        membership: owning_membership(),
    }))
}

fn part_def_engine() -> PackageBodyElement {
    PackageBodyElement::PartDef(n(PartDef {
        is_individual: false,
        definition_prefix: None,
        identification: id("Engine"),
        specializes: None,
        body: PartDefBody::Brace {
            elements: vec![
                n(PartDefBodyElement::PortUsage(n(PortUsage {
                    is_abstract: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "fuelCmdPort".to_string(),
                    short_name: None,
                    type_name: Some("FuelCmdPort".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    references: None,
                    crosses: None,
                    intersects: None,
                    value: None,
                    body: PortBody::Semicolon,
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                }))),
                n(PartDefBodyElement::PortUsage(n(PortUsage {
                    is_abstract: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "drivePwrPort".to_string(),
                    short_name: None,
                    type_name: Some("DrivePwrPort".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    references: None,
                    crosses: None,
                    intersects: None,
                    value: None,
                    body: PortBody::Semicolon,
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                }))),
            ],
        },
        membership: owning_membership(),
    }))
}

fn part_def_transmission() -> PackageBodyElement {
    PackageBodyElement::PartDef(n(PartDef {
        is_individual: false,
        definition_prefix: None,
        identification: id("Transmission"),
        specializes: None,
        body: PartDefBody::Brace {
            elements: vec![
                n(PartDefBodyElement::PortUsage(n(PortUsage {
                    is_abstract: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "clutchPort".to_string(),
                    short_name: None,
                    type_name: Some("ClutchPort".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    references: None,
                    crosses: None,
                    intersects: None,
                    value: None,
                    body: PortBody::Semicolon,
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                }))),
                n(PartDefBodyElement::PortUsage(n(PortUsage {
                    is_abstract: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "shaftPort_a".to_string(),
                    short_name: None,
                    type_name: Some("ShaftPort_a".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    references: None,
                    crosses: None,
                    intersects: None,
                    value: None,
                    body: PortBody::Semicolon,
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                }))),
            ],
        },
        membership: owning_membership(),
    }))
}

fn part_def_driveshaft() -> PackageBodyElement {
    PackageBodyElement::PartDef(n(PartDef {
        is_individual: false,
        definition_prefix: None,
        identification: id("Driveshaft"),
        specializes: None,
        body: PartDefBody::Brace {
            elements: vec![
                n(PartDefBodyElement::PortUsage(n(PortUsage {
                    is_abstract: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "shaftPort_b".to_string(),
                    short_name: None,
                    type_name: Some("ShaftPort_b".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    references: None,
                    crosses: None,
                    intersects: None,
                    value: None,
                    body: PortBody::Semicolon,
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                }))),
                n(PartDefBodyElement::PortUsage(n(PortUsage {
                    is_abstract: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "shaftPort_c".to_string(),
                    short_name: None,
                    type_name: Some("ShaftPort_c".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    references: None,
                    crosses: None,
                    intersects: None,
                    value: None,
                    body: PortBody::Semicolon,
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                }))),
            ],
        },
        membership: owning_membership(),
    }))
}

fn interface_def_engine_to_transmission() -> PackageBodyElement {
    PackageBodyElement::InterfaceDef(n(InterfaceDef {
        identification: id("EngineToTransmissionInterface"),
        specializes: None,
        body: InterfaceDefBody::Brace {
            elements: vec![
                n(InterfaceDefBodyElement::EndDecl(n(EndDecl {
                    name: "drivePwrPort".to_string(),
                    type_name: "DrivePwrPort".to_string(),
                    uses_derived_syntax: false,
                    references: None,
                    name_span: None,
                    type_ref_span: None,
                }))),
                n(InterfaceDefBodyElement::EndDecl(n(EndDecl {
                    name: "clutchPort".to_string(),
                    type_name: "ClutchPort".to_string(),
                    uses_derived_syntax: false,
                    references: None,
                    name_span: None,
                    type_ref_span: None,
                }))),
            ],
        },
        membership: owning_membership(),
    }))
}

fn interface_def_driveshaft() -> PackageBodyElement {
    PackageBodyElement::InterfaceDef(n(InterfaceDef {
        identification: id("DriveshaftInterface"),
        specializes: None,
        body: InterfaceDefBody::Brace {
            elements: vec![
                n(InterfaceDefBodyElement::EndDecl(n(EndDecl {
                    name: "shaftPort_a".to_string(),
                    type_name: "ShaftPort_a".to_string(),
                    uses_derived_syntax: false,
                    references: None,
                    name_span: None,
                    type_ref_span: None,
                }))),
                n(InterfaceDefBodyElement::EndDecl(n(EndDecl {
                    name: "shaftPort_d".to_string(),
                    type_name: "ShaftPort_d".to_string(),
                    uses_derived_syntax: false,
                    references: None,
                    name_span: None,
                    type_ref_span: None,
                }))),
                n(InterfaceDefBodyElement::RefDecl(n(RefDecl {
                    name: "driveshaft".to_string(),
                    type_name: "Driveshaft".to_string(),
                    typing: Some(typed("Driveshaft")),
                    redefines: None,
                    value: None,
                    body: RefBody::Brace { elements: vec![] },
                    name_span: None,
                    type_ref_span: None,
                    membership: Membership::feature(None, Span::dummy()),
                }))),
                n(InterfaceDefBodyElement::ConnectStmt(n(ConnectStmt {
                    from: end(n(Expression::FeatureRef("shaftPort_a".to_string()))),
                    to: end(expr_path("driveshaft.shaftPort_b")),
                    extra_ends: vec![],
                    body: ConnectBody::Brace,
                }))),
                n(InterfaceDefBodyElement::ConnectStmt(n(ConnectStmt {
                    from: end(expr_path("driveshaft.shaftPort_c")),
                    to: end(n(Expression::FeatureRef("shaftPort_d".to_string()))),
                    extra_ends: vec![],
                    body: ConnectBody::Semicolon,
                }))),
            ],
        },
        membership: owning_membership(),
    }))
}

fn usages_package() -> Package {
    Package {
        identification: id("Usages"),
        body: PackageBody::Brace {
            elements: vec![n(PackageBodyElement::PartUsage(n(part_vehicle1_c1())))],
        },
    }
}

fn part_vehicle1_c1() -> PartUsage {
    PartUsage {
        usage_prefix: None,
        is_individual: false,
        is_reference: false,
        direction: None,
        is_derived: false,
        is_constant: false,
        name: "vehicle1_c1".to_string(),
        short_name: None,
        type_name: "VehicleA".to_string(),
        typing: Some(typed("VehicleA")),
        multiplicity: None,
        ordered: false,
        subsets: None,
        redefines: None,
        value: None,
        name_span: None,
        type_ref_span: None,
        body: PartUsageBody::Brace {
            elements: vec![
                n(PartUsageBodyElement::Bind(n(Bind {
                    left: expr_path("fuelCmdPort"),
                    right: expr_path("engine.fuelCmdPort"),
                    body: Some(ConnectBody::Semicolon),
                }))),
                n(PartUsageBodyElement::PartUsage(Box::new(n(PartUsage {
                    usage_prefix: None,
                    is_individual: false,
                    is_reference: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "engine".to_string(),
                    short_name: None,
                    type_name: "Engine".to_string(),
                    typing: Some(typed("Engine")),
                    multiplicity: None,
                    ordered: false,
                    subsets: None,
                    redefines: None,
                    value: None,
                    body: PartUsageBody::Semicolon,
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                })))),
                n(PartUsageBodyElement::InterfaceUsage(n(
                    InterfaceUsage::TypedConnect {
                        interface_type: Some("EngineToTransmissionInterface".to_string()),
                        from: expr_path("engine.drivePwrPort"),
                        to: expr_path("transmission.clutchPort"),
                        body: ConnectBody::Brace,
                        body_elements: vec![],
                    },
                ))),
                n(PartUsageBodyElement::PartUsage(Box::new(n(PartUsage {
                    usage_prefix: None,
                    is_individual: false,
                    is_reference: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "transmission".to_string(),
                    short_name: None,
                    type_name: "Transmission".to_string(),
                    typing: Some(typed("Transmission")),
                    multiplicity: None,
                    ordered: false,
                    subsets: None,
                    redefines: None,
                    value: None,
                    body: PartUsageBody::Semicolon,
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                })))),
                n(PartUsageBodyElement::PartUsage(Box::new(n(PartUsage {
                    usage_prefix: None,
                    is_individual: false,
                    is_reference: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "driveshaft".to_string(),
                    short_name: None,
                    type_name: "Driveshaft".to_string(),
                    typing: Some(typed("Driveshaft")),
                    multiplicity: None,
                    ordered: false,
                    subsets: None,
                    redefines: None,
                    value: None,
                    body: PartUsageBody::Brace { elements: vec![] },
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                })))),
                n(PartUsageBodyElement::InterfaceUsage(n(
                    InterfaceUsage::TypedConnect {
                        interface_type: Some("DriveshaftInterface".to_string()),
                        from: expr_path("transmission.shaftPort_a"),
                        to: expr_path("rearAxleAssembly.shaftPort_d"),
                        body: ConnectBody::Brace,
                        body_elements: vec![n(InterfaceUsageBodyElement::RefRedef {
                            name: "driveshaft".to_string(),
                            value: expr_member_chain("vehicle1_c1.driveshaft"),
                            body: RefBody::Brace { elements: vec![] },
                        })],
                    },
                ))),
                n(PartUsageBodyElement::PartUsage(Box::new(n(
                    part_rear_axle_assembly(),
                )))),
                n(PartUsageBodyElement::Bind(n(Bind {
                    left: expr_path("rearAxleAssembly.leftWheel.wheelToRoadPort"),
                    right: expr_path("vehicleToRoadPort.leftWheelToRoadPort"),
                    body: Some(ConnectBody::Semicolon),
                }))),
                n(PartUsageBodyElement::Bind(n(Bind {
                    left: expr_path("rearAxleAssembly.rightWheel.wheelToRoadPort"),
                    right: expr_path("vehicleToRoadPort.rightWheelToRoadPort"),
                    body: Some(ConnectBody::Semicolon),
                }))),
                n(PartUsageBodyElement::PortUsage(n(PortUsage {
                    is_abstract: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "vehicleToRoadPort".to_string(),
                    short_name: None,
                    type_name: None,
                    multiplicity: None,
                    subsets: None,
                    redefines: Some(redef("VehicleA::vehicleToRoadPort")),
                    references: None,
                    crosses: None,
                    intersects: None,
                    value: None,
                    body: PortBody::Brace {
                        elements: vec![
                            n(PortBodyElement::PortUsage(n(PortUsage {
                                is_abstract: false,
                                direction: None,
                                is_derived: false,
                                is_constant: false,
                                name: "leftWheelToRoadPort".to_string(),
                                short_name: None,
                                type_name: None,
                                multiplicity: None,
                                subsets: Some((
                                    subs("wheelToRoadPort"),
                                    Some(expr_index("wheelToRoadPort", 1)),
                                )),
                                redefines: None,
                                references: None,
                                crosses: None,
                                intersects: None,
                                value: None,
                                body: PortBody::Semicolon,
                                name_span: None,
                                type_ref_span: None,
                                membership: feature_membership(),
                            }))),
                            n(PortBodyElement::PortUsage(n(PortUsage {
                                is_abstract: false,
                                direction: None,
                                is_derived: false,
                                is_constant: false,
                                name: "rightWheelToRoadPort".to_string(),
                                short_name: None,
                                type_name: None,
                                multiplicity: None,
                                subsets: Some((
                                    subs("wheelToRoadPort"),
                                    Some(expr_index("wheelToRoadPort", 2)),
                                )),
                                redefines: None,
                                references: None,
                                crosses: None,
                                intersects: None,
                                value: None,
                                body: PortBody::Semicolon,
                                name_span: None,
                                type_ref_span: None,
                                membership: feature_membership(),
                            }))),
                        ],
                    },
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                }))),
            ],
        },
        membership: feature_membership(),
    }
}

fn part_rear_axle_assembly() -> PartUsage {
    PartUsage {
        usage_prefix: None,
        is_individual: false,
        is_reference: false,
        direction: None,
        is_derived: false,
        is_constant: false,
        name: "rearAxleAssembly".to_string(),
        short_name: None,
        type_name: "RearAxleAssembly".to_string(),
        typing: Some(typed("RearAxleAssembly")),
        multiplicity: None,
        ordered: false,
        subsets: None,
        redefines: None,
        value: None,
        name_span: None,
        type_ref_span: None,
        body: PartUsageBody::Brace {
            elements: vec![
                n(PartUsageBodyElement::Bind(n(Bind {
                    left: expr_path("shaftPort_d"),
                    right: expr_path("differential.shaftPort_d"),
                    body: Some(ConnectBody::Semicolon),
                }))),
                n(PartUsageBodyElement::PartUsage(Box::new(n(
                    part_differential(),
                )))),
                n(PartUsageBodyElement::InterfaceUsage(n(
                    InterfaceUsage::Connection {
                        from: expr_path("differential.leftDiffPort"),
                        to: expr_path("rearAxle.leftHalfAxle.axleToDiffPort"),
                        body_elements: vec![],
                    },
                ))),
                n(PartUsageBodyElement::InterfaceUsage(n(
                    InterfaceUsage::Connection {
                        from: expr_path("differential.rightDiffPort"),
                        to: expr_path("rearAxle.rightHalfAxle.axleToDiffPort"),
                        body_elements: vec![],
                    },
                ))),
                n(PartUsageBodyElement::PartUsage(Box::new(n(
                    part_rear_axle(),
                )))),
                n(PartUsageBodyElement::Connect(n(Connect {
                    from: end(expr_path("rearAxle.leftHalfAxle.axleToWheelPort")),
                    to: end(expr_path("leftWheel.wheelToAxlePort")),
                    body: ConnectBody::Semicolon,
                }))),
                n(PartUsageBodyElement::Connect(n(Connect {
                    from: end(expr_path("rearAxle.rightHalfAxle.axleToWheelPort")),
                    to: end(expr_path("rightWheel.wheelToAxlePort")),
                    body: ConnectBody::Semicolon,
                }))),
                n(PartUsageBodyElement::PartUsage(Box::new(n(PartUsage {
                    usage_prefix: None,
                    is_individual: false,
                    is_reference: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "rearWheel".to_string(),
                    short_name: None,
                    type_name: "Wheel".to_string(),
                    typing: Some(typed("Wheel")),
                    multiplicity: Some(mult(2)),
                    ordered: true,
                    subsets: None,
                    redefines: None,
                    value: None,
                    body: PartUsageBody::Semicolon,
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                })))),
                n(PartUsageBodyElement::PartUsage(Box::new(n(PartUsage {
                    usage_prefix: None,
                    is_individual: false,
                    is_reference: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "leftWheel".to_string(),
                    short_name: None,
                    type_name: "".to_string(),
                    typing: None,
                    multiplicity: None,
                    ordered: false,
                    subsets: Some((subs("rearWheel"), Some(expr_index("rearWheel", 1)))),
                    redefines: None,
                    value: None,
                    body: PartUsageBody::Brace {
                        elements: vec![
                            n(PartUsageBodyElement::PortUsage(n(PortUsage {
                                is_abstract: false,
                                direction: None,
                                is_derived: false,
                                is_constant: false,
                                name: "wheelToAxlePort".to_string(),
                                short_name: None,
                                type_name: Some("WheelToAxlePort".to_string()),
                                multiplicity: None,
                                subsets: None,
                                redefines: None,
                                references: None,
                                crosses: None,
                                intersects: None,
                                value: None,
                                body: PortBody::Semicolon,
                                name_span: None,
                                type_ref_span: None,
                                membership: feature_membership(),
                            }))),
                            n(PartUsageBodyElement::PortUsage(n(PortUsage {
                                is_abstract: false,
                                direction: None,
                                is_derived: false,
                                is_constant: false,
                                name: "wheelToRoadPort".to_string(),
                                short_name: None,
                                type_name: Some("WheelToRoadPort".to_string()),
                                multiplicity: None,
                                subsets: None,
                                redefines: None,
                                references: None,
                                crosses: None,
                                intersects: None,
                                value: None,
                                body: PortBody::Semicolon,
                                name_span: None,
                                type_ref_span: None,
                                membership: feature_membership(),
                            }))),
                        ],
                    },
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                })))),
                n(PartUsageBodyElement::PartUsage(Box::new(n(PartUsage {
                    usage_prefix: None,
                    is_individual: false,
                    is_reference: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "rightWheel".to_string(),
                    short_name: None,
                    type_name: "".to_string(),
                    typing: None,
                    multiplicity: None,
                    ordered: false,
                    subsets: Some((subs("rearWheel"), Some(expr_index("rearWheel", 2)))),
                    redefines: None,
                    value: None,
                    body: PartUsageBody::Brace {
                        elements: vec![
                            n(PartUsageBodyElement::PortUsage(n(PortUsage {
                                is_abstract: false,
                                direction: None,
                                is_derived: false,
                                is_constant: false,
                                name: "wheelToAxlePort".to_string(),
                                short_name: None,
                                type_name: Some("WheelToAxlePort".to_string()),
                                multiplicity: None,
                                subsets: None,
                                redefines: None,
                                references: None,
                                crosses: None,
                                intersects: None,
                                value: None,
                                body: PortBody::Semicolon,
                                name_span: None,
                                type_ref_span: None,
                                membership: feature_membership(),
                            }))),
                            n(PartUsageBodyElement::PortUsage(n(PortUsage {
                                is_abstract: false,
                                direction: None,
                                is_derived: false,
                                is_constant: false,
                                name: "wheelToRoadPort".to_string(),
                                short_name: None,
                                type_name: Some("WheelToRoadPort".to_string()),
                                multiplicity: None,
                                subsets: None,
                                redefines: None,
                                references: None,
                                crosses: None,
                                intersects: None,
                                value: None,
                                body: PortBody::Semicolon,
                                name_span: None,
                                type_ref_span: None,
                                membership: feature_membership(),
                            }))),
                        ],
                    },
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                })))),
            ],
        },
        membership: feature_membership(),
    }
}

fn part_differential() -> PartUsage {
    PartUsage {
        usage_prefix: None,
        is_individual: false,
        is_reference: false,
        direction: None,
        is_derived: false,
        is_constant: false,
        name: "differential".to_string(),
        short_name: None,
        type_name: "Differential".to_string(),
        typing: Some(typed("Differential")),
        multiplicity: None,
        ordered: false,
        subsets: None,
        redefines: None,
        value: None,
        name_span: None,
        type_ref_span: None,
        body: PartUsageBody::Brace {
            elements: vec![
                n(PartUsageBodyElement::PortUsage(n(PortUsage {
                    is_abstract: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "shaftPort_d".to_string(),
                    short_name: None,
                    type_name: Some("ShaftPort_d".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    references: None,
                    crosses: None,
                    intersects: None,
                    value: None,
                    body: PortBody::Brace { elements: vec![] },
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                }))),
                n(PartUsageBodyElement::PortUsage(n(PortUsage {
                    is_abstract: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "leftDiffPort".to_string(),
                    short_name: None,
                    type_name: Some("DiffPort".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    references: None,
                    crosses: None,
                    intersects: None,
                    value: None,
                    body: PortBody::Semicolon,
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                }))),
                n(PartUsageBodyElement::PortUsage(n(PortUsage {
                    is_abstract: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "rightDiffPort".to_string(),
                    short_name: None,
                    type_name: Some("DiffPort".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    references: None,
                    crosses: None,
                    intersects: None,
                    value: None,
                    body: PortBody::Semicolon,
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                }))),
            ],
        },
        membership: feature_membership(),
    }
}

fn part_rear_axle() -> PartUsage {
    PartUsage {
        usage_prefix: None,
        is_individual: false,
        is_reference: false,
        direction: None,
        is_derived: false,
        is_constant: false,
        name: "rearAxle".to_string(),
        short_name: None,
        type_name: "RearAxle".to_string(),
        typing: Some(typed("RearAxle")),
        multiplicity: None,
        ordered: false,
        subsets: None,
        redefines: None,
        value: None,
        name_span: None,
        type_ref_span: None,
        body: PartUsageBody::Brace {
            elements: vec![
                n(PartUsageBodyElement::PartUsage(Box::new(n(PartUsage {
                    usage_prefix: None,
                    is_individual: false,
                    is_reference: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "leftHalfAxle".to_string(),
                    short_name: None,
                    type_name: "HalfAxle".to_string(),
                    typing: Some(typed("HalfAxle")),
                    multiplicity: None,
                    ordered: false,
                    subsets: None,
                    redefines: None,
                    value: None,
                    body: PartUsageBody::Semicolon,
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                })))),
                n(PartUsageBodyElement::PartUsage(Box::new(n(PartUsage {
                    usage_prefix: None,
                    is_individual: false,
                    is_reference: false,
                    direction: None,
                    is_derived: false,
                    is_constant: false,
                    name: "rightHalfAxle".to_string(),
                    short_name: None,
                    type_name: "HalfAxle".to_string(),
                    typing: Some(typed("HalfAxle")),
                    multiplicity: None,
                    ordered: false,
                    subsets: None,
                    redefines: None,
                    value: None,
                    body: PartUsageBody::Semicolon,
                    name_span: None,
                    type_ref_span: None,
                    membership: feature_membership(),
                })))),
            ],
        },
        membership: feature_membership(),
    }
}

/// Uses SYSML_V2_RELEASE_DIR when set (CI); otherwise sysml-v2-release in repo.
fn validation_fixture_path(relative: &str) -> std::path::PathBuf {
    let root = std::env::var_os("SYSML_V2_RELEASE_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("sysml-v2-release")
        });
    root.join("sysml")
        .join("src")
        .join("validation")
        .join(relative)
}

#[test]
#[ignore = "requires SysML v2 release fixtures; run with: cargo test --test validation -- --include-ignored"]
fn test_parse_2a_parts_interconnection() {
    super::init_log();
    let path =
        validation_fixture_path("02-Parts Interconnection").join("2a-Parts Interconnection.sysml");
    log::debug!("fixture path: {}", path.display());
    let input = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read fixture {}: {}", path.display(), e));
    log::debug!("input len: {} bytes", input.len());
    let result = parse(&input).expect("parse should succeed for 2a-Parts Interconnection.sysml");
    let expected = expected_ast();
    super::assert_ast_eq(
        &result,
        &expected,
        "parsed AST should match expected for 2a-Parts Interconnection.sysml",
    );
}
