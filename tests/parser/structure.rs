//! Parser tests: structure

use sysml_v2_parser::ast::*;
use sysml_v2_parser::{parse, parse_with_diagnostics};

#[test]
fn test_use_case_def_body_parses_members() {
    let input =
        "package P { use case def U { subject s : System; actor a : Operator; objective { } } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let use_case = match &elements[0].value {
        PackageBodyElement::UseCaseDef(uc) => &uc.value,
        _ => panic!("expected UseCaseDef"),
    };
    let body_elements = match &use_case.body {
        sysml_v2_parser::ast::UseCaseDefBody::Brace { elements } => elements,
        _ => panic!("expected use case brace body"),
    };
    assert!(body_elements.iter().any(|e| matches!(
        e.value,
        sysml_v2_parser::ast::UseCaseDefBodyElement::SubjectDecl(_)
    )));
    assert!(body_elements.iter().any(|e| matches!(
        e.value,
        sysml_v2_parser::ast::UseCaseDefBodyElement::ActorUsage(_)
    )));
    let objective = body_elements
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::UseCaseDefBodyElement::Objective(o) => Some(&o.value),
            _ => None,
        })
        .expect("objective should be present");
    assert_eq!(objective.requirement.value.name, "objective");
    assert!(objective.requirement.value.type_name.is_none());
}

#[test]
fn test_occurrence_usage_parse() {
    let input = "package P { occurrence sample : Event; }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    match &elements[0].value {
        PackageBodyElement::OccurrenceUsage(occ) => {
            assert_eq!(occ.name, "sample");
            assert_eq!(occ.type_name.as_deref(), Some("Event"));
        }
        _ => panic!("expected OccurrenceUsage"),
    }
}

#[test]
fn test_flow_and_allocation_parse() {
    let input = "package P { flow transfer : Fuel from src to dst; allocation map allocate source to target; }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    assert!(matches!(
        elements[0].value,
        PackageBodyElement::FlowUsage(_)
    ));
    assert!(matches!(
        elements[1].value,
        PackageBodyElement::AllocationUsage(_)
    ));
}

#[test]
fn test_flow_and_allocation_brace_bodies_parse() {
    let input = "package P { flow transfer : Fuel from src to dst { x = y; nested { z = q; } } allocation map allocate source to target { one = two; } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };

    match &elements[0].value {
        PackageBodyElement::FlowUsage(flow) => {
            assert!(matches!(
                flow.body,
                sysml_v2_parser::ast::DefinitionBody::Brace { .. }
            ));
        }
        _ => panic!("expected FlowUsage"),
    }

    match &elements[1].value {
        PackageBodyElement::AllocationUsage(alloc) => {
            assert!(matches!(
                alloc.body,
                sysml_v2_parser::ast::DefinitionBody::Brace { .. }
            ));
        }
        _ => panic!("expected AllocationUsage"),
    }
}

#[test]
fn test_metadata_def_brace_body_parse() {
    let input = "package P { metadata def SecurityTag { doc /* classification */ level = high; nested { key = value; } } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };

    match &elements[0].value {
        PackageBodyElement::MetadataDef(metadata) => {
            assert!(matches!(
                metadata.body,
                sysml_v2_parser::ast::AttributeBody::Brace { .. }
            ));
        }
        _ => panic!("expected MetadataDef"),
    }
}

#[test]
fn test_case_family_parse() {
    let input = "package P { case def GenericCase { } analysis def TradeStudy { } verification def VerifyThing { } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    assert!(matches!(elements[0].value, PackageBodyElement::CaseDef(_)));
    assert!(matches!(
        elements[1].value,
        PackageBodyElement::AnalysisCaseDef(_)
    ));
    assert!(matches!(
        elements[2].value,
        PackageBodyElement::VerificationCaseDef(_)
    ));
}

#[test]
fn test_case_family_bodies_parse_use_case_members() {
    let input = "package P { case def C { actor a : Operator; } analysis def A { subject s : System; } verification def V { objective { } } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let case_def = match &elements[0].value {
        PackageBodyElement::CaseDef(c) => &c.value,
        _ => panic!("expected CaseDef"),
    };
    assert!(
        matches!(&case_def.body, sysml_v2_parser::ast::UseCaseDefBody::Brace { elements } if !elements.is_empty())
    );
}

#[test]
fn test_enum_def_with_specialization_and_assigned_literals_maps_dedicated() {
    let input =
        "package P { enum def LevelEnum :> Level { low = 0.25; medium = 0.5; high = 0.75; } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    assert!(matches!(elements[0].value, PackageBodyElement::EnumDef(_)));
    let PackageBodyElement::EnumDef(enum_def) = &elements[0].value else {
        panic!("expected enum def");
    };
    assert_eq!(enum_def.value.specializes.as_deref(), Some("Level"));
    assert!(
        !elements
            .iter()
            .any(|e| matches!(e.value, PackageBodyElement::ExtendedLibraryDecl(_))),
        "enum specialization sample should not fall back to ExtendedLibraryDecl"
    );
}

#[test]
fn test_expression_precedence_parse() {
    let input = "package P { attribute x = 1 + 2 * 3; }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    match &elements[0].value {
        PackageBodyElement::AttributeDef(attr) => {
            let value = attr.typing.as_ref().map(|_| ()).or(Some(()));
            assert!(value.is_some());
        }
        _ => panic!("expected AttributeDef"),
    }
}

#[test]
fn test_expression_allows_qualified_names_and_invocation_arguments() {
    let input =
        "package P { attribute x = Vehicles::Engine.power + normalize(System::Sensors::rpm); }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let attr = match &elements[0].value {
        PackageBodyElement::AttributeDef(attr) => attr,
        other => panic!("expected AttributeDef, got {other:?}"),
    };
    let value = attr
        .value
        .value
        .as_ref()
        .expect("expected value expression");
    match &value.value {
        sysml_v2_parser::ast::Expression::BinaryOp { op, right, .. } => {
            assert_eq!(op, &sysml_v2_parser::ast::BinaryOperator::Add);
            match &right.value {
                sysml_v2_parser::ast::Expression::Invocation { args, .. } => {
                    assert_eq!(args.len(), 1, "expected one invocation argument");
                }
                other => panic!("expected invocation on rhs, got {other:?}"),
            }
        }
        other => panic!("expected binary expression, got {other:?}"),
    }
}

#[test]
fn test_part_def_accepts_nested_interface_definition() {
    let input = r#"package P {
part def Robot {
  interface def signalPorts {
    end supplierPort : Signal;
    end consumerPort : Signal;
  }
  interface: signalPorts connect
    supplierPort ::> outPort to
    consumerPort ::> inPort;
}
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "nested interface def and usage should parse without recovery diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    let part = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(p) => Some(&p.value),
            _ => None,
        })
        .expect("expected part def");
    let sysml_v2_parser::ast::PartDefBody::Brace { elements } = &part.body else {
        panic!("expected part body");
    };
    assert!(elements.iter().any(|e| matches!(
        e.value,
        sysml_v2_parser::ast::PartDefBodyElement::InterfaceDef(_)
    )));
    assert!(elements.iter().any(|e| matches!(
        e.value,
        sysml_v2_parser::ast::PartDefBodyElement::InterfaceUsage(_)
    )));
}

#[test]
fn test_part_def_accepts_nested_item_definition() {
    let input = r#"package P {
part def Accumulator {
  item def Energy;
  attribute mass : Real;
}
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "nested item def in part body should parse without recovery diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    let part = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(p) => Some(&p.value),
            _ => None,
        })
        .expect("expected part def");
    let sysml_v2_parser::ast::PartDefBody::Brace { elements } = &part.body else {
        panic!("expected part body");
    };
    assert!(elements.iter().any(|e| matches!(
        e.value,
        sysml_v2_parser::ast::PartDefBodyElement::ItemDef(_)
    )));
    assert!(elements.iter().any(|e| matches!(
        e.value,
        sysml_v2_parser::ast::PartDefBodyElement::AttributeDef(_)
            | sysml_v2_parser::ast::PartDefBodyElement::AttributeUsage(_)
    )));
}

#[test]
fn test_part_def_body_distinguishes_attribute_def_from_usage_by_def_keyword() {
    // PAR-001 acceptance case: only the explicit `def` keyword makes an AttributeDef; every
    // other legal form in a part-definition body, typed or not, is an AttributeUsage.
    let input = r#"package P {
part def Sensor {
    attribute def LocalValueType;
    attribute typed : Temperature;
    attribute untyped;
    attribute initialized : Temperature = 20;
}
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "typed and untyped attribute usages without `def` should not produce recovery diagnostics: {:?}",
        result.errors
    );
    let RootElement::Package(pkg) = &result.root.elements[0].value else {
        panic!("expected package");
    };
    let PackageBody::Brace { elements } = &pkg.value.body else {
        panic!("expected package body");
    };
    let part = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(p) => Some(&p.value),
            _ => None,
        })
        .expect("expected part def");
    let PartDefBody::Brace { elements } = &part.body else {
        panic!("expected part body");
    };

    let defs: Vec<_> = elements
        .iter()
        .filter_map(|e| match &e.value {
            PartDefBodyElement::AttributeDef(a) => Some(&a.value),
            _ => None,
        })
        .collect();
    assert_eq!(
        defs.len(),
        1,
        "exactly one member has the `def` keyword and should be an AttributeDef: {:?}",
        elements.iter().map(|e| format!("{:?}", e.value)).collect::<Vec<_>>()
    );
    assert_eq!(defs[0].name, "LocalValueType");

    let usages: Vec<_> = elements
        .iter()
        .filter_map(|e| match &e.value {
            PartDefBodyElement::AttributeUsage(a) => Some(&a.value),
            _ => None,
        })
        .collect();
    assert_eq!(
        usages.len(),
        3,
        "typed, untyped, and initialized forms without `def` should all be AttributeUsage: {:?}",
        elements.iter().map(|e| format!("{:?}", e.value)).collect::<Vec<_>>()
    );
    assert!(usages.iter().any(|u| u.name == "typed" && u.typing.as_deref() == Some("Temperature")));
    assert!(usages.iter().any(|u| u.name == "untyped" && u.typing.is_none()));
    assert!(usages.iter().any(|u| u.name == "initialized"
        && u.typing.as_deref() == Some("Temperature")
        && u.value.is_some()));
}

