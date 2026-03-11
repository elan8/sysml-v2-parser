//! Parser test for `02-Parts Interconnection/2a-Parts Interconnection.sysml`.

use std::path::Path;
use sysml_parser::ast::{
    Bind, Connect, ConnectBody, ConnectStmt, EndDecl, Expression, Identification, Import,
    InterfaceDef, InterfaceDefBody, InterfaceDefBodyElement, InterfaceUsage,
    InterfaceUsageBodyElement, Package, PackageBody, PackageBodyElement, PartDef, PartDefBody,
    PartDefBodyElement, PartUsage, PartUsageBody, PartUsageBodyElement, PortBody, PortDef,
    PortDefBody, PortDefBodyElement, PortUsage, RefBody, RefDecl, RootNamespace, Visibility,
};
use sysml_parser::parse;

fn id(name: &str) -> Identification {
    Identification {
        short_name: None,
        name: Some(name.to_string()),
    }
}

/// Path expression from dot-separated path (e.g. "engine.fuelCmdPort").
fn expr_path(path: &str) -> Expression {
    let segments: Vec<&str> = path.split('.').collect();
    let mut expr = Expression::FeatureRef(segments[0].to_string());
    for seg in segments.iter().skip(1) {
        expr = Expression::MemberAccess(Box::new(expr), (*seg).to_string());
    }
    expr
}

/// Index expression base#(n).
fn expr_index(base: &str, n: i64) -> Expression {
    Expression::Index {
        base: Box::new(Expression::FeatureRef(base.to_string())),
        index: Box::new(Expression::LiteralInteger(n)),
    }
}

