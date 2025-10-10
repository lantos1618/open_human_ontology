use super::dna::Nucleotide;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SNP {
    pub rs_id: String,
    pub chromosome: u8,
    pub position: usize,
    pub reference_allele: Nucleotide,
    pub alternate_alleles: Vec<Nucleotide>,
    pub minor_allele_frequency: f64,
}

impl SNP {
    pub fn new(
        rs_id: String,
        chromosome: u8,
        position: usize,
        reference_allele: Nucleotide,
        alternate_alleles: Vec<Nucleotide>,
    ) -> Self {
        SNP {
            rs_id,
            chromosome,
            position,
            reference_allele,
            alternate_alleles,
            minor_allele_frequency: 0.0,
        }
    }

    pub fn with_maf(mut self, maf: f64) -> Self {
        self.minor_allele_frequency = maf.clamp(0.0, 0.5);
        self
    }

    pub fn is_common(&self) -> bool {
        self.minor_allele_frequency >= 0.05
    }

    pub fn is_rare(&self) -> bool {
        self.minor_allele_frequency > 0.0 && self.minor_allele_frequency < 0.01
    }

    pub fn is_transition(&self) -> bool {
        for alt in &self.alternate_alleles {
            if self.is_transition_pair(&self.reference_allele, alt) {
                return true;
            }
        }
        false
    }

    pub fn is_transversion(&self) -> bool {
        !self.is_transition()
    }

    fn is_transition_pair(&self, n1: &Nucleotide, n2: &Nucleotide) -> bool {
        matches!(
            (n1, n2),
            (Nucleotide::Adenine, Nucleotide::Guanine)
                | (Nucleotide::Guanine, Nucleotide::Adenine)
                | (Nucleotide::Cytosine, Nucleotide::Thymine)
                | (Nucleotide::Thymine, Nucleotide::Cytosine)
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SNPAssociation {
    pub snp: SNP,
    pub trait_or_disease: String,
    pub odds_ratio: f64,
    pub p_value: f64,
    pub effect_size: f64,
    pub risk_allele: Nucleotide,
}

impl SNPAssociation {
    pub fn new(
        snp: SNP,
        trait_or_disease: String,
        odds_ratio: f64,
        p_value: f64,
        risk_allele: Nucleotide,
    ) -> Self {
        SNPAssociation {
            snp,
            trait_or_disease,
            odds_ratio,
            p_value,
            effect_size: (odds_ratio - 1.0).abs(),
            risk_allele,
        }
    }

    pub fn is_significant(&self) -> bool {
        self.p_value < 5e-8
    }

    pub fn is_protective(&self) -> bool {
        self.odds_ratio < 1.0
    }

    pub fn is_risk(&self) -> bool {
        self.odds_ratio > 1.0
    }

    pub fn relative_risk_percentage(&self) -> f64 {
        (self.odds_ratio - 1.0) * 100.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SNPGenotype {
    pub snp: SNP,
    pub allele1: Nucleotide,
    pub allele2: Nucleotide,
}

impl SNPGenotype {
    pub fn new(snp: SNP, allele1: Nucleotide, allele2: Nucleotide) -> Self {
        SNPGenotype {
            snp,
            allele1,
            allele2,
        }
    }

    pub fn is_homozygous(&self) -> bool {
        self.allele1 == self.allele2
    }

    pub fn is_heterozygous(&self) -> bool {
        self.allele1 != self.allele2
    }

    pub fn is_reference(&self) -> bool {
        self.allele1 == self.snp.reference_allele && self.allele2 == self.snp.reference_allele
    }

    pub fn has_alternate_allele(&self) -> bool {
        self.snp.alternate_alleles.contains(&self.allele1)
            || self.snp.alternate_alleles.contains(&self.allele2)
    }

    pub fn allele_count(&self, allele: &Nucleotide) -> u8 {
        let mut count = 0;
        if self.allele1 == *allele {
            count += 1;
        }
        if self.allele2 == *allele {
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snp_creation() {
        let snp = SNP::new(
            "rs123456".to_string(),
            1,
            1000000,
            Nucleotide::Adenine,
            vec![Nucleotide::Guanine],
        )
        .with_maf(0.25);

        assert_eq!(snp.rs_id, "rs123456");
        assert!(snp.is_common());
        assert!(!snp.is_rare());
    }

    #[test]
    fn test_transition() {
        let snp = SNP::new(
            "rs123456".to_string(),
            1,
            1000000,
            Nucleotide::Adenine,
            vec![Nucleotide::Guanine],
        );

        assert!(snp.is_transition());
        assert!(!snp.is_transversion());
    }

    #[test]
    fn test_snp_association() {
        let snp = SNP::new(
            "rs123456".to_string(),
            1,
            1000000,
            Nucleotide::Adenine,
            vec![Nucleotide::Guanine],
        );

        let assoc = SNPAssociation::new(
            snp,
            "Type 2 Diabetes".to_string(),
            1.5,
            1e-10,
            Nucleotide::Guanine,
        );

        assert!(assoc.is_significant());
        assert!(assoc.is_risk());
        assert!(!assoc.is_protective());
        assert_eq!(assoc.relative_risk_percentage(), 50.0);
    }

    #[test]
    fn test_snp_genotype() {
        let snp = SNP::new(
            "rs123456".to_string(),
            1,
            1000000,
            Nucleotide::Adenine,
            vec![Nucleotide::Guanine],
        );

        let genotype = SNPGenotype::new(snp.clone(), Nucleotide::Adenine, Nucleotide::Guanine);

        assert!(genotype.is_heterozygous());
        assert!(!genotype.is_homozygous());
        assert!(!genotype.is_reference());
        assert!(genotype.has_alternate_allele());
        assert_eq!(genotype.allele_count(&Nucleotide::Adenine), 1);
    }
}
