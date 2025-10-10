use super::genome::Chromosome;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VariantType {
    SNP,
    Insertion,
    Deletion,
    Duplication,
    Inversion,
    CopyNumberVariation,
    StructuralVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VariantEffect {
    Benign,
    LikelyBenign,
    Uncertain,
    LikelyPathogenic,
    Pathogenic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VariantConsequence {
    Synonymous,
    Missense,
    Nonsense,
    Frameshift,
    SpliceSite,
    Regulatory,
    Intronic,
    Intergenic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticVariation {
    pub chromosome: Chromosome,
    pub position: u64,
    pub reference: String,
    pub alternate: String,
    pub variant_type: VariantType,
    pub consequence: VariantConsequence,
    pub effect: VariantEffect,
    pub allele_frequency: f64,
    pub rsid: Option<String>,
    pub gene_symbol: Option<String>,
}

impl GeneticVariation {
    pub fn is_rare(&self) -> bool {
        self.allele_frequency < 0.01
    }

    pub fn is_common(&self) -> bool {
        self.allele_frequency > 0.05
    }

    pub fn is_clinically_significant(&self) -> bool {
        matches!(
            self.effect,
            VariantEffect::Pathogenic | VariantEffect::LikelyPathogenic
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyNumberVariant {
    pub chromosome: Chromosome,
    pub start_position: u64,
    pub end_position: u64,
    pub copy_number: u32,
    pub length_bp: u64,
}

impl CopyNumberVariant {
    pub fn is_deletion(&self) -> bool {
        self.copy_number < 2
    }

    pub fn is_duplication(&self) -> bool {
        self.copy_number > 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_classification() {
        let variant = GeneticVariation {
            chromosome: Chromosome::Chr17,
            position: 43044295,
            reference: "A".to_string(),
            alternate: "G".to_string(),
            variant_type: VariantType::SNP,
            consequence: VariantConsequence::Missense,
            effect: VariantEffect::Pathogenic,
            allele_frequency: 0.001,
            rsid: Some("rs80357906".to_string()),
            gene_symbol: Some("BRCA1".to_string()),
        };

        assert!(variant.is_rare());
        assert!(!variant.is_common());
        assert!(variant.is_clinically_significant());
    }

    #[test]
    fn test_cnv() {
        let deletion = CopyNumberVariant {
            chromosome: Chromosome::Chr22,
            start_position: 18900000,
            end_position: 21800000,
            copy_number: 1,
            length_bp: 2900000,
        };

        assert!(deletion.is_deletion());
        assert!(!deletion.is_duplication());
    }
}