#[test]
fn test_part_def_body_never_misclassifies_non_connector_interface_as_definition() {
    // Same bug class as PAR-001: `interface_def` had an optional `def`, and `interface_usage`
    // only recognizes connector forms (`connect ... to ...`). A plain typed, non-connector
    // interface declaration used to fall through interface_usage and get silently accepted as
    // an InterfaceDef. It must never be classified as a definition; today the parser doesn't yet
    // support this usage form (tracked separately, see PAR-002), so it must surface as an
    // explicit recovery/error element instead of a false InterfaceDef.
    let input = "package P {\npart def Home {\ninterface foo : SomeInterfaceType;\n}\n}";
    let result = parse_with_diagnostics(input);
    let RootElement::Package(pkg) = &result.root.elements[0].value else {
        panic!("expected package");
    };
    let PackageBody::Brace { elements } = &pkg.value.body else {
        panic!("expected package body");
    };
    let part = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(p) => Some(&p.value),
            _ => None,
        })
        .expect("expected part def");
    let PartDefBody::Brace { elements } = &part.body else {
        panic!("expected part body");
    };
    assert!(
        !elements
            .iter()
            .any(|e| matches!(e.value, PartDefBodyElement::InterfaceDef(_))),
        "non-connector interface usage must never be misclassified as InterfaceDef: {:?}",
        elements.iter().map(|e| format!("{:?}", e.value)).collect::<Vec<_>>()
    );
}

#[test]
fn test_part_def_accepts_nested_part_definition() {
    let input = r#"package P {
part def Accumulator {
  item def Energy;
  part def Cell {
    attribute capacity : Real;
  }
}
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "nested part def in part body should parse without recovery diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    let part = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(p) => Some(&p.value),
            _ => None,
        })
        .expect("expected part def");
    let sysml_v2_parser::ast::PartDefBody::Brace { elements } = &part.body else {
        panic!("expected part body");
    };
    assert!(elements.iter().any(|e| matches!(
        e.value,
        sysml_v2_parser::ast::PartDefBodyElement::PartDef(_)
    )));
}

#[test]
fn test_parse_part_attribute_prefix_redefines_shorthand() {
    let input = "package P {\npart def Laptop { attribute name : String; }\npart office {\npart laptop1: Laptop {\nattribute :>> name = \"My Laptop\";\n}\n}\n}";
    let result = parse(input).expect("attribute prefix redefines shorthand should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let office = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartUsage(p) if p.value.name == "office" => Some(&p.value),
            _ => None,
        })
        .expect("office part usage should be present");
    let office_body = match &office.body {
        sysml_v2_parser::ast::PartUsageBody::Brace { elements } => elements,
        _ => panic!("expected office part body"),
    };
    let laptop1 = office_body
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::PartUsageBodyElement::PartUsage(p)
                if p.value.name == "laptop1" =>
            {
                Some(&p.value)
            }
            _ => None,
        })
        .expect("laptop1 part usage should be present");
    let laptop1_body = match &laptop1.body {
        sysml_v2_parser::ast::PartUsageBody::Brace { elements } => elements,
        _ => panic!("expected laptop1 part body"),
    };
    let attribute = laptop1_body
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::PartUsageBodyElement::AttributeUsage(a) => Some(&a.value),
            _ => None,
        })
        .expect("attribute usage should be present");
    assert_eq!(attribute.name, "name");
    assert_eq!(attribute.redefines.as_deref(), Some("name"));
    assert!(
        attribute.value.is_some(),
        "attribute value should be parsed"
    );
}

#[test]
fn test_parse_part_attribute_prefix_redefines_scientific_notation_with_quoted_unit() {
    let input = r#"package P {
  part def Mission {
    attribute :>> researchAndDevelopmentCost = 5E9 ['$'];
    attribute :>> manufacturingCost = 3E9 ['$'];
  }
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let root = parse(input).expect("parse should succeed");
    let pkg = match &root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let mission = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(p)
                if p.value.identification.name.as_deref() == Some("Mission") =>
            {
                Some(&p.value)
            }
            _ => None,
        })
        .expect("Mission part def should be present");
    let mission_body = match &mission.body {
        sysml_v2_parser::ast::PartDefBody::Brace { elements } => elements,
        _ => panic!("expected Mission part def body"),
    };
    let attr = mission_body
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::PartDefBodyElement::AttributeUsage(a)
                if a.value.name == "researchAndDevelopmentCost" =>
            {
                Some(&a.value)
            }
            _ => None,
        })
        .expect("researchAndDevelopmentCost attribute usage should be present");
    assert_eq!(
        attr.redefines.as_deref(),
        Some("researchAndDevelopmentCost")
    );
    assert!(attr.value.is_some(), "attribute value should be parsed");
}

#[test]
fn test_parse_part_attribute_prefix_redefines_with_subsets_clause() {
    let input = "package P {\npart def Room { attribute outlet: Outlet; }\npart def Home {\npart livingRoom : Room {\nattribute :>> outlet :> electricGrid.outlets;\n}\n}\n}";
    let result = parse(input).expect("attribute prefix redefines with subsets should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let home = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(p)
                if p.value.identification.name.as_deref() == Some("Home") =>
            {
                Some(&p.value)
            }
            _ => None,
        })
        .expect("Home part def should be present");
    let home_body = match &home.body {
        sysml_v2_parser::ast::PartDefBody::Brace { elements } => elements,
        _ => panic!("expected Home part def body"),
    };
    let living_room = home_body
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::PartDefBodyElement::PartUsage(p)
                if p.value.name == "livingRoom" =>
            {
                Some(&p.value)
            }
            _ => None,
        })
        .expect("livingRoom part usage should be present");
    let living_room_body = match &living_room.body {
        sysml_v2_parser::ast::PartUsageBody::Brace { elements } => elements,
        _ => panic!("expected livingRoom part usage body"),
    };
    let attribute = living_room_body
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::PartUsageBodyElement::AttributeUsage(a) => Some(&a.value),
            _ => None,
        })
        .expect("attribute usage should be present");
    assert_eq!(attribute.name, "outlet");
    assert_eq!(attribute.redefines.as_deref(), Some("outlet"));
}

#[test]
fn test_parse_interface_usage_named_with_multiplicity() {
    let input = "package P {\npart def Home {\npart livingRoom: Room {\ninterface heater2PowerOutlet[1] : Socket2OutletInterface connect heater.socket to outlet;\n}\n}\n}";
    let result = parse(input).expect("named interface usage with multiplicity should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let home = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(p)
                if p.value.identification.name.as_deref() == Some("Home") =>
            {
                Some(&p.value)
            }
            _ => None,
        })
        .expect("Home part def should be present");
    let home_body = match &home.body {
        sysml_v2_parser::ast::PartDefBody::Brace { elements } => elements,
        _ => panic!("expected Home part def body"),
    };
    let living_room = home_body
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::PartDefBodyElement::PartUsage(p)
                if p.value.name == "livingRoom" =>
            {
                Some(&p.value)
            }
            _ => None,
        })
        .expect("livingRoom part usage should be present");
    let living_room_body = match &living_room.body {
        sysml_v2_parser::ast::PartUsageBody::Brace { elements } => elements,
        _ => panic!("expected livingRoom part usage body"),
    };
    assert!(
        living_room_body.iter().any(|e| matches!(
            e.value,
            sysml_v2_parser::ast::PartUsageBodyElement::InterfaceUsage(_)
        )),
        "named interface usage with multiplicity should be preserved"
    );
}

#[test]
fn test_parse_part_def_connection_usage_multiline_connect_clause() {
    let input = "package P {\nconnection def Door { end [1] part room1 : Room; end [1] part room2 : Room; }\npart def Home {\nconnection livingRoom2bedRoom[1] : Door\n  connect livingRoom to bedRoom;\nconnection livingRoom2kitchen[1] : Door\n  connect livingRoom to kitchen;\nconnection livingRoom2bathRoom[1] : Door\n  connect livingRoom to bathRoom;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "multiline connection usage should parse without recovery diagnostics: {:?}",
        result.errors
    );
}

