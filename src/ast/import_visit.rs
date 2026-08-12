//! Exhaustive traversal of the AST positions that can own import-like targets.
//!
//! This is intentionally typed rather than serialization-driven. Each containing enum is
//! matched without a wildcard so adding a new nesting route becomes a compile error here.

use super::*;

pub(super) fn validate_import_target_provenance(document: &ParsedDocument) -> Result<(), String> {
    let mut visitor = ImportTargetValidator { document };
    visitor.root(&document.root)
}

struct ImportTargetValidator<'a> {
    document: &'a ParsedDocument,
}

impl ImportTargetValidator<'_> {
    fn target(&mut self, target: &ImportTarget) -> Result<(), String> {
        target
            .validate_provenance(&self.document.source, &self.document.qualified_references)
            .map_err(|error| error.to_string())
    }

    fn root(&mut self, root: &RootNamespace) -> Result<(), String> {
        for element in &root.elements {
            match &element.value {
                RootElement::Package(node) => self.package_body(&node.value.body)?,
                RootElement::LibraryPackage(node) => self.package_body(&node.value.body)?,
                RootElement::Namespace(node) => self.package_body(&node.value.body)?,
                RootElement::Import(node) => self.target(&node.value.target)?,
                RootElement::Member(node) => self.package_element(&node.value)?,
            }
        }
        Ok(())
    }

    fn package_body(&mut self, body: &PackageBody) -> Result<(), String> {
        match body {
            PackageBody::Semicolon => Ok(()),
            PackageBody::Brace { elements } => {
                for element in elements {
                    self.package_element(&element.value)?;
                }
                Ok(())
            }
        }
    }

    fn package_element(&mut self, element: &PackageBodyElement) -> Result<(), String> {
        match element {
            PackageBodyElement::Package(n) => self.package_body(&n.value.body),
            PackageBodyElement::LibraryPackage(n) => self.package_body(&n.value.body),
            PackageBodyElement::Import(n) => self.target(&n.value.target),
            PackageBodyElement::PartDef(n) => self.part_def_body(&n.value.body),
            PackageBodyElement::PartUsage(n) => self.part_usage_body(&n.value.body),
            PackageBodyElement::ActionDef(n) => self.action_def_body(&n.value.body),
            PackageBodyElement::ActionUsage(n) => self.action_usage_body(&n.value.body),
            PackageBodyElement::RequirementDef(n) => self.requirement_body(&n.value.body),
            PackageBodyElement::RequirementUsage(n) => self.requirement_body(&n.value.body),
            PackageBodyElement::ConcernUsage(n) => self.requirement_body(&n.value.body),
            PackageBodyElement::ViewpointDef(n) => self.requirement_body(&n.value.body),
            PackageBodyElement::ViewpointUsage(n) => self.requirement_body(&n.value.body),
            PackageBodyElement::UseCaseDef(n) => self.use_case_body(&n.value.body),
            PackageBodyElement::UseCaseUsage(n) => self.use_case_body(&n.value.body),
            PackageBodyElement::CaseDef(n) => self.use_case_body(&n.value.body),
            PackageBodyElement::CaseUsage(n) => self.use_case_body(&n.value.body),
            PackageBodyElement::AnalysisCaseDef(n) => self.use_case_body(&n.value.body),
            PackageBodyElement::AnalysisCaseUsage(n) => self.use_case_body(&n.value.body),
            PackageBodyElement::VerificationCaseDef(n) => self.use_case_body(&n.value.body),
            PackageBodyElement::VerificationCaseUsage(n) => self.use_case_body(&n.value.body),
            PackageBodyElement::ViewUsage(n) => self.view_body(&n.value.body),
            PackageBodyElement::Error(_)
            | PackageBodyElement::Doc(_)
            | PackageBodyElement::Comment(_)
            | PackageBodyElement::TextualRep(_)
            | PackageBodyElement::Filter(_)
            | PackageBodyElement::PortDef(_)
            | PackageBodyElement::InterfaceDef(_)
            | PackageBodyElement::AliasDef(_)
            | PackageBodyElement::AttributeDef(_)
            | PackageBodyElement::Satisfy(_)
            | PackageBodyElement::Actor(_)
            | PackageBodyElement::StateDef(_)
            | PackageBodyElement::StateUsage(_)
            | PackageBodyElement::ItemDef(_)
            | PackageBodyElement::IndividualDef(_)
            | PackageBodyElement::ConstraintDef(_)
            | PackageBodyElement::ConstraintUsage(_)
            | PackageBodyElement::CalcDef(_)
            | PackageBodyElement::ViewDef(_)
            | PackageBodyElement::RenderingDef(_)
            | PackageBodyElement::RenderingUsage(_)
            | PackageBodyElement::ConnectionDef(_)
            | PackageBodyElement::MetadataDef(_)
            | PackageBodyElement::MetadataUsage(_)
            | PackageBodyElement::EnumDef(_)
            | PackageBodyElement::OccurrenceDef(_)
            | PackageBodyElement::OccurrenceUsage(_)
            | PackageBodyElement::Dependency(_)
            | PackageBodyElement::AllocationDef(_)
            | PackageBodyElement::AllocationUsage(_)
            | PackageBodyElement::FlowDef(_)
            | PackageBodyElement::FlowUsage(_)
            | PackageBodyElement::FeatureDecl(_)
            | PackageBodyElement::ClassifierDecl(_)
            | PackageBodyElement::KermlSemanticDecl(_)
            | PackageBodyElement::KermlFeatureDecl(_)
            | PackageBodyElement::ExtendedLibraryDecl(_)
            | PackageBodyElement::AttributeUsage(_)
            | PackageBodyElement::ItemUsage(_)
            | PackageBodyElement::PortUsage(_)
            | PackageBodyElement::ConnectionUsage(_)
            | PackageBodyElement::InterfaceUsage(_)
            | PackageBodyElement::Ref(_)
            | PackageBodyElement::EnumerationUsage(_)
            | PackageBodyElement::MetadataKeywordUsage(_)
            | PackageBodyElement::Connect(_)
            | PackageBodyElement::DefaultReferenceUsage(_)
            | PackageBodyElement::AssertConstraint(_) => Ok(()),
        }
    }

    fn requirement_body(&mut self, body: &RequirementDefBody) -> Result<(), String> {
        let RequirementDefBody::Brace { elements } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                RequirementDefBodyElement::Import(n) => self.target(&n.value.target)?,
                RequirementDefBodyElement::RequirementUsage(n) => {
                    self.requirement_body(&n.value.body)?
                }
                RequirementDefBodyElement::Frame(n) => self.requirement_body(&n.value.body)?,
                RequirementDefBodyElement::Error(_)
                | RequirementDefBodyElement::Other(_)
                | RequirementDefBodyElement::Annotation(_)
                | RequirementDefBodyElement::MetadataAnnotation(_)
                | RequirementDefBodyElement::MetadataKeywordUsage(_)
                | RequirementDefBodyElement::SubjectDecl(_)
                | RequirementDefBodyElement::SubjectRef(_)
                | RequirementDefBodyElement::RequirementActorDecl(_)
                | RequirementDefBodyElement::Stakeholder(_)
                | RequirementDefBodyElement::Purpose(_)
                | RequirementDefBodyElement::AttributeDef(_)
                | RequirementDefBodyElement::AttributeUsage(_)
                | RequirementDefBodyElement::VariantUsage(_)
                | RequirementDefBodyElement::VerifyRequirement(_)
                | RequirementDefBodyElement::RequireConstraint(_)
                | RequirementDefBodyElement::Constraint(_)
                | RequirementDefBodyElement::TextualRep(_)
                | RequirementDefBodyElement::Doc(_) => {}
            }
        }
        Ok(())
    }

    fn view_body(&mut self, body: &ViewBody) -> Result<(), String> {
        let ViewBody::Brace { elements } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                ViewBodyElement::Expose(n) => self.target(&n.value.target)?,
                ViewBodyElement::Error(_)
                | ViewBodyElement::Other(_)
                | ViewBodyElement::Doc(_)
                | ViewBodyElement::Filter(_)
                | ViewBodyElement::ViewRendering(_)
                | ViewBodyElement::Satisfy(_) => {}
            }
        }
        Ok(())
    }

    fn use_case_body(&mut self, body: &UseCaseDefBody) -> Result<(), String> {
        let UseCaseDefBody::Brace { elements } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                UseCaseDefBodyElement::ActionUsage(n) => self.action_usage_body(&n.value.body)?,
                UseCaseDefBodyElement::AnalysisCaseUsage(n) => self.use_case_body(&n.value.body)?,
                UseCaseDefBodyElement::RequirementUsage(n) => {
                    self.requirement_body(&n.value.body)?
                }
                UseCaseDefBodyElement::PartUsage(n) => self.part_usage_body(&n.value.body)?,
                UseCaseDefBodyElement::Error(_)
                | UseCaseDefBodyElement::Other(_)
                | UseCaseDefBodyElement::Annotation(_)
                | UseCaseDefBodyElement::MetadataAnnotation(_)
                | UseCaseDefBodyElement::MetadataKeywordUsage(_)
                | UseCaseDefBodyElement::AttributeDef(_)
                | UseCaseDefBodyElement::Doc(_)
                | UseCaseDefBodyElement::SubjectDecl(_)
                | UseCaseDefBodyElement::SubjectRef(_)
                | UseCaseDefBodyElement::ActorUsage(_)
                | UseCaseDefBodyElement::ActorRedefinitionAssignment(_)
                | UseCaseDefBodyElement::Objective(_)
                | UseCaseDefBodyElement::FirstSuccession(_)
                | UseCaseDefBodyElement::ThenIncludeUseCase(_)
                | UseCaseDefBodyElement::ThenUseCaseUsage(_)
                | UseCaseDefBodyElement::ThenDone(_)
                | UseCaseDefBodyElement::IncludeUseCase(_)
                | UseCaseDefBodyElement::RefRedefinition(_)
                | UseCaseDefBodyElement::ReturnRef(_)
                | UseCaseDefBodyElement::CaseReturnDecl(_)
                | UseCaseDefBodyElement::Assign(_)
                | UseCaseDefBodyElement::ForLoop(_)
                | UseCaseDefBodyElement::ThenAction(_)
                | UseCaseDefBodyElement::CalcUsage(_)
                | UseCaseDefBodyElement::AttributeUsage(_)
                | UseCaseDefBodyElement::Expression(_)
                | UseCaseDefBodyElement::FlowUsage(_) => {}
            }
        }
        Ok(())
    }

    fn action_def_body(&mut self, body: &ActionDefBody) -> Result<(), String> {
        let ActionDefBody::Brace { elements } = body else {
            return Ok(());
        };
        for element in elements {
            self.action_def_element(&element.value)?;
        }
        Ok(())
    }

    fn action_def_element(&mut self, element: &ActionDefBodyElement) -> Result<(), String> {
        match element {
            ActionDefBodyElement::PartUsage(n) => self.part_usage_body(&n.value.body),
            ActionDefBodyElement::ActionUsage(n) => self.action_usage_body(&n.value.body),
            ActionDefBodyElement::ForLoop(n) => self.action_def_body(&n.value.body),
            ActionDefBodyElement::WhileStmt(n) => self.action_def_body(&n.value.body),
            ActionDefBodyElement::LoopStmt(n) => self.action_def_body(&n.value.body),
            ActionDefBodyElement::IfStmt(n) => {
                self.action_def_body(&n.value.then_body)?;
                if let Some(body) = &n.value.else_body {
                    self.action_def_body(body)?;
                }
                Ok(())
            }
            ActionDefBodyElement::Error(_)
            | ActionDefBodyElement::InOutDecl(_)
            | ActionDefBodyElement::Doc(_)
            | ActionDefBodyElement::Annotation(_)
            | ActionDefBodyElement::MetadataAnnotation(_)
            | ActionDefBodyElement::MetadataKeywordUsage(_)
            | ActionDefBodyElement::MetadataUsage(_)
            | ActionDefBodyElement::TextualRep(_)
            | ActionDefBodyElement::RefDecl(_)
            | ActionDefBodyElement::Perform(_)
            | ActionDefBodyElement::Bind(_)
            | ActionDefBodyElement::FlowUsage(_)
            | ActionDefBodyElement::FirstStmt(_)
            | ActionDefBodyElement::MergeStmt(_)
            | ActionDefBodyElement::DecisionStmt(_)
            | ActionDefBodyElement::JoinStmt(_)
            | ActionDefBodyElement::ForkStmt(_)
            | ActionDefBodyElement::TerminateStmt(_)
            | ActionDefBodyElement::StateUsage(_)
            | ActionDefBodyElement::ItemUsage(_)
            | ActionDefBodyElement::AssertConstraint(_)
            | ActionDefBodyElement::OccurrenceUsage(_)
            | ActionDefBodyElement::Assign(_)
            | ActionDefBodyElement::ThenAction(_)
            | ActionDefBodyElement::Decl(_)
            | ActionDefBodyElement::DefaultReferenceUsage(_) => Ok(()),
        }
    }

    fn action_usage_body(&mut self, body: &ActionUsageBody) -> Result<(), String> {
        let ActionUsageBody::Brace { elements } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                ActionUsageBodyElement::PartUsage(n) => self.part_usage_body(&n.value.body)?,
                ActionUsageBodyElement::ActionUsage(n) => self.action_usage_body(&n.value.body)?,
                ActionUsageBodyElement::ForLoop(n) => self.action_def_body(&n.value.body)?,
                ActionUsageBodyElement::WhileStmt(n) => self.action_def_body(&n.value.body)?,
                ActionUsageBodyElement::LoopStmt(n) => self.action_def_body(&n.value.body)?,
                ActionUsageBodyElement::IfStmt(n) => {
                    self.action_def_body(&n.value.then_body)?;
                    if let Some(body) = &n.value.else_body {
                        self.action_def_body(body)?;
                    }
                }
                ActionUsageBodyElement::Error(_)
                | ActionUsageBodyElement::Doc(_)
                | ActionUsageBodyElement::Annotation(_)
                | ActionUsageBodyElement::MetadataAnnotation(_)
                | ActionUsageBodyElement::MetadataKeywordUsage(_)
                | ActionUsageBodyElement::MetadataUsage(_)
                | ActionUsageBodyElement::TextualRep(_)
                | ActionUsageBodyElement::InOutDecl(_)
                | ActionUsageBodyElement::RefDecl(_)
                | ActionUsageBodyElement::Bind(_)
                | ActionUsageBodyElement::FlowUsage(_)
                | ActionUsageBodyElement::FirstStmt(_)
                | ActionUsageBodyElement::MergeStmt(_)
                | ActionUsageBodyElement::DecisionStmt(_)
                | ActionUsageBodyElement::JoinStmt(_)
                | ActionUsageBodyElement::ForkStmt(_)
                | ActionUsageBodyElement::TerminateStmt(_)
                | ActionUsageBodyElement::StateUsage(_)
                | ActionUsageBodyElement::ItemUsage(_)
                | ActionUsageBodyElement::AssertConstraint(_)
                | ActionUsageBodyElement::OccurrenceUsage(_)
                | ActionUsageBodyElement::Assign(_)
                | ActionUsageBodyElement::ThenAction(_)
                | ActionUsageBodyElement::Decl(_)
                | ActionUsageBodyElement::DefaultReferenceUsage(_)
                | ActionUsageBodyElement::VariantUsage(_) => {}
            }
        }
        Ok(())
    }

    fn part_def_body(&mut self, body: &PartDefBody) -> Result<(), String> {
        let PartDefBody::Brace { elements } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                PartDefBodyElement::Import(n) => self.target(&n.value.target)?,
                PartDefBodyElement::PartDef(n) => self.part_def_body(&n.value.body)?,
                PartDefBodyElement::PartUsage(n) => self.part_usage_body(&n.value.body)?,
                PartDefBodyElement::ActionDef(n) => self.action_def_body(&n.value.body)?,
                PartDefBodyElement::ActionUsage(n) => self.action_usage_body(&n.value.body)?,
                PartDefBodyElement::RequirementDef(n) => self.requirement_body(&n.value.body)?,
                PartDefBodyElement::RequirementUsage(n) => self.requirement_body(&n.value.body)?,
                PartDefBodyElement::ViewpointDef(n) => self.requirement_body(&n.value.body)?,
                PartDefBodyElement::ViewpointUsage(n) => self.requirement_body(&n.value.body)?,
                PartDefBodyElement::ViewUsage(n) => self.view_body(&n.value.body)?,
                PartDefBodyElement::CaseDef(n) => self.use_case_body(&n.value.body)?,
                PartDefBodyElement::CaseUsage(n) => self.use_case_body(&n.value.body)?,
                PartDefBodyElement::UseCaseDef(n) => self.use_case_body(&n.value.body)?,
                PartDefBodyElement::UseCaseUsage(n) => self.use_case_body(&n.value.body)?,
                PartDefBodyElement::AnalysisCaseDef(n) => self.use_case_body(&n.value.body)?,
                PartDefBodyElement::AnalysisCaseUsage(n) => self.use_case_body(&n.value.body)?,
                PartDefBodyElement::VerificationCaseDef(n) => self.use_case_body(&n.value.body)?,
                PartDefBodyElement::VerificationCaseUsage(n) => {
                    self.use_case_body(&n.value.body)?
                }
                PartDefBodyElement::Error(_)
                | PartDefBodyElement::Doc(_)
                | PartDefBodyElement::Comment(_)
                | PartDefBodyElement::Annotation(_)
                | PartDefBodyElement::MetadataAnnotation(_)
                | PartDefBodyElement::MetadataKeywordUsage(_)
                | PartDefBodyElement::Dependency(_)
                | PartDefBodyElement::Other(_)
                | PartDefBodyElement::AttributeDef(_)
                | PartDefBodyElement::AttributeUsage(_)
                | PartDefBodyElement::DefaultReferenceUsage(_)
                | PartDefBodyElement::ItemDef(_)
                | PartDefBodyElement::ItemUsage(_)
                | PartDefBodyElement::Ref(_)
                | PartDefBodyElement::PortUsage(_)
                | PartDefBodyElement::OccurrenceUsage(_)
                | PartDefBodyElement::InterfaceDef(_)
                | PartDefBodyElement::InterfaceUsage(_)
                | PartDefBodyElement::Connect(_)
                | PartDefBodyElement::FlowUsage(_)
                | PartDefBodyElement::Connection(_)
                | PartDefBodyElement::Perform(_)
                | PartDefBodyElement::Allocate(_)
                | PartDefBodyElement::OpaqueMember(_)
                | PartDefBodyElement::ExhibitState(_)
                | PartDefBodyElement::CalcUsage(_)
                | PartDefBodyElement::ConstraintDef(_)
                | PartDefBodyElement::ConstraintUsage(_)
                | PartDefBodyElement::StateUsage(_)
                | PartDefBodyElement::EnumerationUsage(_)
                | PartDefBodyElement::AssertConstraint(_)
                | PartDefBodyElement::Satisfy(_)
                | PartDefBodyElement::VariantUsage(_)
                | PartDefBodyElement::StateDef(_)
                | PartDefBodyElement::MetadataDef(_)
                | PartDefBodyElement::MetadataUsage(_)
                | PartDefBodyElement::FlowDef(_)
                | PartDefBodyElement::OccurrenceDef(_)
                | PartDefBodyElement::ConnectionDef(_)
                | PartDefBodyElement::PortDef(_)
                | PartDefBodyElement::CalcDef(_)
                | PartDefBodyElement::EnumDef(_)
                | PartDefBodyElement::AllocationDef(_)
                | PartDefBodyElement::AllocationUsage(_)
                | PartDefBodyElement::ViewDef(_)
                | PartDefBodyElement::RenderingDef(_)
                | PartDefBodyElement::RenderingUsage(_)
                | PartDefBodyElement::FirstStmt(_)
                | PartDefBodyElement::Bind(_)
                | PartDefBodyElement::AliasDef(_) => {}
            }
        }
        Ok(())
    }

    fn part_usage_body(&mut self, body: &PartUsageBody) -> Result<(), String> {
        let PartUsageBody::Brace { elements } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                PartUsageBodyElement::Import(n) => self.target(&n.value.target)?,
                PartUsageBodyElement::PartUsage(n) => self.part_usage_body(&n.value.body)?,
                PartUsageBodyElement::ActionUsage(n) => self.action_usage_body(&n.value.body)?,
                PartUsageBodyElement::RequirementDef(n) => self.requirement_body(&n.value.body)?,
                PartUsageBodyElement::RequirementUsage(n) => {
                    self.requirement_body(&n.value.body)?
                }
                PartUsageBodyElement::UseCaseUsage(n) => self.use_case_body(&n.value.body)?,
                PartUsageBodyElement::AnalysisCaseDef(n) => self.use_case_body(&n.value.body)?,
                PartUsageBodyElement::AnalysisCaseUsage(n) => self.use_case_body(&n.value.body)?,
                PartUsageBodyElement::VerificationCaseUsage(n) => {
                    self.use_case_body(&n.value.body)?
                }
                PartUsageBodyElement::Error(_)
                | PartUsageBodyElement::Doc(_)
                | PartUsageBodyElement::Annotation(_)
                | PartUsageBodyElement::AttributeUsage(_)
                | PartUsageBodyElement::DefaultReferenceUsage(_)
                | PartUsageBodyElement::EnumerationUsage(_)
                | PartUsageBodyElement::OccurrenceUsage(_)
                | PartUsageBodyElement::PortUsage(_)
                | PartUsageBodyElement::Bind(_)
                | PartUsageBodyElement::Ref(_)
                | PartUsageBodyElement::InterfaceUsage(_)
                | PartUsageBodyElement::Connect(_)
                | PartUsageBodyElement::FlowUsage(_)
                | PartUsageBodyElement::Perform(_)
                | PartUsageBodyElement::SuccessionUsage(_)
                | PartUsageBodyElement::Allocate(_)
                | PartUsageBodyElement::Satisfy(_)
                | PartUsageBodyElement::StateUsage(_)
                | PartUsageBodyElement::MetadataAnnotation(_)
                | PartUsageBodyElement::MetadataKeywordUsage(_)
                | PartUsageBodyElement::VariantUsage(_)
                | PartUsageBodyElement::StateDef(_)
                | PartUsageBodyElement::MetadataDef(_)
                | PartUsageBodyElement::FlowDef(_)
                | PartUsageBodyElement::OccurrenceDef(_)
                | PartUsageBodyElement::PortDef(_)
                | PartUsageBodyElement::CalcDef(_)
                | PartUsageBodyElement::ConnectionDef(_)
                | PartUsageBodyElement::EnumDef(_)
                | PartUsageBodyElement::Connection(_)
                | PartUsageBodyElement::AssertConstraint(_)
                | PartUsageBodyElement::ConstraintDef(_)
                | PartUsageBodyElement::ConstraintUsage(_)
                | PartUsageBodyElement::CalcUsage(_)
                | PartUsageBodyElement::ItemDef(_)
                | PartUsageBodyElement::ItemUsage(_)
                | PartUsageBodyElement::MetadataUsage(_)
                | PartUsageBodyElement::AliasDef(_)
                | PartUsageBodyElement::IncludeUseCase(_) => {}
            }
        }
        Ok(())
    }
}
