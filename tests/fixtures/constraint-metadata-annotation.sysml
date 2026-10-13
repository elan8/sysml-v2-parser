package ConstraintMetadataDemo {
	constraint def SafetyConstraint {
		@Approval : ApprovalKind;
		in mass : Real;
		mass >= 0
	}
}