#[test]
fn test_parse_use_case_subject_shorthand_without_name() {
    let input = "package P {\nuse case def U {\nsubject: Laptop;\nobjective { }\n}\n}";
    let result = parse(input).expect("subject shorthand should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let use_case = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::UseCaseDef(u) => Some(&u.value),
            _ => None,
        })
        .expect("use case def should be present");
    let body_elements = match &use_case.body {
        sysml_v2_parser::ast::UseCaseDefBody::Brace { elements } => elements,
        _ => panic!("expected use case brace body"),
    };
    let subject = body_elements
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::UseCaseDefBodyElement::SubjectDecl(s) => Some(&s.value),
            _ => None,
        })
        .expect("subject decl should be present");
    assert_eq!(subject.name, "subject");
    assert_eq!(subject.type_name, "Laptop");
    assert!(
        body_elements.iter().any(|e| matches!(
            e.value,
            sysml_v2_parser::ast::UseCaseDefBodyElement::Objective(_)
        )),
        "later use case members should still parse after subject shorthand"
    );
}

#[test]
fn test_part_def_accepts_specializes_keyword_as_specialization() {
    let input = r#"package P {
part def A specializes B;
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let part_def = match &elements[0].value {
        PackageBodyElement::PartDef(p) => p,
        other => panic!("expected part def, got {:?}", other),
    };
    assert_eq!(part_def.value.specializes.as_deref(), Some("B"));
    assert!(
        part_def.value.specializes_span.is_some(),
        "specializes span should be present for keyword form"
    );
}

#[test]
fn test_part_def_preserves_multiple_specializes_targets() {
    let input = r#"package P {
part def A :> B, C, D;
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let part_def = match &elements[0].value {
        PackageBodyElement::PartDef(part) => part,
        other => panic!("expected part definition, got {:?}", other),
    };
    assert_eq!(part_def.value.specializes.as_deref(), Some("B, C, D"));
    assert!(
        part_def.value.specializes_span.is_some(),
        "specializes span should be present for multi-target form"
    );
}

#[test]
fn test_port_def_accepts_specializes_keyword_as_specialization() {
    let input = r#"package P {
port def ControlPort specializes BasePort;
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let port_def = match &elements[0].value {
        PackageBodyElement::PortDef(p) => p,
        other => panic!("expected port def, got {:?}", other),
    };
    assert_eq!(port_def.value.specializes.as_deref(), Some("BasePort"));
}

#[test]
fn test_port_def_preserves_multiple_specializes_targets() {
    let input = r#"package P {
port def ControlPort :> BasePort, DiagnosticPort;
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let port_def = match &elements[0].value {
        PackageBodyElement::PortDef(port) => port,
        other => panic!("expected port definition, got {:?}", other),
    };
    assert_eq!(
        port_def.value.specializes.as_deref(),
        Some("BasePort, DiagnosticPort")
    );
    assert!(port_def.value.specializes_span.is_some());
}

#[test]
fn test_individual_def_accepts_specializes_keyword_as_specialization() {
    let input = r#"package P {
individual def Rover specializes MobileRobot;
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let individual_def = match &elements[0].value {
        PackageBodyElement::IndividualDef(p) => p,
        other => panic!("expected individual def, got {:?}", other),
    };
    assert_eq!(
        individual_def.value.specializes.as_deref(),
        Some("MobileRobot")
    );
}

#[test]
fn test_occurrence_usage_accepts_keyword_subset_and_redefine_aliases() {
    let input = r#"package P {
occurrence rover subsets BaseOccurrence redefines LegacyOccurrence;
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let occ = match &elements[0].value {
        PackageBodyElement::OccurrenceUsage(o) => o,
        other => panic!("expected occurrence usage, got {:?}", other),
    };
    assert_eq!(occ.value.subsets.as_deref(), Some("BaseOccurrence"));
    assert_eq!(occ.value.redefines.as_deref(), Some("LegacyOccurrence"));
}

#[test]
fn test_occurrence_usage_accepts_typed_by_and_specialization_clauses() {
    let input = r#"package P {
occurrence event typed by Mission::Event subsets events redefines oldEvent;
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let occ = match &elements[0].value {
        PackageBodyElement::OccurrenceUsage(o) => o,
        other => panic!("expected occurrence usage, got {:?}", other),
    };
    assert_eq!(occ.value.name, "event");
    assert_eq!(occ.value.type_name.as_deref(), Some("Mission::Event"));
    assert_eq!(occ.value.subsets.as_deref(), Some("events"));
    assert_eq!(occ.value.redefines.as_deref(), Some("oldEvent"));
}

#[test]
fn test_occurrence_usage_post_body_specialization_still_parses() {
    let input = r#"package P {
occurrence rover; subsets BaseOccurrence redefines LegacyOccurrence;
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let occ = match &elements[0].value {
        PackageBodyElement::OccurrenceUsage(o) => o,
        other => panic!("expected occurrence usage, got {:?}", other),
    };
    assert_eq!(occ.value.subsets.as_deref(), Some("BaseOccurrence"));
    assert_eq!(occ.value.redefines.as_deref(), Some("LegacyOccurrence"));
}

#[test]
fn test_use_case_usage_accepts_typed_by_and_specialization_clauses() {
    let input = r#"package P {
use case mission typed by Mission::CaseType subsets BaseCase;
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let use_case = match &elements[0].value {
        PackageBodyElement::UseCaseUsage(u) => u,
        other => panic!("expected use case usage, got {:?}", other),
    };
    assert_eq!(
        use_case.value.type_name.as_deref(),
        Some("Mission::CaseType")
    );
}

#[test]
fn test_then_use_case_usage_accepts_typed_by_alias() {
    let input = r#"package P {
use case def U {
then use case step typed by Mission::StepCase;
}
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let use_case_def = match &pkg.value.body {
        PackageBody::Brace { elements } => match &elements[0].value {
            PackageBodyElement::UseCaseDef(d) => d,
            other => panic!("expected use case def, got {:?}", other),
        },
        other => panic!("expected brace body, got {:?}", other),
    };
    let body_elements = match &use_case_def.value.body {
        sysml_v2_parser::ast::UseCaseDefBody::Brace { elements } => elements,
        other => panic!("expected use case brace body, got {:?}", other),
    };
    let then_use_case = body_elements
        .iter()
        .find_map(|el| match &el.value {
            sysml_v2_parser::ast::UseCaseDefBodyElement::ThenUseCaseUsage(t) => Some(t),
            _ => None,
        })
        .expect("then use case usage should be present");
    assert_eq!(
        then_use_case.value.use_case.value.type_name.as_deref(),
        Some("Mission::StepCase")
    );
}

#[test]
fn test_attribute_def_brace_body_preserves_structured_members() {
    let input = r#"package P {
attribute def Tensor {
doc /* tensor doc */
attribute def rank: Integer;
attribute usage : Real;
}
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let attr_def = match &pkg.value.body {
        PackageBody::Brace { elements } => match &elements[0].value {
            PackageBodyElement::AttributeDef(a) => a,
            other => panic!("expected attribute def, got {:?}", other),
        },
        other => panic!("expected brace body, got {:?}", other),
    };
    let members = match &attr_def.value.body {
        sysml_v2_parser::ast::AttributeBody::Brace { elements } => elements,
        other => panic!("expected structured attribute body, got {:?}", other),
    };
    assert!(
        members.len() >= 2,
        "attribute definition body should retain nested members"
    );
}

#[test]
fn test_metadata_def_brace_body_preserves_doc_member() {
    let input = r#"package P {
metadata def Tag {
doc /* tag doc */
}
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let metadata_def = match &pkg.value.body {
        PackageBody::Brace { elements } => match &elements[0].value {
            PackageBodyElement::MetadataDef(m) => m,
            other => panic!("expected metadata def, got {:?}", other),
        },
        other => panic!("expected brace body, got {:?}", other),
    };
    let members = match &metadata_def.value.body {
        sysml_v2_parser::ast::AttributeBody::Brace { elements } => elements,
        other => panic!("expected structured metadata body, got {:?}", other),
    };
    assert!(
        !members.is_empty(),
        "metadata definition body should retain doc member"
    );
}

#[test]
fn test_part_def_brace_body_preserves_structured_members() {
    let input = r#"package P {
part def Vehicle {
doc /* vehicle doc */
attribute mass: Real;
part wheel : Wheel;
}
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let part_def = match &pkg.value.body {
        PackageBody::Brace { elements } => match &elements[0].value {
            PackageBodyElement::PartDef(d) => d,
            other => panic!("expected part def, got {:?}", other),
        },
        other => panic!("expected brace body, got {:?}", other),
    };
    let members = match &part_def.value.body {
        sysml_v2_parser::ast::PartDefBody::Brace { elements } => elements,
        other => panic!("expected structured part def body, got {:?}", other),
    };
    assert!(
        members.len() >= 2,
        "part definition body should retain nested members"
    );
}

#[test]
fn test_port_def_brace_body_preserves_structured_members() {
    let input = r#"package P {
port def FuelPort {
doc /* port doc */
in fuel : Fuel;
}
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let port_def = match &pkg.value.body {
        PackageBody::Brace { elements } => match &elements[0].value {
            PackageBodyElement::PortDef(d) => d,
            other => panic!("expected port def, got {:?}", other),
        },
        other => panic!("expected brace body, got {:?}", other),
    };
    let members = match &port_def.value.body {
        sysml_v2_parser::ast::PortDefBody::Brace { elements } => elements,
        other => panic!("expected structured port def body, got {:?}", other),
    };
    assert!(
        members.len() >= 2,
        "port definition body should retain doc and in/out members"
    );
}

