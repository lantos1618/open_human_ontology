use serde::{Deserialize, Serialize};
use super::dna::DNASequence;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Allele {
    pub gene_symbol: String,
    pub variant_name: String,
    pub sequence: DNASequence,
    pub allele_type: AlleleType,
    pub frequency: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlleleType {
    WildType,
    Missense,
    Nonsense,
    Frameshift,
    Silent,
    Splice,
    Insertion,
    Deletion,
    Duplication,
}

impl Allele {
    pub fn new(
        gene_symbol: String,
        variant_name: String,
        sequence: DNASequence,
        allele_type: AlleleType,
    ) -> Self {
        Allele {
            gene_symbol,
            variant_name,
            sequence,
            allele_type,
            frequency: 0.0,
        }
    }

    pub fn with_frequency(mut self, frequency: f64) -> Self {
        self.frequency = frequency.clamp(0.0, 1.0);
        self
    }

    pub fn is_pathogenic(&self) -> bool {
        matches!(
            self.allele_type,
            AlleleType::Nonsense | AlleleType::Frameshift
        )
    }

    pub fn is_benign(&self) -> bool {
        matches!(self.allele_type, AlleleType::Silent | AlleleType::WildType)
    }

    pub fn is_common(&self) -> bool {
        self.frequency >= 0.05
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllelePair {
    pub allele1: Allele,
    pub allele2: Allele,
    pub zygosity: Zygosity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Zygosity {
    Homozygous,
    Heterozygous,
    Hemizygous,
}

impl AllelePair {
    pub fn new(allele1: Allele, allele2: Allele) -> Self {
        let zygosity = if allele1 == allele2 {
            Zygosity::Homozygous
        } else {
            Zygosity::Heterozygous
        };

        AllelePair {
            allele1,
            allele2,
            zygosity,
        }
    }

    pub fn hemizygous(allele: Allele) -> Self {
        AllelePair {
            allele1: allele.clone(),
            allele2: allele,
            zygosity: Zygosity::Hemizygous,
        }
    }

    pub fn is_homozygous(&self) -> bool {
        matches!(self.zygosity, Zygosity::Homozygous)
    }

    pub fn is_heterozygous(&self) -> bool {
        matches!(self.zygosity, Zygosity::Heterozygous)
    }

    pub fn has_pathogenic_allele(&self) -> bool {
        self.allele1.is_pathogenic() || self.allele2.is_pathogenic()
    }

    pub fn carrier_status(&self) -> CarrierStatus {
        match (self.allele1.is_pathogenic(), self.allele2.is_pathogenic()) {
            (true, true) => CarrierStatus::Affected,
            (true, false) | (false, true) => CarrierStatus::Carrier,
            (false, false) => CarrierStatus::Unaffected,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CarrierStatus {
    Affected,
    Carrier,
    Unaffected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlleleFrequency {
    pub population: String,
    pub allele: Allele,
    pub frequency: f64,
    pub sample_size: usize,
}

impl AlleleFrequency {
    pub fn new(population: String, allele: Allele, frequency: f64, sample_size: usize) -> Self {
        AlleleFrequency {
            population,
            allele,
            frequency: frequency.clamp(0.0, 1.0),
            sample_size,
        }
    }

    pub fn is_reliable(&self) -> bool {
        self.sample_size >= 1000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allele_creation() {
        let seq = DNASequence::from_str("ATGC").unwrap();
        let allele = Allele::new(
            "BRCA1".to_string(),
            "c.185delAG".to_string(),
            seq,
            AlleleType::Frameshift,
        ).with_frequency(0.001);

        assert!(allele.is_pathogenic());
        assert!(!allele.is_benign());
        assert!(!allele.is_common());
    }

    #[test]
    fn test_allele_pair() {
        let seq1 = DNASequence::from_str("ATGC").unwrap();
        let seq2 = DNASequence::from_str("ATGC").unwrap();

        let allele1 = Allele::new(
            "BRCA1".to_string(),
            "c.185delAG".to_string(),
            seq1,
            AlleleType::Frameshift,
        );

        let allele2 = Allele::new(
            "BRCA1".to_string(),
            "wildtype".to_string(),
            seq2,
            AlleleType::WildType,
        );

        let pair = AllelePair::new(allele1, allele2);

        assert!(pair.is_heterozygous());
        assert!(pair.has_pathogenic_allele());
        assert_eq!(pair.carrier_status(), CarrierStatus::Carrier);
    }

    #[test]
    fn test_affected_status() {
        let seq = DNASequence::from_str("ATGC").unwrap();
        let allele = Allele::new(
            "CFTR".to_string(),
            "F508del".to_string(),
            seq,
            AlleleType::Deletion,
        );

        let pair = AllelePair::new(allele.clone(), allele);

        assert!(pair.is_homozygous());
        assert_eq!(pair.carrier_status(), CarrierStatus::Unaffected);
    }
}
