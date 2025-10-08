use serde::{Deserialize, Serialize};
use super::allele::AllelePair;
use super::snp::SNPGenotype;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Genotype {
    pub gene_alleles: HashMap<String, AllelePair>,
    pub snps: Vec<SNPGenotype>,
    pub ancestry: Vec<AncestryComponent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AncestryComponent {
    pub population: String,
    pub percentage: f64,
}

impl Genotype {
    pub fn new() -> Self {
        Genotype {
            gene_alleles: HashMap::new(),
            snps: Vec::new(),
            ancestry: Vec::new(),
        }
    }

    pub fn add_gene_alleles(&mut self, gene_symbol: String, allele_pair: AllelePair) {
        self.gene_alleles.insert(gene_symbol, allele_pair);
    }

    pub fn add_snp(&mut self, snp_genotype: SNPGenotype) {
        self.snps.push(snp_genotype);
    }

    pub fn add_ancestry(&mut self, population: String, percentage: f64) {
        self.ancestry.push(AncestryComponent {
            population,
            percentage: percentage.clamp(0.0, 100.0),
        });
    }

    pub fn get_gene_alleles(&self, gene_symbol: &str) -> Option<&AllelePair> {
        self.gene_alleles.get(gene_symbol)
    }

    pub fn has_pathogenic_variant(&self, gene_symbol: &str) -> bool {
        self.gene_alleles
            .get(gene_symbol)
            .map(|pair| pair.has_pathogenic_allele())
            .unwrap_or(false)
    }

    pub fn pathogenic_genes(&self) -> Vec<String> {
        self.gene_alleles
            .iter()
            .filter(|(_, pair)| pair.has_pathogenic_allele())
            .map(|(gene, _)| gene.clone())
            .collect()
    }

    pub fn primary_ancestry(&self) -> Option<&AncestryComponent> {
        self.ancestry
            .iter()
            .max_by(|a, b| a.percentage.partial_cmp(&b.percentage).unwrap())
    }

    pub fn is_admixed(&self) -> bool {
        self.ancestry.len() > 1 &&
        self.ancestry.iter().all(|a| a.percentage < 80.0)
    }
}

impl Default for Genotype {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhenotypeAssociation {
    pub genotype_marker: String,
    pub phenotype: String,
    pub penetrance: f64,
    pub age_of_onset: Option<f64>,
    pub severity: Severity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Mild,
    Moderate,
    Severe,
    LifeThreatening,
}

impl PhenotypeAssociation {
    pub fn new(
        genotype_marker: String,
        phenotype: String,
        penetrance: f64,
    ) -> Self {
        PhenotypeAssociation {
            genotype_marker,
            phenotype,
            penetrance: penetrance.clamp(0.0, 1.0),
            age_of_onset: None,
            severity: Severity::Moderate,
        }
    }

    pub fn with_age_of_onset(mut self, age: f64) -> Self {
        self.age_of_onset = Some(age);
        self
    }

    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    pub fn is_highly_penetrant(&self) -> bool {
        self.penetrance > 0.8
    }

    pub fn is_early_onset(&self) -> bool {
        self.age_of_onset.map(|age| age < 40.0).unwrap_or(false)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenotypeRiskProfile {
    pub genotype: Genotype,
    pub disease_risks: Vec<DiseaseRisk>,
    pub pharmacogenetic_markers: Vec<PharmacogeneticMarker>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiseaseRisk {
    pub disease: String,
    pub baseline_risk: f64,
    pub genetic_risk: f64,
    pub lifetime_risk: f64,
    pub contributing_variants: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PharmacogeneticMarker {
    pub gene: String,
    pub drug: String,
    pub metabolizer_status: MetabolizerStatus,
    pub recommendation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetabolizerStatus {
    UltraRapid,
    Rapid,
    Normal,
    Intermediate,
    Poor,
}

impl GenotypeRiskProfile {
    pub fn new(genotype: Genotype) -> Self {
        GenotypeRiskProfile {
            genotype,
            disease_risks: Vec::new(),
            pharmacogenetic_markers: Vec::new(),
        }
    }

    pub fn add_disease_risk(&mut self, risk: DiseaseRisk) {
        self.disease_risks.push(risk);
    }

    pub fn add_pharmacogenetic_marker(&mut self, marker: PharmacogeneticMarker) {
        self.pharmacogenetic_markers.push(marker);
    }

    pub fn high_risk_diseases(&self) -> Vec<&DiseaseRisk> {
        self.disease_risks
            .iter()
            .filter(|r| r.genetic_risk > r.baseline_risk * 2.0)
            .collect()
    }

    pub fn drug_interactions(&self, drug: &str) -> Vec<&PharmacogeneticMarker> {
        self.pharmacogenetic_markers
            .iter()
            .filter(|m| m.drug == drug)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::dna::DNASequence;
    use super::super::allele::{Allele, AlleleType};

    #[test]
    fn test_genotype_creation() {
        let mut genotype = Genotype::new();

        let seq1 = DNASequence::from_str("ATGC").unwrap();
        let seq2 = DNASequence::from_str("ATGC").unwrap();

        let allele1 = Allele::new(
            "BRCA1".to_string(),
            "wildtype".to_string(),
            seq1,
            AlleleType::WildType,
        );

        let allele2 = Allele::new(
            "BRCA1".to_string(),
            "c.185delAG".to_string(),
            seq2,
            AlleleType::Frameshift,
        );

        let pair = AllelePair::new(allele1, allele2);
        genotype.add_gene_alleles("BRCA1".to_string(), pair);

        assert!(genotype.has_pathogenic_variant("BRCA1"));
        assert_eq!(genotype.pathogenic_genes(), vec!["BRCA1"]);
    }

    #[test]
    fn test_ancestry() {
        let mut genotype = Genotype::new();
        genotype.add_ancestry("European".to_string(), 60.0);
        genotype.add_ancestry("East Asian".to_string(), 40.0);

        assert!(genotype.is_admixed());
        assert_eq!(genotype.primary_ancestry().unwrap().population, "European");
    }

    #[test]
    fn test_phenotype_association() {
        let assoc = PhenotypeAssociation::new(
            "BRCA1_c.185delAG".to_string(),
            "Breast Cancer".to_string(),
            0.85,
        )
        .with_age_of_onset(45.0)
        .with_severity(Severity::Severe);

        assert!(assoc.is_highly_penetrant());
        assert!(!assoc.is_early_onset());
    }

    #[test]
    fn test_risk_profile() {
        let genotype = Genotype::new();
        let mut profile = GenotypeRiskProfile::new(genotype);

        let risk = DiseaseRisk {
            disease: "Type 2 Diabetes".to_string(),
            baseline_risk: 0.10,
            genetic_risk: 0.25,
            lifetime_risk: 0.30,
            contributing_variants: vec!["rs7903146".to_string()],
        };

        profile.add_disease_risk(risk);

        let high_risks = profile.high_risk_diseases();
        assert_eq!(high_risks.len(), 1);
    }
}