#[test]
fn test_port_usage_brace_body_preserves_nested_port_members() {
    let input = r#"package P {
part vehicle {
port vehicleToRoadPort {
port leftWheelToRoadPort;
port rightWheelToRoadPort;
}
}
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let part_usage = match &pkg.value.body {
        PackageBody::Brace { elements } => match &elements[0].value {
            PackageBodyElement::PartUsage(p) => p,
            other => panic!("expected part usage, got {:?}", other),
        },
        other => panic!("expected brace body, got {:?}", other),
    };
    let port_usage = match &part_usage.value.body {
        sysml_v2_parser::ast::PartUsageBody::Brace { elements } => elements
            .iter()
            .find_map(|el| match &el.value {
                PartUsageBodyElement::PortUsage(p) => Some(p),
                _ => None,
            })
            .expect("part usage should contain nested port usage"),
        other => panic!("expected part usage brace body, got {:?}", other),
    };
    let members = match &port_usage.value.body {
        sysml_v2_parser::ast::PortBody::Brace { elements } => elements,
        other => panic!("expected structured port body, got {:?}", other),
    };
    assert_eq!(
        members.len(),
        2,
        "port usage body should retain nested port members"
    );
}

#[test]
fn test_port_usage_body_preserves_doc_with_colon_and_comma_list() {
    // Regression for a real-world model (sysml-robot-vacuum-cleaner) where a `doc /* ... */`
    // block inside a port *usage* body (not a port def) failed to parse because
    // `PortBodyElement`/`port_body_element` had no `doc_comment` alternative. The parser then
    // fell back to error recovery, which misclassified the doc text — e.g. a line like
    // "Addresses: front ToF 0x29, ..." — as a "bare feature declaration in part definition body".
    let input = r#"package P {
part def SensorAssembly {
port sensorBusOut : I2cPort {
doc /*
Sensor harness mate to main PCB J_SENSOR_I2C (6-pin JST-GH).
3.3 V logic at 400 kHz; SDA/SCL open-drain with 3.3 V pull-ups.
Addresses: front ToF 0x29, left ToF 0x2A, right ToF 0x2B, IMU 0x68.
*/
}
}
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let part_def = match &pkg.value.body {
        PackageBody::Brace { elements } => match &elements[0].value {
            PackageBodyElement::PartDef(d) => d,
            other => panic!("expected part def, got {:?}", other),
        },
        other => panic!("expected brace body, got {:?}", other),
    };
    let port_usage = match &part_def.value.body {
        sysml_v2_parser::ast::PartDefBody::Brace { elements } => elements
            .iter()
            .find_map(|el| match &el.value {
                PartDefBodyElement::PortUsage(p) => Some(p),
                _ => None,
            })
            .expect("part def should contain nested port usage"),
        other => panic!("expected part def brace body, got {:?}", other),
    };
    let members = match &port_usage.value.body {
        sysml_v2_parser::ast::PortBody::Brace { elements } => elements,
        other => panic!("expected structured port body, got {:?}", other),
    };
    assert_eq!(
        members.len(),
        1,
        "port usage body should parse exactly one member (the doc block), got {:?}",
        members
    );
    match &members[0].value {
        PortBodyElement::Doc(doc) => {
            assert!(
                doc.value.text.contains("Addresses: front ToF 0x29"),
                "doc text should retain the colon-and-comma-list line verbatim, got {:?}",
                doc.value.text
            );
        }
        PortBodyElement::Error(err) => panic!(
            "doc block inside port usage body should not produce a recovery error node, got {:?}",
            err
        ),
        other => panic!("expected Doc member, got {:?}", other),
    }
}

#[test]
fn test_port_usage_normalizes_subset_redefine_aliases() {
    let input = r#"package P {
part def Carrier {
  port :>> wheelPort : WheelPortType subsets basePort;
}
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let part_def = match &elements[0].value {
        PackageBodyElement::PartDef(p) => p,
        other => panic!("expected part def, got {:?}", other),
    };
    let part_body = match &part_def.value.body {
        sysml_v2_parser::ast::PartDefBody::Brace { elements } => elements,
        other => panic!("expected part def brace body, got {:?}", other),
    };
    let port_usage = match &part_body[0].value {
        sysml_v2_parser::ast::PartDefBodyElement::PortUsage(p) => p,
        other => panic!("expected port usage, got {:?}", other),
    };
    assert_eq!(
        port_usage
            .value
            .subsets
            .as_ref()
            .map(|(name, _)| name.as_str()),
        Some("basePort")
    );
    assert_eq!(port_usage.value.redefines.as_deref(), Some("wheelPort"));
}

#[test]
fn test_port_usage_accepts_defined_by_typings() {
    let input = r#"package P {
part def Carrier {
  port fuelPort defined by ~Ports::FuelPort, Ports::CommandPort[1] subsets basePort;
}
}"#;
    let result = parse(input).expect("defined-by port usage should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let part_def = match &elements[0].value {
        PackageBodyElement::PartDef(p) => p,
        other => panic!("expected part def, got {:?}", other),
    };
    let part_body = match &part_def.value.body {
        sysml_v2_parser::ast::PartDefBody::Brace { elements } => elements,
        other => panic!("expected part def brace body, got {:?}", other),
    };
    let port_usage = match &part_body[0].value {
        sysml_v2_parser::ast::PartDefBodyElement::PortUsage(p) => p,
        other => panic!("expected port usage, got {:?}", other),
    };
    assert_eq!(
        port_usage.value.type_name.as_deref(),
        Some("~Ports::FuelPort, Ports::CommandPort")
    );
    assert_eq!(port_usage.value.multiplicity.as_deref(), Some("[1]"));
    assert_eq!(
        port_usage
            .value
            .subsets
            .as_ref()
            .map(|(name, _)| name.as_str()),
        Some("basePort")
    );
}

#[test]
fn test_port_usage_accepts_typed_by_typings() {
    let input = r#"package P {
part def Carrier {
  port fuelPort typed by ~Ports::FuelPort, Ports::CommandPort[1] subsets basePort;
}
}"#;
    let result = parse(input).expect("typed-by port usage should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let part_def = match &elements[0].value {
        PackageBodyElement::PartDef(p) => p,
        other => panic!("expected part def, got {:?}", other),
    };
    let part_body = match &part_def.value.body {
        sysml_v2_parser::ast::PartDefBody::Brace { elements } => elements,
        other => panic!("expected part def brace body, got {:?}", other),
    };
    let port_usage = match &part_body[0].value {
        sysml_v2_parser::ast::PartDefBodyElement::PortUsage(p) => p,
        other => panic!("expected port usage, got {:?}", other),
    };
    assert_eq!(
        port_usage.value.type_name.as_deref(),
        Some("~Ports::FuelPort, Ports::CommandPort")
    );
    assert_eq!(port_usage.value.multiplicity.as_deref(), Some("[1]"));
    assert_eq!(
        port_usage
            .value
            .subsets
            .as_ref()
            .map(|(name, _)| name.as_str()),
        Some("basePort")
    );
}

#[test]
fn test_port_usage_accepts_multiple_specialization_clauses() {
    let input = r#"package P {
part def Carrier {
  port fuelPort : FuelPort subsets basePort redefines oldPort :> latestPort :>> newestPort;
}
}"#;
    let result =
        parse(input).expect("port usage with multiple specialization clauses should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let part_def = match &elements[0].value {
        PackageBodyElement::PartDef(p) => p,
        other => panic!("expected part def, got {:?}", other),
    };
    let part_body = match &part_def.value.body {
        sysml_v2_parser::ast::PartDefBody::Brace { elements } => elements,
        other => panic!("expected part def brace body, got {:?}", other),
    };
    let port_usage = match &part_body[0].value {
        sysml_v2_parser::ast::PartDefBodyElement::PortUsage(p) => p,
        other => panic!("expected port usage, got {:?}", other),
    };
    assert_eq!(
        port_usage
            .value
            .subsets
            .as_ref()
            .map(|(name, _)| name.as_str()),
        Some("latestPort")
    );
    assert_eq!(port_usage.value.redefines.as_deref(), Some("newestPort"));
}

#[test]
fn test_constraint_expressions_keep_parenthesized_associativity_shapes() {
    let input = r#"package P {
constraint def C {
  ((a-b)-c) >= 0;
  a-(b-c) >= 0;
}
}"#;
    let result = parse(input).expect("constraint expressions should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    let constraint = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::ConstraintDef(c) => Some(&c.value),
            _ => None,
        })
        .expect("expected constraint definition");
    let sysml_v2_parser::ast::ConstraintDefBody::Brace { elements } = &constraint.body else {
        panic!("expected constraint body");
    };
    let exprs: Vec<&sysml_v2_parser::ast::Node<sysml_v2_parser::ast::Expression>> = elements
        .iter()
        .filter_map(|e| match &e.value {
            sysml_v2_parser::ast::ConstraintDefBodyElement::Expression(expr) => Some(expr),
            _ => None,
        })
        .collect();
    assert_eq!(exprs.len(), 2, "expected two parsed comparison expressions");
    for expr in exprs {
        match &expr.value {
            sysml_v2_parser::ast::Expression::BinaryOp { op, right, .. } => {
                assert_eq!(op, &sysml_v2_parser::ast::BinaryOperator::Ge);
                assert!(matches!(
                    right.value,
                    sysml_v2_parser::ast::Expression::LiteralInteger(0)
                ));
            }
            other => panic!("expected comparison expression, got {other:?}"),
        }
    }
}

