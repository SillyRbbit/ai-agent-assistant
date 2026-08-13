use super::*;

impl InfrastructureOperationsFixtureCatalog {
    #[must_use]
    pub const fn built_in() -> Self {
        Self
    }

    pub fn cloud_request(
        self,
        scenario_id: CloudScenarioId,
    ) -> InfrastructureOperationsResult<CloudInfrastructureWorkflowRequest> {
        let data = match scenario_id {
            CloudScenarioId::TerraformDecisionBriefV1 => cloud_fixture_request_data()?,
        };
        Ok(CloudInfrastructureWorkflowRequest {
            scenario_id,
            data,
            proof: CatalogProof {
                version: BUILT_IN_CATALOG_VERSION,
            },
        })
    }

    pub fn systems_request(
        self,
        scenario_id: SystemsOperationsScenarioId,
    ) -> InfrastructureOperationsResult<SystemsOperationsWorkflowRequest> {
        let data = match scenario_id {
            SystemsOperationsScenarioId::SanitizedServiceRecoveryV1 => {
                systems_fixture_request_data()?
            }
        };
        Ok(SystemsOperationsWorkflowRequest {
            scenario_id,
            data,
            proof: CatalogProof {
                version: BUILT_IN_CATALOG_VERSION,
            },
        })
    }
}

impl CloudInfrastructureWorkflowRequest {
    pub(super) fn validate_catalog_binding(&self) -> InfrastructureOperationsResult<()> {
        let canonical =
            InfrastructureOperationsFixtureCatalog::built_in().cloud_request(self.scenario_id)?;
        if self == &canonical && self.proof.version == BUILT_IN_CATALOG_VERSION {
            Ok(())
        } else {
            Err(InfrastructureOperationsError::CatalogBindingMismatch)
        }
    }
}

impl SystemsOperationsWorkflowRequest {
    pub(super) fn validate_catalog_binding(&self) -> InfrastructureOperationsResult<()> {
        let canonical =
            InfrastructureOperationsFixtureCatalog::built_in().systems_request(self.scenario_id)?;
        if self == &canonical && self.proof.version == BUILT_IN_CATALOG_VERSION {
            Ok(())
        } else {
            Err(InfrastructureOperationsError::CatalogBindingMismatch)
        }
    }
}

pub(super) fn cloud_fixture_request_data() -> InfrastructureOperationsResult<WorkflowRequestData> {
    let fixtures = vec![
        InfrastructureOperationsFixture::new(
            "terraform-configuration",
            InfrastructureOperationsFixtureKind::Cloud(CloudFixtureKind::TerraformConfiguration),
            "Synthetic Terraform configuration",
            "resource \"fixture_compute\" \"example\" {\n  public_ingress = true\n}",
        )?,
        InfrastructureOperationsFixture::new(
            "azure-architecture",
            InfrastructureOperationsFixtureKind::Cloud(CloudFixtureKind::AzureArchitecture),
            "Synthetic Azure architecture note",
            "The synthetic design places an internet-facing endpoint before one workload tier.",
        )?,
    ];
    let criteria = vec![
        InfrastructureOperationsCriterion::new(
            "criterion-static-review",
            "Identify fixture-supported infrastructure risks and limitations.",
        )?,
        InfrastructureOperationsCriterion::new(
            "criterion-no-execution",
            "State that Terraform, provider inventory, and external checks were not run.",
        )?,
    ];
    let evidence = vec![
        InfrastructureOperationsValidationEvidence::new(
            "evidence-static-observation",
            InfrastructureOperationsEvidenceStatus::ObservedFixture,
            "The application supplied the synthetic configuration and architecture note.",
            vec![InfrastructureOperationsId::new("criterion-static-review")?],
            vec![
                InfrastructureOperationsId::new("terraform-configuration")?,
                InfrastructureOperationsId::new("azure-architecture")?,
            ],
        )?,
        InfrastructureOperationsValidationEvidence::new(
            "evidence-terraform-not-run",
            InfrastructureOperationsEvidenceStatus::NotRun,
            "Terraform and provider checks were not run.",
            vec![InfrastructureOperationsId::new("criterion-no-execution")?],
            Vec::new(),
        )?,
    ];
    WorkflowRequestData::new(
        "Review the supplied synthetic Terraform and architecture fixtures and produce an inert decision brief.",
        fixtures,
        criteria,
        evidence,
    )
}

fn systems_fixture_request_data() -> InfrastructureOperationsResult<WorkflowRequestData> {
    let fixtures = vec![
        InfrastructureOperationsFixture::new(
            "service-snapshot",
            InfrastructureOperationsFixtureKind::Systems(SystemsFixtureKind::ServiceSnapshot),
            "Synthetic service snapshot",
            "fixture-api.service is inactive after three fixture-only start attempts.",
        )?,
        InfrastructureOperationsFixture::new(
            "sanitized-log",
            InfrastructureOperationsFixtureKind::Systems(
                SystemsFixtureKind::SanitizedLogExcerpt,
            ),
            "Sanitized synthetic log excerpt",
            "The fixture log reports a dependency timeout and contains no host or account identity.",
        )?,
        InfrastructureOperationsFixture::new(
            "recovery-scenario",
            InfrastructureOperationsFixtureKind::Systems(SystemsFixtureKind::RecoveryScenario),
            "Synthetic recovery scenario",
            "Analyze recovery options without restarting a service or changing configuration.",
        )?,
    ];
    let criteria = vec![
        InfrastructureOperationsCriterion::new(
            "criterion-diagnostic",
            "Distinguish fixture-supported findings from hypotheses.",
        )?,
        InfrastructureOperationsCriterion::new(
            "criterion-safe-remediation",
            "Propose diagnostics and remediation without executing a command or effect.",
        )?,
    ];
    let evidence = vec![InfrastructureOperationsValidationEvidence::new(
        "evidence-service-log-observation",
        InfrastructureOperationsEvidenceStatus::ObservedFixture,
        "The application supplied the sanitized service and log fixtures.",
        vec![InfrastructureOperationsId::new("criterion-diagnostic")?],
        vec![
            InfrastructureOperationsId::new("service-snapshot")?,
            InfrastructureOperationsId::new("sanitized-log")?,
        ],
    )?];
    WorkflowRequestData::new(
        "Analyze the supplied sanitized service and log fixtures and propose inert diagnostics and recovery steps.",
        fixtures,
        criteria,
        evidence,
    )
}
