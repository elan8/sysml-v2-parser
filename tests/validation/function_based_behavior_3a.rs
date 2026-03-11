//! Parser test for `03-Function-based Behavior/3a-Function-based Behavior-1.sysml`.

use std::path::Path;
use sysml_parser::ast::{
    ActionDef, ActionDefBody, ActionUsage, ActionUsageBody, ActionUsageBodyElement, AliasBody,
    AliasDef, AttributeBody, AttributeDef, Bind, ConnectBody, Expression, FirstMergeBody, FirstStmt,
    Flow, Identification, InOut, InOutDecl, MergeStmt, Package, PackageBody, PackageBodyElement,
    RootNamespace, Visibility,
};
use sysml_parser::parse;

fn id(name: &str) -> Identification {
    Identification {
        short_name: None,
        name: Some(name.to_string()),
    }
}

fn expr_path(path: &str) -> Expression {
    let segments: Vec<&str> = path.split('.').collect();
    let mut expr = Expression::FeatureRef(segments[0].to_string());
    for seg in segments.iter().skip(1) {
        expr = Expression::MemberAccess(Box::new(expr), (*seg).to_string());
    }
    expr
}

fn expected_ast() -> RootNamespace {
    RootNamespace {
        elements: vec![PackageBodyElement::Package(Package {
            identification: id("3a-Function-based Behavior-1"),
            body: PackageBody::Brace {
                elements: vec![
                    PackageBodyElement::Import(sysml_parser::ast::Import {
                        visibility: Some(Visibility::Public),
                        is_import_all: true,
                        target: "Definitions::*".to_string(),
                    }),
                    PackageBodyElement::Import(sysml_parser::ast::Import {
                        visibility: Some(Visibility::Public),
                        is_import_all: true,
                        target: "Usages::*".to_string(),
                    }),
                    PackageBodyElement::Package(definitions_package()),
                    PackageBodyElement::Package(usages_package()),
                ],
            },
        })],
    }
}

fn definitions_package() -> Package {
    Package {
        identification: id("Definitions"),
        body: PackageBody::Brace {
            elements: vec![
                PackageBodyElement::AliasDef(AliasDef {
                    identification: id("Torque"),
                    target: "ISQ::TorqueValue".to_string(),
                    body: AliasBody::Brace,
                }),
                PackageBodyElement::AttributeDef(AttributeDef {
                    name: "FuelCmd".to_string(),
                    typing: None,
                    body: AttributeBody::Semicolon,
                }),
                PackageBodyElement::AttributeDef(AttributeDef {
                    name: "EngineStart".to_string(),
                    typing: None,
                    body: AttributeBody::Semicolon,
                }),
                PackageBodyElement::AttributeDef(AttributeDef {
                    name: "EngineOff".to_string(),
                    typing: None,
                    body: AttributeBody::Semicolon,
                }),
                PackageBodyElement::ActionDef(ActionDef {
                    identification: id("Generate Torque"),
                    body: ActionDefBody::Brace {
                        elements: vec![
                            InOutDecl {
                                direction: InOut::In,
                                name: "fuelCmd".to_string(),
                                type_name: "FuelCmd".to_string(),
                            },
                            InOutDecl {
                                direction: InOut::Out,
                                name: "engineTorque".to_string(),
                                type_name: "Torque".to_string(),
                            },
                        ],
                    },
                }),
                PackageBodyElement::ActionDef(ActionDef {
                    identification: id("Amplify Torque"),
                    body: ActionDefBody::Brace {
                        elements: vec![
                            InOutDecl {
                                direction: InOut::In,
                                name: "engineTorque".to_string(),
                                type_name: "Torque".to_string(),
                            },
                            InOutDecl {
                                direction: InOut::Out,
                                name: "transmissionTorque".to_string(),
                                type_name: "Torque".to_string(),
                            },
                        ],
                    },
                }),
                PackageBodyElement::ActionDef(ActionDef {
                    identification: id("Transfer Torque"),
                    body: ActionDefBody::Brace {
                        elements: vec![
                            InOutDecl {
                                direction: InOut::In,
                                name: "transmissionTorque".to_string(),
                                type_name: "Torque".to_string(),
                            },
                            InOutDecl {
                                direction: InOut::Out,
                                name: "driveshaftTorque".to_string(),
                                type_name: "Torque".to_string(),
                            },
                        ],
                    },
                }),
                PackageBodyElement::ActionDef(ActionDef {
                    identification: id("Distribute Torque"),
                    body: ActionDefBody::Brace {
                        elements: vec![
                            InOutDecl {
                                direction: InOut::In,
                                name: "driveShaftTorque".to_string(),
                                type_name: "Torque".to_string(),
                            },
                            InOutDecl {
                                direction: InOut::Out,
                                name: "wheelTorque1".to_string(),
                                type_name: "Torque".to_string(),
                            },
                            InOutDecl {
                                direction: InOut::Out,
                                name: "wheelTorque2".to_string(),
                                type_name: "Torque".to_string(),
                            },
                        ],
                    },
                }),
                PackageBodyElement::ActionDef(ActionDef {
                    identification: id("Provide Power"),
                    body: ActionDefBody::Brace {
                        elements: vec![
                            InOutDecl {
                                direction: InOut::In,
                                name: "fuelCmd".to_string(),
                                type_name: "FuelCmd".to_string(),
                            },
                            InOutDecl {
                                direction: InOut::Out,
                                name: "wheelTorque1".to_string(),
                                type_name: "Torque".to_string(),
                            },
                            InOutDecl {
                                direction: InOut::Out,
                                name: "wheelTorque2".to_string(),
                                type_name: "Torque".to_string(),
                            },
                        ],
                    },
                }),
            ],
        },
    }
}