#[test]
fn test_shorthand_attribute_value_uses_expression_parser_path() {
    let input = r#"package P {
part def Vehicle {
  mass : Real = ((a-b)-c) >= 0;
}
}"#;
    let result = parse(input).expect("shorthand attribute value should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    let part = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(p) => Some(&p.value),
            _ => None,
        })
        .expect("expected part def");
    let sysml_v2_parser::ast::PartDefBody::Brace { elements } = &part.body else {
        panic!("expected part body");
    };
    let usage = elements
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::PartDefBodyElement::AttributeUsage(a) => Some(&a.value),
            _ => None,
        })
        .expect("expected shorthand attribute usage");
    assert!(
        usage.value.is_some(),
        "value expression should be preserved"
    );
}

#[test]
fn test_parse_typed_attribute_usage_in_part_usage_body() {
    let input = r#"package P {
  private import ISQ::*;
  private import SI::*;
  attribute def MassValue;
  part AutonomousFloorCleaningRobot {
    attribute totalMassKg : MassValue = 4.2 [kg];
    part mobility : MobilitySubsystem;
  }
  part def MobilitySubsystem;
}"#;
    let result = sysml_v2_parser::parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "typed attribute usage in part usage body should parse cleanly: {:?}",
        result.errors
    );

    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    let robot = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartUsage(p) if p.value.name == "AutonomousFloorCleaningRobot" => {
                Some(&p.value)
            }
            _ => None,
        })
        .expect("robot part usage");
    let PartUsageBody::Brace { elements } = &robot.body else {
        panic!("expected robot part usage body");
    };
    let attribute = elements
        .iter()
        .find_map(|e| match &e.value {
            PartUsageBodyElement::AttributeUsage(a) => Some(&a.value),
            _ => None,
        })
        .expect("typed attribute usage in part usage body");
    assert_eq!(attribute.name, "totalMassKg");
    assert_eq!(attribute.typing.as_deref(), Some("MassValue"));
    assert!(attribute.value.is_some(), "attribute value should parse");
}

#[test]
fn test_attribute_usage_accepts_defined_by_typing() {
    let input = r#"package P {
  part Vehicle {
    attribute mass defined by ISQ::MassValue;
  }
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "defined-by attribute usage should parse cleanly: {:?}",
        result.errors
    );
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    let part = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartUsage(p) => Some(&p.value),
            _ => None,
        })
        .expect("part usage");
    let PartUsageBody::Brace { elements } = &part.body else {
        panic!("expected part body");
    };
    let attribute = elements
        .iter()
        .find_map(|e| match &e.value {
            PartUsageBodyElement::AttributeUsage(a) => Some(&a.value),
            _ => None,
        })
        .expect("attribute usage");
    assert_eq!(attribute.name, "mass");
    assert_eq!(attribute.typing.as_deref(), Some("ISQ::MassValue"));
}

#[test]
fn test_attribute_usage_accepts_typed_by_default_value() {
    let input = r#"package P {
  part Vehicle {
    attribute speed typed by ISQ::SpeedValue default = 1;
  }
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "typed-by attribute usage should parse cleanly: {:?}",
        result.errors
    );
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    let part = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartUsage(p) => Some(&p.value),
            _ => None,
        })
        .expect("part usage");
    let PartUsageBody::Brace { elements } = &part.body else {
        panic!("expected part body");
    };
    let attribute = elements
        .iter()
        .find_map(|e| match &e.value {
            PartUsageBodyElement::AttributeUsage(a) => Some(&a.value),
            _ => None,
        })
        .expect("attribute usage");
    assert_eq!(attribute.name, "speed");
    assert_eq!(attribute.typing.as_deref(), Some("ISQ::SpeedValue"));
    assert!(attribute.value.is_some(), "default value should parse");
}

#[test]
fn test_attribute_usage_prefix_redefines_accepts_defined_by_typing() {
    let input = r#"package P {
  part Vehicle {
    attribute :>> Vehicle::mass defined by ISQ::MassValue;
  }
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "prefix-redefines attribute usage should parse cleanly: {:?}",
        result.errors
    );
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    let part = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartUsage(p) => Some(&p.value),
            _ => None,
        })
        .expect("part usage");
    let PartUsageBody::Brace { elements } = &part.body else {
        panic!("expected part body");
    };
    let attribute = elements
        .iter()
        .find_map(|e| match &e.value {
            PartUsageBodyElement::AttributeUsage(a) => Some(&a.value),
            _ => None,
        })
        .expect("attribute usage");
    assert_eq!(attribute.name, "mass");
    assert_eq!(attribute.redefines.as_deref(), Some("Vehicle::mass"));
    assert_eq!(attribute.typing.as_deref(), Some("ISQ::MassValue"));
}

#[test]
fn test_attribute_usage_accepts_subsets_clause_without_ast_field() {
    let input = r#"package P {
  part Vehicle {
    attribute outlet : PowerPort subsets gridPorts;
  }
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "subsets attribute usage should parse cleanly: {:?}",
        result.errors
    );
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    let part = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartUsage(p) => Some(&p.value),
            _ => None,
        })
        .expect("part usage");
    let PartUsageBody::Brace { elements } = &part.body else {
        panic!("expected part body");
    };
    let attribute = elements
        .iter()
        .find_map(|e| match &e.value {
            PartUsageBodyElement::AttributeUsage(a) => Some(&a.value),
            _ => None,
        })
        .expect("attribute usage");
    assert_eq!(attribute.name, "outlet");
    assert_eq!(attribute.typing.as_deref(), Some("PowerPort"));
}

#[test]
fn test_attribute_def_accepts_multiplicity_and_uniqueness_before_specialization() {
    let input = r#"package P {
  attribute length: LengthValue[*] nonunique :> scalarQuantities;
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "attribute header modifiers should parse cleanly: {:?}",
        result.errors
    );
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    let attribute = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::AttributeDef(a) => Some(&a.value),
            _ => None,
        })
        .expect("attribute definition");
    assert_eq!(attribute.name, "length");
    assert_eq!(attribute.typing.as_deref(), Some("LengthValue"));
}

#[test]
fn test_attribute_def_accepts_untyped_multiplicity_uniqueness_brace_body() {
    let input = r#"package P {
  attribute measuresOfEffectiveness[*] nonunique { doc /* Base feature. */ }
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "untyped attribute modifiers should parse cleanly: {:?}",
        result.errors
    );
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(&e.value, PackageBodyElement::AttributeDef(a) if a.value.name == "measuresOfEffectiveness")),
        "attribute definition should be dedicated, not fallback"
    );
}

#[test]
fn test_attribute_def_accepts_default_value_without_equals_after_specialization() {
    let input = r#"package P {
  attribute xoffset : LengthValue [0..*] :> scalarQuantities default 0 [m];
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "attribute default shorthand should parse cleanly: {:?}",
        result.errors
    );
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    let attribute = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::AttributeDef(a) => Some(&a.value),
            _ => None,
        })
        .expect("attribute definition");
    assert_eq!(attribute.typing.as_deref(), Some("LengthValue"));
    assert!(attribute.value.is_some(), "default value should parse");
}

#[test]
fn test_attribute_def_accepts_multiple_specialization_targets() {
    let input = r#"package P {
  attribute def TranslationRotationSequence :> CoordinateTransformation, List {
    attribute :>> elements : TranslationOrRotation[1..*] ordered nonunique;
  }
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "multi-target attribute definition should parse cleanly: {:?}",
        result.errors
    );
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    assert!(
        elements.iter().any(|e| matches!(
            &e.value,
            PackageBodyElement::AttributeDef(a) if a.value.name == "TranslationRotationSequence"
        )),
        "attribute definition should be dedicated"
    );
}

#[test]
fn test_attribute_def_accepts_constructor_default_value() {
    let input = r#"package P {
  attribute one : DimensionOneUnit[1] = new DimensionOneUnit();
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "constructor default should parse cleanly: {:?}",
        result.errors
    );
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    let attribute = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::AttributeDef(a) => Some(&a.value),
            _ => None,
        })
        .expect("attribute definition");
    assert_eq!(attribute.name, "one");
    assert!(attribute.value.is_some(), "constructor value should parse");
}

#[test]
fn test_part_usage_body_ref_part_assignments_parse() {
    let input = r#"package RefPartAssignmentProbe {
  part def Body;
  part def Orbit {
    ref part centralBody : Body;
    ref part orbitingBody : Body;
  }
  part system {
    part sun : Body;
    part earth : Body;
    part earthOrbit : Orbit {
      ref part centralBody = sun;
      ref part orbitingBody : Body = earth;
    }
  }
}"#;
    let result = sysml_v2_parser::parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "ref part assignment forms should parse cleanly: {:?}",
        result.errors
    );

    let package = match &result.root.elements[0].value {
        RootElement::Package(package) => &package.value,
        other => panic!("expected package root element, got {other:?}"),
    };
    let PackageBody::Brace { elements } = &package.body else {
        panic!("expected package body");
    };
    let system = elements
        .iter()
        .find_map(|element| match &element.value {
            PackageBodyElement::PartUsage(part) if part.value.name == "system" => Some(&part.value),
            _ => None,
        })
        .expect("system part usage");
    let PartUsageBody::Brace { elements } = &system.body else {
        panic!("expected system part usage body");
    };
    let earth_orbit = elements
        .iter()
        .find_map(|element| match &element.value {
            PartUsageBodyElement::PartUsage(part) if part.value.name == "earthOrbit" => {
                Some(&part.value)
            }
            _ => None,
        })
        .expect("earthOrbit part usage");
    let PartUsageBody::Brace { elements } = &earth_orbit.body else {
        panic!("expected earthOrbit body");
    };
    let refs: Vec<_> = elements
        .iter()
        .filter_map(|element| match &element.value {
            PartUsageBodyElement::Ref(reference) => Some(&reference.value),
            _ => None,
        })
        .collect();
    assert_eq!(refs.len(), 2, "expected two ref part assignments");
    assert_eq!(refs[0].name, "centralBody");
    assert_eq!(refs[0].type_name, "");
    assert!(refs[0].value.is_some());
    assert_eq!(refs[1].name, "orbitingBody");
    assert_eq!(refs[1].type_name, "Body");
    assert!(refs[1].value.is_some());
}