/// Expected AST for `2a-Parts Interconnection.sysml`.
fn expected_ast() -> RootNamespace {
    RootNamespace {
        elements: vec![PackageBodyElement::Package(Package {
            identification: id("2a-Parts Interconnection"),
            body: PackageBody::Brace {
                elements: vec![
                    PackageBodyElement::Import(Import {
                        visibility: Some(Visibility::Public),
                        is_import_all: true,
                        target: "Definitions::*".to_string(),
                    }),
                    PackageBodyElement::Import(Import {
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
                // Port defs
                port_def_semicolon("FuelCmdPort"),
                port_def_semicolon("DrivePwrPort"),
                port_def_semicolon("ClutchPort"),
                port_def_semicolon("ShaftPort_a"),
                port_def_semicolon("ShaftPort_b"),
                port_def_semicolon("ShaftPort_c"),
                port_def_semicolon("ShaftPort_d"),
                port_def_semicolon("DiffPort"),
                port_def_semicolon("AxlePort"),
                port_def_semicolon("AxleToWheelPort"),
                port_def_semicolon("WheelToAxlePort"),
                port_def_semicolon("WheelToRoadPort"),
                port_def_vehicle_to_road(),
                // Part defs
                part_def_vehicle_a(),
                PackageBodyElement::PartDef(PartDef {
                    identification: id("AxleAssembly"),
                    specializes: None,
                    body: PartDefBody::Semicolon,
                }),
                part_def_rear_axle_assembly(),
                PackageBodyElement::PartDef(PartDef {
                    identification: id("Axle"),
                    specializes: None,
                    body: PartDefBody::Semicolon,
                }),
                PackageBodyElement::PartDef(PartDef {
                    identification: id("RearAxle"),
                    specializes: Some("Axle".to_string()),
                    body: PartDefBody::Semicolon,
                }),
                part_def_half_axle(),
                part_def_engine(),
                part_def_transmission(),
                part_def_driveshaft(),
                PackageBodyElement::PartDef(PartDef {
                    identification: id("Differential"),
                    specializes: None,
                    body: PartDefBody::Brace { elements: vec![] },
                }),
                PackageBodyElement::PartDef(PartDef {
                    identification: id("Wheel"),
                    specializes: None,
                    body: PartDefBody::Semicolon,
                }),
                // Interface defs
                interface_def_engine_to_transmission(),
                interface_def_driveshaft(),
            ],
        },
    }
}

fn port_def_semicolon(name: &str) -> PackageBodyElement {
    PackageBodyElement::PortDef(PortDef {
        identification: id(name),
        body: PortDefBody::Semicolon,
    })
}

fn port_def_vehicle_to_road() -> PackageBodyElement {
    PackageBodyElement::PortDef(PortDef {
        identification: id("VehicleToRoadPort"),
        body: PortDefBody::Brace {
            elements: vec![PortDefBodyElement::PortUsage(PortUsage {
                name: "wheelToRoadPort".to_string(),
                type_name: Some("WheelToRoadPort".to_string()),
                multiplicity: Some("[2]".to_string()),
                subsets: None,
                redefines: None,
                body: PortBody::Semicolon,
            })],
        },
    })
}

fn part_def_vehicle_a() -> PackageBodyElement {
    PackageBodyElement::PartDef(PartDef {
        identification: id("VehicleA"),
        specializes: None,
        body: PartDefBody::Brace {
            elements: vec![
                PartDefBodyElement::PortUsage(PortUsage {
                    name: "fuelCmdPort".to_string(),
                    type_name: Some("FuelCmdPort".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    body: PortBody::Semicolon,
                }),
                PartDefBodyElement::PortUsage(PortUsage {
                    name: "vehicleToRoadPort".to_string(),
                    type_name: Some("VehicleToRoadPort".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    body: PortBody::Semicolon,
                }),
            ],
        },
    })
}

fn part_def_rear_axle_assembly() -> PackageBodyElement {
    PackageBodyElement::PartDef(PartDef {
        identification: id("RearAxleAssembly"),
        specializes: Some("AxleAssembly".to_string()),
        body: PartDefBody::Brace {
            elements: vec![PartDefBodyElement::PortUsage(PortUsage {
                name: "shaftPort_d".to_string(),
                type_name: Some("ShaftPort_d".to_string()),
                multiplicity: None,
                subsets: None,
                redefines: None,
                body: PortBody::Semicolon,
            })],
        },
    })
}

fn part_def_half_axle() -> PackageBodyElement {
    PackageBodyElement::PartDef(PartDef {
        identification: id("HalfAxle"),
        specializes: None,
        body: PartDefBody::Brace {
            elements: vec![
                PartDefBodyElement::PortUsage(PortUsage {
                    name: "axleToDiffPort".to_string(),
                    type_name: Some("AxlePort".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    body: PortBody::Semicolon,
                }),
                PartDefBodyElement::PortUsage(PortUsage {
                    name: "axleToWheelPort".to_string(),
                    type_name: Some("AxleToWheelPort".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    body: PortBody::Semicolon,
                }),
            ],
        },
    })
}

fn part_def_engine() -> PackageBodyElement {
    PackageBodyElement::PartDef(PartDef {
        identification: id("Engine"),
        specializes: None,
        body: PartDefBody::Brace {
            elements: vec![
                PartDefBodyElement::PortUsage(PortUsage {
                    name: "fuelCmdPort".to_string(),
                    type_name: Some("FuelCmdPort".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    body: PortBody::Semicolon,
                }),
                PartDefBodyElement::PortUsage(PortUsage {
                    name: "drivePwrPort".to_string(),
                    type_name: Some("DrivePwrPort".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    body: PortBody::Semicolon,
                }),
            ],
        },
    })
}

fn part_def_transmission() -> PackageBodyElement {
    PackageBodyElement::PartDef(PartDef {
        identification: id("Transmission"),
        specializes: None,
        body: PartDefBody::Brace {
            elements: vec![
                PartDefBodyElement::PortUsage(PortUsage {
                    name: "clutchPort".to_string(),
                    type_name: Some("ClutchPort".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    body: PortBody::Semicolon,
                }),
                PartDefBodyElement::PortUsage(PortUsage {
                    name: "shaftPort_a".to_string(),
                    type_name: Some("ShaftPort_a".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    body: PortBody::Semicolon,
                }),
            ],
        },
    })
}

fn part_def_driveshaft() -> PackageBodyElement {
    PackageBodyElement::PartDef(PartDef {
        identification: id("Driveshaft"),
        specializes: None,
        body: PartDefBody::Brace {
            elements: vec![
                PartDefBodyElement::PortUsage(PortUsage {
                    name: "shaftPort_b".to_string(),
                    type_name: Some("ShaftPort_b".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    body: PortBody::Semicolon,
                }),
                PartDefBodyElement::PortUsage(PortUsage {
                    name: "shaftPort_c".to_string(),
                    type_name: Some("ShaftPort_c".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    body: PortBody::Semicolon,
                }),
            ],
        },
    })
}

fn interface_def_engine_to_transmission() -> PackageBodyElement {
    PackageBodyElement::InterfaceDef(InterfaceDef {
        identification: id("EngineToTransmissionInterface"),
        body: InterfaceDefBody::Brace {
            elements: vec![
                InterfaceDefBodyElement::EndDecl(EndDecl {
                    name: "drivePwrPort".to_string(),
                    type_name: "DrivePwrPort".to_string(),
                }),
                InterfaceDefBodyElement::EndDecl(EndDecl {
                    name: "clutchPort".to_string(),
                    type_name: "ClutchPort".to_string(),
                }),
            ],
        },
    })
}

fn interface_def_driveshaft() -> PackageBodyElement {
    PackageBodyElement::InterfaceDef(InterfaceDef {
        identification: id("DriveshaftInterface"),
        body: InterfaceDefBody::Brace {
            elements: vec![
                InterfaceDefBodyElement::EndDecl(EndDecl {
                    name: "shaftPort_a".to_string(),
                    type_name: "ShaftPort_a".to_string(),
                }),
                InterfaceDefBodyElement::EndDecl(EndDecl {
                    name: "shaftPort_d".to_string(),
                    type_name: "ShaftPort_d".to_string(),
                }),
                InterfaceDefBodyElement::RefDecl(RefDecl {
                    name: "driveshaft".to_string(),
                    type_name: "Driveshaft".to_string(),
                    body: RefBody::Brace,
                }),
                InterfaceDefBodyElement::ConnectStmt(ConnectStmt {
                    from: Expression::FeatureRef("shaftPort_a".to_string()),
                    to: expr_path("driveshaft.shaftPort_b"),
                    body: ConnectBody::Brace,
                }),
                InterfaceDefBodyElement::ConnectStmt(ConnectStmt {
                    from: expr_path("driveshaft.shaftPort_c"),
                    to: Expression::FeatureRef("shaftPort_d".to_string()),
                    body: ConnectBody::Semicolon,
                }),
            ],
        },
    })
}

fn usages_package() -> Package {
    Package {
        identification: id("Usages"),
        body: PackageBody::Brace {
            elements: vec![PackageBodyElement::PartUsage(part_vehicle1_c1())],
        },
    }
}

fn part_vehicle1_c1() -> PartUsage {
    PartUsage {
        name: "vehicle1_c1".to_string(),
        type_name: "VehicleA".to_string(),
        multiplicity: None,
        ordered: false,
        subsets: None,
        body: PartUsageBody::Brace {
            elements: vec![
                PartUsageBodyElement::Bind(Bind {
                    left: expr_path("fuelCmdPort"),
                    right: expr_path("engine.fuelCmdPort"),
                    body: Some(ConnectBody::Semicolon),
                }),
                PartUsageBodyElement::PartUsage(Box::new(PartUsage {
                    name: "engine".to_string(),
                    type_name: "Engine".to_string(),
                    multiplicity: None,
                    ordered: false,
                    subsets: None,
                    body: PartUsageBody::Semicolon,
                })),
                PartUsageBodyElement::InterfaceUsage(InterfaceUsage::TypedConnect {
                    interface_type: Some("EngineToTransmissionInterface".to_string()),
                    from: expr_path("engine.drivePwrPort"),
                    to: expr_path("transmission.clutchPort"),
                    body: ConnectBody::Brace,
                    body_elements: vec![],
                }),
                PartUsageBodyElement::PartUsage(Box::new(PartUsage {
                    name: "transmission".to_string(),
                    type_name: "Transmission".to_string(),
                    multiplicity: None,
                    ordered: false,
                    subsets: None,
                    body: PartUsageBody::Semicolon,
                })),
                PartUsageBodyElement::PartUsage(Box::new(PartUsage {
                    name: "driveshaft".to_string(),
                    type_name: "Driveshaft".to_string(),
                    multiplicity: None,
                    ordered: false,
                    subsets: None,
                    body: PartUsageBody::Brace { elements: vec![] },
                })),
                PartUsageBodyElement::InterfaceUsage(InterfaceUsage::TypedConnect {
                    interface_type: Some("DriveshaftInterface".to_string()),
                    from: expr_path("transmission.shaftPort_a"),
                    to: expr_path("rearAxleAssembly.shaftPort_d"),
                    body: ConnectBody::Brace,
                    body_elements: vec![InterfaceUsageBodyElement::RefRedef {
                        name: "driveshaft".to_string(),
                        value: expr_path("vehicle1_c1.driveshaft"),
                        body: RefBody::Brace,
                    }],
                }),
                PartUsageBodyElement::PartUsage(Box::new(part_rear_axle_assembly())),
                PartUsageBodyElement::Bind(Bind {
                    left: expr_path("rearAxleAssembly.leftWheel.wheelToRoadPort"),
                    right: expr_path("vehicleToRoadPort.leftWheelToRoadPort"),
                    body: Some(ConnectBody::Semicolon),
                }),
                PartUsageBodyElement::Bind(Bind {
                    left: expr_path("rearAxleAssembly.rightWheel.wheelToRoadPort"),
                    right: expr_path("vehicleToRoadPort.rightWheelToRoadPort"),
                    body: Some(ConnectBody::Semicolon),
                }),
                PartUsageBodyElement::PortUsage(PortUsage {
                    name: "vehicleToRoadPort".to_string(),
                    type_name: None,
                    multiplicity: None,
                    subsets: None,
                    redefines: Some("VehicleA::vehicleToRoadPort".to_string()),
                    body: PortBody::BraceWithPorts {
                        elements: vec![
                            PortUsage {
                                name: "leftWheelToRoadPort".to_string(),
                                type_name: None,
                                multiplicity: None,
                                subsets: Some((
                                    "wheelToRoadPort".to_string(),
                                    Some(expr_index("wheelToRoadPort", 1)),
                                )),
                                redefines: None,
                                body: PortBody::Semicolon,
                            },
                            PortUsage {
                                name: "rightWheelToRoadPort".to_string(),
                                type_name: None,
                                multiplicity: None,
                                subsets: Some((
                                    "wheelToRoadPort".to_string(),
                                    Some(expr_index("wheelToRoadPort", 2)),
                                )),
                                redefines: None,
                                body: PortBody::Semicolon,
                            },
                        ],
                    },
                }),
            ],
        },
    }
}

fn part_rear_axle_assembly() -> PartUsage {
    PartUsage {
        name: "rearAxleAssembly".to_string(),
        type_name: "RearAxleAssembly".to_string(),
        multiplicity: None,
        ordered: false,
        subsets: None,
        body: PartUsageBody::Brace {
            elements: vec![
                PartUsageBodyElement::Bind(Bind {
                    left: expr_path("shaftPort_d"),
                    right: expr_path("differential.shaftPort_d"),
                    body: Some(ConnectBody::Semicolon),
                }),
                PartUsageBodyElement::PartUsage(Box::new(part_differential())),
                PartUsageBodyElement::InterfaceUsage(InterfaceUsage::Connection {
                    from: expr_path("differential.leftDiffPort"),
                    to: expr_path("rearAxle.leftHalfAxle.axleToDiffPort"),
                    body_elements: vec![],
                }),
                PartUsageBodyElement::InterfaceUsage(InterfaceUsage::Connection {
                    from: expr_path("differential.rightDiffPort"),
                    to: expr_path("rearAxle.rightHalfAxle.axleToDiffPort"),
                    body_elements: vec![],
                }),
                PartUsageBodyElement::PartUsage(Box::new(part_rear_axle())),
                PartUsageBodyElement::Connect(Connect {
                    from: expr_path("rearAxle.leftHalfAxle.axleToWheelPort"),
                    to: expr_path("leftWheel.wheelToAxlePort"),
                    body: ConnectBody::Semicolon,
                }),
                PartUsageBodyElement::Connect(Connect {
                    from: expr_path("rearAxle.rightHalfAxle.axleToWheelPort"),
                    to: expr_path("rightWheel.wheelToAxlePort"),
                    body: ConnectBody::Semicolon,
                }),
                PartUsageBodyElement::PartUsage(Box::new(PartUsage {
                    name: "rearWheel".to_string(),
                    type_name: "Wheel".to_string(),
                    multiplicity: Some("[2]".to_string()),
                    ordered: true,
                    subsets: None,
                    body: PartUsageBody::Semicolon,
                })),
                PartUsageBodyElement::PartUsage(Box::new(PartUsage {
                    name: "leftWheel".to_string(),
                    type_name: "".to_string(),
                    multiplicity: None,
                    ordered: false,
                    subsets: Some((
                        "rearWheel".to_string(),
                        Some(expr_index("rearWheel", 1)),
                    )),
                    body: PartUsageBody::Brace {
                        elements: vec![
                            PartUsageBodyElement::PortUsage(PortUsage {
                                name: "wheelToAxlePort".to_string(),
                                type_name: Some("WheelToAxlePort".to_string()),
                                multiplicity: None,
                                subsets: None,
                                redefines: None,
                                body: PortBody::Semicolon,
                            }),
                            PartUsageBodyElement::PortUsage(PortUsage {
                                name: "wheelToRoadPort".to_string(),
                                type_name: Some("WheelToRoadPort".to_string()),
                                multiplicity: None,
                                subsets: None,
                                redefines: None,
                                body: PortBody::Semicolon,
                            }),
                        ],
                    },
                })),
                PartUsageBodyElement::PartUsage(Box::new(PartUsage {
                    name: "rightWheel".to_string(),
                    type_name: "".to_string(),
                    multiplicity: None,
                    ordered: false,
                    subsets: Some((
                        "rearWheel".to_string(),
                        Some(expr_index("rearWheel", 2)),
                    )),
                    body: PartUsageBody::Brace {
                        elements: vec![
                            PartUsageBodyElement::PortUsage(PortUsage {
                                name: "wheelToAxlePort".to_string(),
                                type_name: Some("WheelToAxlePort".to_string()),
                                multiplicity: None,
                                subsets: None,
                                redefines: None,
                                body: PortBody::Semicolon,
                            }),
                            PartUsageBodyElement::PortUsage(PortUsage {
                                name: "wheelToRoadPort".to_string(),
                                type_name: Some("WheelToRoadPort".to_string()),
                                multiplicity: None,
                                subsets: None,
                                redefines: None,
                                body: PortBody::Semicolon,
                            }),
                        ],
                    },
                })),
            ],
        },
    }
}

fn part_differential() -> PartUsage {
    PartUsage {
        name: "differential".to_string(),
        type_name: "Differential".to_string(),
        multiplicity: None,
        ordered: false,
        subsets: None,
        body: PartUsageBody::Brace {
            elements: vec![
                PartUsageBodyElement::PortUsage(PortUsage {
                    name: "shaftPort_d".to_string(),
                    type_name: Some("ShaftPort_d".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    body: PortBody::Brace,
                }),
                PartUsageBodyElement::PortUsage(PortUsage {
                    name: "leftDiffPort".to_string(),
                    type_name: Some("DiffPort".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    body: PortBody::Semicolon,
                }),
                PartUsageBodyElement::PortUsage(PortUsage {
                    name: "rightDiffPort".to_string(),
                    type_name: Some("DiffPort".to_string()),
                    multiplicity: None,
                    subsets: None,
                    redefines: None,
                    body: PortBody::Semicolon,
                }),
            ],
        },
    }
}

fn part_rear_axle() -> PartUsage {
    PartUsage {
        name: "rearAxle".to_string(),
        type_name: "RearAxle".to_string(),
        multiplicity: None,
        ordered: false,
        subsets: None,
        body: PartUsageBody::Brace {
            elements: vec![
                PartUsageBodyElement::PartUsage(Box::new(PartUsage {
                    name: "leftHalfAxle".to_string(),
                    type_name: "HalfAxle".to_string(),
                    multiplicity: None,
                    ordered: false,
                    subsets: None,
                    body: PartUsageBody::Semicolon,
                })),
                PartUsageBodyElement::PartUsage(Box::new(PartUsage {
                    name: "rightHalfAxle".to_string(),
                    type_name: "HalfAxle".to_string(),
                    multiplicity: None,
                    ordered: false,
                    subsets: None,
                    body: PartUsageBody::Semicolon,
                })),
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
fn test_parse_2a_parts_interconnection() {
    let path = validation_fixture_path("02-Parts Interconnection").join("2a-Parts Interconnection.sysml");
    let input = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read fixture {}: {}", path.display(), e));
    let result = parse(&input).expect("parse should succeed for 2a-Parts Interconnection.sysml");
    let expected = expected_ast();
    assert_eq!(
        result, expected,
        "parsed AST should match expected for 2a-Parts Interconnection.sysml"
    );
}