fn usages_package() -> Package {
    Package {
        identification: id("Usages"),
        body: PackageBody::Brace {
            elements: vec![PackageBodyElement::ActionUsage(provide_power_action())],
        },
    }
}

fn provide_power_action() -> ActionUsage {
    ActionUsage {
        name: "provide power".to_string(),
        type_name: "Provide Power".to_string(),
        accept: None,
        body: ActionUsageBody::Brace {
            elements: vec![
                ActionUsageBodyElement::InOutDecl(InOutDecl {
                    direction: InOut::In,
                    name: "fuelCmd".to_string(),
                    type_name: "FuelCmd".to_string(),
                }),
                ActionUsageBodyElement::InOutDecl(InOutDecl {
                    direction: InOut::Out,
                    name: "wheelTorque1".to_string(),
                    type_name: "Torque".to_string(),
                }),
                ActionUsageBodyElement::InOutDecl(InOutDecl {
                    direction: InOut::Out,
                    name: "wheelTorque2".to_string(),
                    type_name: "Torque".to_string(),
                }),
                ActionUsageBodyElement::Bind(Bind {
                    left: expr_path("generate torque.fuelCmd"),
                    right: expr_path("fuelCmd"),
                    body: Some(ConnectBody::Brace),
                }),
                ActionUsageBodyElement::ActionUsage(Box::new(ActionUsage {
                    name: "generate torque".to_string(),
                    type_name: "Generate Torque".to_string(),
                    accept: None,
                    body: ActionUsageBody::Brace { elements: vec![] },
                })),
                ActionUsageBodyElement::Flow(Flow {
                    from: expr_path("generate torque.engineTorque"),
                    to: expr_path("amplify torque.engineTorque"),
                    body: ConnectBody::Brace,
                }),
                ActionUsageBodyElement::ActionUsage(Box::new(ActionUsage {
                    name: "amplify torque".to_string(),
                    type_name: "Amplify Torque".to_string(),
                    accept: None,
                    body: ActionUsageBody::Semicolon,
                })),
                ActionUsageBodyElement::Flow(Flow {
                    from: expr_path("amplify torque.transmissionTorque"),
                    to: expr_path("transfer torque.transmissionTorque"),
                    body: ConnectBody::Semicolon,
                }),
                ActionUsageBodyElement::ActionUsage(Box::new(ActionUsage {
                    name: "transfer torque".to_string(),
                    type_name: "Transfer Torque".to_string(),
                    accept: None,
                    body: ActionUsageBody::Semicolon,
                })),
                ActionUsageBodyElement::Flow(Flow {
                    from: expr_path("transfer torque.driveshaftTorque"),
                    to: expr_path("distribute torque.driveShaftTorque"),
                    body: ConnectBody::Semicolon,
                }),
                ActionUsageBodyElement::ActionUsage(Box::new(ActionUsage {
                    name: "distribute torque".to_string(),
                    type_name: "Distribute Torque".to_string(),
                    accept: None,
                    body: ActionUsageBody::Semicolon,
                })),
                ActionUsageBodyElement::Bind(Bind {
                    left: expr_path("wheelTorque1"),
                    right: expr_path("distribute torque.wheelTorque1"),
                    body: Some(ConnectBody::Semicolon),
                }),
                ActionUsageBodyElement::Bind(Bind {
                    left: expr_path("wheelTorque2"),
                    right: expr_path("distribute torque.wheelTorque2"),
                    body: Some(ConnectBody::Semicolon),
                }),
                ActionUsageBodyElement::FirstStmt(FirstStmt {
                    first: Expression::FeatureRef("start".to_string()),
                    then: Expression::FeatureRef("continue".to_string()),
                    body: FirstMergeBody::Brace,
                }),
                ActionUsageBodyElement::MergeStmt(MergeStmt {
                    merge: Expression::FeatureRef("continue".to_string()),
                    body: FirstMergeBody::Brace,
                }),
                ActionUsageBodyElement::FirstStmt(FirstStmt {
                    first: Expression::FeatureRef("continue".to_string()),
                    then: Expression::FeatureRef("engineStarted".to_string()),
                    body: FirstMergeBody::Semicolon,
                }),
                ActionUsageBodyElement::ActionUsage(Box::new(ActionUsage {
                    name: "engineStarted".to_string(),
                    type_name: "EngineStart".to_string(),
                    accept: Some(("engineStart".to_string(), "EngineStart".to_string())),
                    body: ActionUsageBody::Brace { elements: vec![] },
                })),
                ActionUsageBodyElement::FirstStmt(FirstStmt {
                    first: Expression::FeatureRef("engineStarted".to_string()),
                    then: Expression::FeatureRef("engineStopped".to_string()),
                    body: FirstMergeBody::Semicolon,
                }),
                ActionUsageBodyElement::ActionUsage(Box::new(ActionUsage {
                    name: "engineStopped".to_string(),
                    type_name: "EngineOff".to_string(),
                    accept: Some(("engineOff".to_string(), "EngineOff".to_string())),
                    body: ActionUsageBody::Semicolon,
                })),
                ActionUsageBodyElement::FirstStmt(FirstStmt {
                    first: Expression::FeatureRef("engineStopped".to_string()),
                    then: Expression::FeatureRef("continue".to_string()),
                    body: FirstMergeBody::Semicolon,
                }),
                ActionUsageBodyElement::FirstStmt(FirstStmt {
                    first: Expression::FeatureRef("engineStarted".to_string()),
                    then: Expression::FeatureRef("generate torque".to_string()),
                    body: FirstMergeBody::Semicolon,
                }),
                ActionUsageBodyElement::FirstStmt(FirstStmt {
                    first: Expression::FeatureRef("engineStarted".to_string()),
                    then: Expression::FeatureRef("amplify torque".to_string()),
                    body: FirstMergeBody::Semicolon,
                }),
                ActionUsageBodyElement::FirstStmt(FirstStmt {
                    first: Expression::FeatureRef("engineStarted".to_string()),
                    then: Expression::FeatureRef("transfer torque".to_string()),
                    body: FirstMergeBody::Semicolon,
                }),
                ActionUsageBodyElement::FirstStmt(FirstStmt {
                    first: Expression::FeatureRef("engineStarted".to_string()),
                    then: Expression::FeatureRef("distribute torque".to_string()),
                    body: FirstMergeBody::Semicolon,
                }),
                ActionUsageBodyElement::FirstStmt(FirstStmt {
                    first: Expression::FeatureRef("generate torque".to_string()),
                    then: Expression::FeatureRef("engineStopped".to_string()),
                    body: FirstMergeBody::Semicolon,
                }),
                ActionUsageBodyElement::FirstStmt(FirstStmt {
                    first: Expression::FeatureRef("amplify torque".to_string()),
                    then: Expression::FeatureRef("engineStopped".to_string()),
                    body: FirstMergeBody::Semicolon,
                }),
                ActionUsageBodyElement::FirstStmt(FirstStmt {
                    first: Expression::FeatureRef("transfer torque".to_string()),
                    then: Expression::FeatureRef("engineStopped".to_string()),
                    body: FirstMergeBody::Semicolon,
                }),
                ActionUsageBodyElement::FirstStmt(FirstStmt {
                    first: Expression::FeatureRef("distribute torque".to_string()),
                    then: Expression::FeatureRef("engineStopped".to_string()),
                    body: FirstMergeBody::Semicolon,
                }),
            ],
        },
    }
}

fn validation_fixture_path(relative: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("sysml-v2-release")
        .join("sysml")
        .join("src")
        .join("validation")
        .join(relative)
}

#[test]
fn test_parse_3a_function_based_behavior() {
    let path = validation_fixture_path("03-Function-based Behavior")
        .join("3a-Function-based Behavior-1.sysml");
    let input = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read fixture {}: {}", path.display(), e));
    let result = parse(&input)
        .expect("parse should succeed for 3a-Function-based Behavior-1.sysml");
    let expected = expected_ast();
    assert_eq!(
        result, expected,
        "parsed AST should match expected for 3a-Function-based Behavior-1.sysml"
    );
}