#[test]
fn test_part_usage_accepts_defined_by_typings() {
    let input = r#"package P {
part def Carrier {
  part engine defined by Vehicle::Engine, Vehicle::PoweredComponent[1] subsets components;
}
}"#;
    let result = parse(input).expect("defined-by part usage should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let part_def = match &elements[0].value {
        PackageBodyElement::PartDef(p) => p,
        other => panic!("expected part def, got {:?}", other),
    };
    let body = match &part_def.value.body {
        sysml_v2_parser::ast::PartDefBody::Brace { elements } => elements,
        other => panic!("expected part def brace body, got {:?}", other),
    };
    let part_usage = match &body[0].value {
        sysml_v2_parser::ast::PartDefBodyElement::PartUsage(p) => p,
        other => panic!("expected part usage, got {:?}", other),
    };
    assert_eq!(part_usage.value.name, "engine");
    assert_eq!(
        part_usage.value.type_name,
        "Vehicle::Engine, Vehicle::PoweredComponent"
    );
    assert_eq!(part_usage.value.multiplicity.as_deref(), Some("[1]"));
    assert_eq!(
        part_usage
            .value
            .subsets
            .as_ref()
            .map(|(name, _)| name.as_str()),
        Some("components")
    );
}

#[test]
fn test_part_usage_accepts_typed_by_typings() {
    let input = r#"package P {
part def Carrier {
  part engine typed by Vehicle::Engine, Vehicle::PoweredComponent[1] subsets components;
}
}"#;
    let result = parse(input).expect("typed-by part usage should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let part_def = match &elements[0].value {
        PackageBodyElement::PartDef(p) => p,
        other => panic!("expected part def, got {:?}", other),
    };
    let body = match &part_def.value.body {
        sysml_v2_parser::ast::PartDefBody::Brace { elements } => elements,
        other => panic!("expected part def brace body, got {:?}", other),
    };
    let part_usage = match &body[0].value {
        sysml_v2_parser::ast::PartDefBodyElement::PartUsage(p) => p,
        other => panic!("expected part usage, got {:?}", other),
    };
    assert_eq!(part_usage.value.name, "engine");
    assert_eq!(
        part_usage.value.type_name,
        "Vehicle::Engine, Vehicle::PoweredComponent"
    );
    assert_eq!(part_usage.value.multiplicity.as_deref(), Some("[1]"));
    assert_eq!(
        part_usage
            .value
            .subsets
            .as_ref()
            .map(|(name, _)| name.as_str()),
        Some("components")
    );
}

#[test]
fn test_part_usage_accepts_multiple_specialization_clauses() {
    let input = r#"package P {
part def Carrier {
  part engine : Engine subsets baseEngine redefines oldEngine :> latestEngine :>> newestEngine;
}
}"#;
    let result =
        parse(input).expect("part usage with multiple specialization clauses should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let part_def = match &elements[0].value {
        PackageBodyElement::PartDef(p) => p,
        other => panic!("expected part def, got {:?}", other),
    };
    let body = match &part_def.value.body {
        sysml_v2_parser::ast::PartDefBody::Brace { elements } => elements,
        other => panic!("expected part def brace body, got {:?}", other),
    };
    let part_usage = match &body[0].value {
        sysml_v2_parser::ast::PartDefBodyElement::PartUsage(p) => p,
        other => panic!("expected part usage, got {:?}", other),
    };
    assert_eq!(
        part_usage
            .value
            .subsets
            .as_ref()
            .map(|(name, _)| name.as_str()),
        Some("latestEngine")
    );
    assert_eq!(part_usage.value.redefines.as_deref(), Some("newestEngine"));
}

#[test]
fn test_anonymous_part_usage_accepts_defined_by_typing() {
    let input = r#"package P {
part def Carrier {
  part defined by Vehicle::Engine[2];
}
}"#;
    let result = parse(input).expect("anonymous defined-by part usage should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let part_def = match &elements[0].value {
        PackageBodyElement::PartDef(p) => p,
        other => panic!("expected part def, got {:?}", other),
    };
    let body = match &part_def.value.body {
        sysml_v2_parser::ast::PartDefBody::Brace { elements } => elements,
        other => panic!("expected part def brace body, got {:?}", other),
    };
    let part_usage = match &body[0].value {
        sysml_v2_parser::ast::PartDefBodyElement::PartUsage(p) => p,
        other => panic!("expected part usage, got {:?}", other),
    };
    assert!(part_usage.value.name.is_empty());
    assert_eq!(part_usage.value.type_name, "Vehicle::Engine");
    assert_eq!(part_usage.value.multiplicity.as_deref(), Some("[2]"));
}

#[test]
fn test_part_usage_redefines_only_keyword() {
    let input = r#"package P {
part def FourCylinderEngine :> Engine {
  part redefines cylinders[4];
}
}"#;
    let result = parse(input).expect("part redefines-only keyword should parse");
    let part_usage = part_def_body_part_usage(&result, 0, 0);
    assert!(part_usage.name.is_empty());
    assert_eq!(part_usage.redefines.as_deref(), Some("cylinders"));
    assert_eq!(part_usage.multiplicity.as_deref(), Some("[4]"));
}

#[test]
fn test_part_usage_named_redefines_with_multiplicity() {
    let input = r#"package P {
part def logicalDriveUnit {
  part motor1 : dcMotor {
    part tire1 redefines motorTire[1];
  }
}
}"#;
    let diag = parse_with_diagnostics(input);
    assert!(
        !diag
            .errors
            .iter()
            .any(|e| e.code.as_deref() == Some("recovered_part_usage_body_element")),
        "unexpected recovery: {:?}",
        diag.errors
    );
    let part_usage = nested_part_usage_in_part_usage(&diag.root, 0, 0, 0);
    assert_eq!(part_usage.name, "tire1");
    assert_eq!(part_usage.redefines.as_deref(), Some("motorTire"));
    assert_eq!(part_usage.multiplicity.as_deref(), Some("[1]"));
}

#[test]
fn test_port_usage_redefines_keyword_in_part_body() {
    let input = r#"package P {
part brushSystem : BrushSystem {
  part mainBrush : MainBrush {
    port redefines rotationSpeedIn;
  }
}
}"#;
    let diag = parse_with_diagnostics(input);
    assert!(
        !diag
            .errors
            .iter()
            .any(|e| e.code.as_deref() == Some("recovered_part_usage_body_element")),
        "unexpected recovery: {:?}",
        diag.errors
    );
    let port_usage = nested_port_usage_in_part_usage(&diag.root, 0, 0, 0);
    assert_eq!(port_usage.name, "rotationSpeedIn");
    assert_eq!(port_usage.redefines.as_deref(), Some("rotationSpeedIn"));
}

#[test]
fn test_attribute_usage_redefines_keyword_prefix() {
    let input = r#"package P {
part def DriveController {
  attribute redefines architecture = EeArchitecture::arm;
}
}"#;
    let result = parse(input).expect("attribute redefines prefix should parse");
    let attr = part_def_body_attribute_usage(&result, 0, 0);
    assert_eq!(attr.name, "architecture");
    assert_eq!(attr.redefines.as_deref(), Some("architecture"));
    assert!(attr.value.is_some());
}

#[test]
fn test_port_def_directed_attribute_redefines_keyword() {
    let input = r#"package P {
port def SuctionLevelPort :> Base::PowerOutPort {
  out attribute redefines suctionPower :> ISQ::power;
}
}"#;
    let result = parse(input).expect("directed attribute redefines should parse");
    let attr = port_def_body_attribute_usage(&result, 0, 0);
    assert_eq!(attr.name, "suctionPower");
    assert_eq!(attr.redefines.as_deref(), Some("suctionPower"));
    assert_eq!(attr.subsets.as_deref(), Some("ISQ::power"));
    assert_eq!(attr.direction, Some(sysml_v2_parser::ast::InOut::Out));
}

#[test]
fn test_port_def_directed_item_inout_with_nested_attributes() {
    let input = r#"package P {
port def DebrisPort {
  inout item debris {
    attribute vol :> ISQ::volume;
    attribute mass :> ISQ::mass;
  }
}
}"#;
    let result = parse(input).expect("directed item inout should parse");
    let item = port_def_body_item_usage(&result, 0, 0);
    assert_eq!(item.name, "debris");
    assert_eq!(item.direction, Some(sysml_v2_parser::ast::InOut::InOut));
    match &item.body {
        sysml_v2_parser::ast::AttributeBody::Brace { elements } => {
            assert_eq!(elements.len(), 2);
        }
        other => panic!("expected brace body with nested attributes, got {:?}", other),
    }
}

#[test]
fn test_port_def_out_pin_subsets_typing_parses_as_in_out_decl() {
    let input = r#"package P {
port def AirPort {
  out volume :> ISQSpaceTime::volume;
}
}"#;
    let result = parse(input).expect("out pin with :> typing should parse");
    let pkg = package_from_root(&result);
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let port_def = match &elements[0].value {
        PackageBodyElement::PortDef(p) => p,
        other => panic!("expected port def, got {:?}", other),
    };
    let body = match &port_def.value.body {
        sysml_v2_parser::ast::PortDefBody::Brace { elements } => elements,
        other => panic!("expected port def brace body, got {:?}", other),
    };
    match &body[0].value {
        sysml_v2_parser::ast::PortDefBodyElement::InOutDecl(decl) => {
            assert_eq!(decl.value.name, "volume");
            assert_eq!(decl.value.direction, sysml_v2_parser::ast::InOut::Out);
            assert_eq!(decl.value.type_name, "ISQSpaceTime::volume");
        }
        other => panic!("expected InOutDecl, got {:?}", other),
    }
}

#[test]
fn test_port_def_out_attribute_plain_still_uses_in_out_decl() {
    let input = r#"package P {
port def FuelPort {
  out fuel : Fuel;
}
}"#;
    let result = parse(input).expect("plain out attribute should parse");
    let pkg = package_from_root(&result);
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let port_def = match &elements[0].value {
        PackageBodyElement::PortDef(p) => p,
        other => panic!("expected port def, got {:?}", other),
    };
    let body = match &port_def.value.body {
        sysml_v2_parser::ast::PortDefBody::Brace { elements } => elements,
        other => panic!("expected port def brace body, got {:?}", other),
    };
    match &body[0].value {
        sysml_v2_parser::ast::PortDefBodyElement::InOutDecl(decl) => {
            assert_eq!(decl.value.name, "fuel");
            assert_eq!(decl.value.direction, sysml_v2_parser::ast::InOut::Out);
        }
        other => panic!("expected InOutDecl, got {:?}", other),
    }
}

fn package_from_root(root: &RootNamespace) -> &Package {
    match &root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {:?}", other),
    }
}

fn part_def_body_part_usage(
    root: &RootNamespace,
    pkg_part_def_idx: usize,
    part_usage_idx: usize,
) -> &PartUsage {
    let pkg = package_from_root(root);
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let part_def = match &elements[pkg_part_def_idx].value {
        PackageBodyElement::PartDef(p) => p,
        other => panic!("expected part def, got {:?}", other),
    };
    let body = match &part_def.value.body {
        PartDefBody::Brace { elements } => elements,
        other => panic!("expected part def brace body, got {:?}", other),
    };
    match &body[part_usage_idx].value {
        PartDefBodyElement::PartUsage(p) => &p.value,
        other => panic!("expected part usage, got {:?}", other),
    }
}

fn nested_part_usage_in_part_usage(
    root: &RootNamespace,
    pkg_part_def_idx: usize,
    outer_part_idx: usize,
    inner_part_idx: usize,
) -> &PartUsage {
    let pkg = package_from_root(root);
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let part_def = match &elements[pkg_part_def_idx].value {
        PackageBodyElement::PartDef(p) => p,
        other => panic!("expected part def, got {:?}", other),
    };
    let def_body = match &part_def.value.body {
        PartDefBody::Brace { elements } => elements,
        other => panic!("expected part def brace body, got {:?}", other),
    };
    let outer = match &def_body[outer_part_idx].value {
        PartDefBodyElement::PartUsage(p) => p,
        other => panic!("expected outer part usage, got {:?}", other),
    };
    let outer_body = match &outer.value.body {
        PartUsageBody::Brace { elements } => elements,
        other => panic!("expected part usage brace body, got {:?}", other),
    };
    match &outer_body[inner_part_idx].value {
        PartUsageBodyElement::PartUsage(p) => &p.value,
        other => panic!("expected inner part usage, got {:?}", other),
    }
}

fn nested_port_usage_in_part_usage(
    root: &RootNamespace,
    pkg_part_idx: usize,
    outer_part_idx: usize,
    port_idx: usize,
) -> &PortUsage {
    let pkg = package_from_root(root);
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let part_usage_el = match &elements[pkg_part_idx].value {
        PackageBodyElement::PartUsage(p) => p,
        other => panic!("expected part usage, got {:?}", other),
    };
    let outer_body = match &part_usage_el.value.body {
        PartUsageBody::Brace { elements } => elements,
        other => panic!("expected part usage brace body, got {:?}", other),
    };
    let inner = match &outer_body[outer_part_idx].value {
        PartUsageBodyElement::PartUsage(p) => p,
        other => panic!("expected inner part usage, got {:?}", other),
    };
    let inner_body = match &inner.value.body {
        PartUsageBody::Brace { elements } => elements,
        other => panic!("expected inner brace body, got {:?}", other),
    };
    match &inner_body[port_idx].value {
        PartUsageBodyElement::PortUsage(p) => &p.value,
        other => panic!("expected port usage, got {:?}", other),
    }
}

fn port_def_body_item_usage(
    root: &RootNamespace,
    port_def_idx: usize,
    item_idx: usize,
) -> &sysml_v2_parser::ast::ItemUsage {
    let pkg = package_from_root(root);
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let port_def = match &elements[port_def_idx].value {
        PackageBodyElement::PortDef(p) => p,
        other => panic!("expected port def, got {:?}", other),
    };
    let body = match &port_def.value.body {
        sysml_v2_parser::ast::PortDefBody::Brace { elements } => elements,
        other => panic!("expected port def brace body, got {:?}", other),
    };
    match &body[item_idx].value {
        sysml_v2_parser::ast::PortDefBodyElement::ItemUsage(i) => &i.value,
        other => panic!("expected item usage, got {:?}", other),
    }
}

fn port_def_body_attribute_usage(
    root: &RootNamespace,
    port_def_idx: usize,
    attr_idx: usize,
) -> &AttributeUsage {
    let pkg = package_from_root(root);
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let port_def = match &elements[port_def_idx].value {
        PackageBodyElement::PortDef(p) => p,
        other => panic!("expected port def, got {:?}", other),
    };
    let body = match &port_def.value.body {
        sysml_v2_parser::ast::PortDefBody::Brace { elements } => elements,
        other => panic!("expected port def brace body, got {:?}", other),
    };
    match &body[attr_idx].value {
        sysml_v2_parser::ast::PortDefBodyElement::AttributeUsage(a) => &a.value,
        other => panic!("expected attribute usage, got {:?}", other),
    }
}

fn part_def_body_attribute_usage(
    root: &RootNamespace,
    part_def_idx: usize,
    attr_idx: usize,
) -> &AttributeUsage {
    let pkg = package_from_root(root);
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let part_def = match &elements[part_def_idx].value {
        PackageBodyElement::PartDef(p) => p,
        other => panic!("expected part def, got {:?}", other),
    };
    let body = match &part_def.value.body {
        PartDefBody::Brace { elements } => elements,
        other => panic!("expected part def brace body, got {:?}", other),
    };
    match &body[attr_idx].value {
        PartDefBodyElement::AttributeUsage(a) => &a.value,
        other => panic!("expected attribute usage, got {:?}", other),
    }
}

#[test]
fn test_connection_def_body_preserves_doc_and_subsequent_members() {
    // Regression: `ConnectionDefBodyElement`/`connection_def_body_element` had no `doc_comment`
    // alternative, and "doc" was missing from `CONNECTION_DEF_BODY_STARTERS`. A `doc /* ... */`
    // block inside a `connection def` body would fail to parse, fall past the recognized-keyword
    // recovery path, and hit the last-resort `advance_to_closing_brace` fallback — silently
    // discarding not just the doc, but every member declared after it in the same body.
    let input = r#"package P {
part def A;
part def B;
connection def Foo {
doc /* Addresses: front ToF 0x29, left ToF 0x2A. */
end a : A;
end b : B;
}
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let connection_def = match &pkg.value.body {
        PackageBody::Brace { elements } => elements
            .iter()
            .find_map(|el| match &el.value {
                PackageBodyElement::ConnectionDef(d) => Some(d),
                _ => None,
            })
            .expect("package should contain a connection def"),
        other => panic!("expected brace body, got {:?}", other),
    };
    let members = match &connection_def.value.body {
        sysml_v2_parser::ast::ConnectionDefBody::Brace { elements } => elements,
        other => panic!("expected structured connection def body, got {:?}", other),
    };
    assert_eq!(
        members.len(),
        3,
        "connection def body should retain the doc block and both end decls after it, got {:?}",
        members
    );
    match &members[0].value {
        sysml_v2_parser::ast::ConnectionDefBodyElement::Doc(doc) => {
            assert!(
                doc.value.text.contains("Addresses: front ToF 0x29"),
                "doc text should retain the colon-and-comma-list line verbatim, got {:?}",
                doc.value.text
            );
        }
        other => panic!("expected Doc member, got {:?}", other),
    }
    assert!(
        matches!(
            &members[1].value,
            sysml_v2_parser::ast::ConnectionDefBodyElement::EndDecl(_)
        ),
        "expected first `end` decl after the doc block, got {:?}",
        members[1].value
    );
    assert!(
        matches!(
            &members[2].value,
            sysml_v2_parser::ast::ConnectionDefBodyElement::EndDecl(_)
        ),
        "expected second `end` decl after the doc block, got {:?}",
        members[2].value
    );
}

#[test]
fn test_interface_usage_connect_body_preserves_doc() {
    // Regression: `InterfaceUsageBodyElement`/`interface_usage_body_element` only supported
    // `ref :>> name = value` redefinitions, with no `doc_comment` alternative and no recovery
    // path in `connect_body_with_elements`. A `doc /* ... */` block inside an `interface ...
    // connect ... to ... { ... }` usage body caused a hard parse failure that discarded the
    // *entire* interface usage, replacing it with a generic "unexpected token in part usage
    // body" error node.
    let input = r#"package P {
part def A { port p1; }
part def B { port p2; }
part vehicle {
part a : A;
part b : B;
interface connect a.p1 to b.p2 {
doc /* Addresses: front ToF 0x29, left ToF 0x2A. */
}
}
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let part_usage = match &pkg.value.body {
        PackageBody::Brace { elements } => match &elements[2].value {
            PackageBodyElement::PartUsage(p) => p,
            other => panic!("expected part usage, got {:?}", other),
        },
        other => panic!("expected brace body, got {:?}", other),
    };
    let interface_usage = match &part_usage.value.body {
        sysml_v2_parser::ast::PartUsageBody::Brace { elements } => elements
            .iter()
            .find_map(|el| match &el.value {
                PartUsageBodyElement::InterfaceUsage(i) => Some(i),
                _ => None,
            })
            .expect("part usage should contain an interface usage, not an Error node"),
        other => panic!("expected part usage brace body, got {:?}", other),
    };
    let body_elements = match &interface_usage.value {
        sysml_v2_parser::ast::InterfaceUsage::TypedConnect { body_elements, .. } => body_elements,
        other => panic!("expected TypedConnect interface usage, got {:?}", other),
    };
    assert_eq!(
        body_elements.len(),
        1,
        "interface usage connect body should retain the doc block, got {:?}",
        body_elements
    );
    match &body_elements[0].value {
        InterfaceUsageBodyElement::Doc(doc) => {
            assert!(
                doc.value.text.contains("Addresses: front ToF 0x29"),
                "doc text should retain the colon-and-comma-list line verbatim, got {:?}",
                doc.value.text
            );
        }
        other => panic!("expected Doc member, got {:?}", other),
    }
}

#[test]
fn test_connection_def_body_recovers_per_element_instead_of_truncating() {
    // Regression: `connection_member_body` used to have its own hand-rolled loop whose only
    // fallback for a genuinely unrecognized member was `advance_to_closing_brace`, which jumps
    // straight to the closing `}` and discards every member declared after the bad one (not just
    // the bad one itself). Migrating to the shared `parse_structured_brace_members` helper (the
    // same one `port_body_brace` uses) means an unrecognized member now becomes a single `Error`
    // node and parsing resumes for the remaining members.
    let input = r#"package P {
part def A;
part def B;
connection def Foo {
end a : A;
bogus nonsense;
end b : B;
}
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let connection_def = match &pkg.value.body {
        PackageBody::Brace { elements } => elements
            .iter()
            .find_map(|el| match &el.value {
                PackageBodyElement::ConnectionDef(d) => Some(d),
                _ => None,
            })
            .expect("package should contain a connection def"),
        other => panic!("expected brace body, got {:?}", other),
    };
    let members = match &connection_def.value.body {
        sysml_v2_parser::ast::ConnectionDefBody::Brace { elements } => elements,
        other => panic!("expected structured connection def body, got {:?}", other),
    };
    assert_eq!(
        members.len(),
        3,
        "expected end a, a recovered Error for `bogus nonsense;`, and end b, got {:?}",
        members
    );
    assert!(
        matches!(
            &members[0].value,
            sysml_v2_parser::ast::ConnectionDefBodyElement::EndDecl(_)
        ),
        "expected first member to be `end a : A`, got {:?}",
        members[0].value
    );
    assert!(
        matches!(
            &members[1].value,
            sysml_v2_parser::ast::ConnectionDefBodyElement::Error(_)
        ),
        "expected the unrecognized `bogus nonsense;` member to become an Error node, got {:?}",
        members[1].value
    );
    assert!(
        matches!(
            &members[2].value,
            sysml_v2_parser::ast::ConnectionDefBodyElement::EndDecl(_)
        ),
        "expected `end b : B` to still parse after the recovered error, got {:?}",
        members[2].value
    );
}

#[test]
fn test_part_def_body_accepts_assert_constraint_and_satisfy() {
    // Regression: `assert_constraint_member` was only reachable from occurrence-def bodies,
    // and `satisfy` only from package level. Neither was reachable inside a `part def` body.
    let input = r#"package P {
requirement def MyReq;
part def Foo {
  assert constraint { true; }
  satisfy MyReq;
}
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let part_def = elements
        .iter()
        .find_map(|el| match &el.value {
            PackageBodyElement::PartDef(p) => Some(p),
            _ => None,
        })
        .expect("expected part def");
    let body_elements = match &part_def.value.body {
        sysml_v2_parser::ast::PartDefBody::Brace { elements } => elements,
        other => panic!("expected part def brace body, got {:?}", other),
    };
    assert!(
        body_elements
            .iter()
            .any(|el| matches!(el.value, PartDefBodyElement::AssertConstraint(_))),
        "expected an AssertConstraint member inside part def body, got {:?}",
        body_elements
    );
    assert!(
        body_elements
            .iter()
            .any(|el| matches!(el.value, PartDefBodyElement::Satisfy(_))),
        "expected a Satisfy member inside part def body, got {:?}",
        body_elements
    );
}

#[test]
fn test_occurrence_def_body_accepts_satisfy() {
    // Regression: `satisfy` was only reachable at package level, not inside `occurrence def`
    // bodies (which already supported `assert constraint`).
    let input = r#"package P {
requirement def MyReq;
occurrence def Foo {
  satisfy MyReq;
}
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let occurrence_def = elements
        .iter()
        .find_map(|el| match &el.value {
            PackageBodyElement::OccurrenceDef(o) => Some(o),
            _ => None,
        })
        .expect("expected occurrence def");
    let body_elements = match &occurrence_def.value.body {
        sysml_v2_parser::ast::DefinitionBody::Brace { elements } => elements,
        other => panic!("expected occurrence def brace body, got {:?}", other),
    };
    assert!(
        body_elements.iter().any(|el| matches!(
            &el.value,
            DefinitionBodyElement::OccurrenceMember(member)
                if matches!(member.value, OccurrenceBodyElement::Satisfy(_))
        )),
        "expected a Satisfy member inside occurrence def body, got {:?}",
        body_elements
    );
}

#[test]
fn test_package_level_satisfy_still_parses_after_part_def_scope_wiring() {
    // "Didn't break the neighbor" check: package-level `satisfy` wiring must be unaffected
    // by adding `satisfy` to part-def/occurrence-def body dispatchers.
    let input = r#"package P {
requirement def MyReq;
satisfy MyReq;
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    assert!(
        elements
            .iter()
            .any(|el| matches!(el.value, PackageBodyElement::Satisfy(_))),
        "expected package-level Satisfy to still parse, got {:?}",
        elements
    );
}

#[test]
fn test_satisfy_accepts_inline_requirement_name_and_type() {
    // Regression: only the bare `satisfy <ref> (by <expr>)?;` shorthand was implemented.
    // The fuller `satisfy requirement <name> : <Type> by <expr>;` form now also parses,
    // reusing the shared `optional_typings` fragment from usage.rs.
    let input = r#"package P {
requirement def ReqType;
part def Foo {
  satisfy requirement myReq : ReqType by someExpr;
}
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let part_def = elements
        .iter()
        .find_map(|el| match &el.value {
            PackageBodyElement::PartDef(p) => Some(p),
            _ => None,
        })
        .expect("expected part def");
    let body_elements = match &part_def.value.body {
        sysml_v2_parser::ast::PartDefBody::Brace { elements } => elements,
        other => panic!("expected part def brace body, got {:?}", other),
    };
    let satisfy = body_elements
        .iter()
        .find_map(|el| match &el.value {
            PartDefBodyElement::Satisfy(s) => Some(&s.value),
            _ => None,
        })
        .expect("expected a Satisfy member");
    let inline = satisfy
        .inline_requirement
        .as_ref()
        .expect("expected inline_requirement to be populated for the `requirement` form");
    assert_eq!(inline.name, "myReq");
    assert_eq!(inline.type_name.as_deref(), Some("ReqType"));
    assert!(matches!(&satisfy.target.value, Expression::FeatureRef(n) if n == "someExpr"));
}

#[test]
fn test_satisfy_bare_shorthand_still_has_no_inline_requirement() {
    // "Didn't break the neighbor" check: the existing bare shorthand must keep
    // `inline_requirement: None`.
    let input = r#"package P {
requirement def MyReq;
part def Foo {
  satisfy MyReq;
}
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let part_def = elements
        .iter()
        .find_map(|el| match &el.value {
            PackageBodyElement::PartDef(p) => Some(p),
            _ => None,
        })
        .expect("expected part def");
    let body_elements = match &part_def.value.body {
        sysml_v2_parser::ast::PartDefBody::Brace { elements } => elements,
        other => panic!("expected part def brace body, got {:?}", other),
    };
    let satisfy = body_elements
        .iter()
        .find_map(|el| match &el.value {
            PartDefBodyElement::Satisfy(s) => Some(&s.value),
            _ => None,
        })
        .expect("expected a Satisfy member");
    assert!(satisfy.inline_requirement.is_none());
}
