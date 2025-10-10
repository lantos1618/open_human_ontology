use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Chromosome {
    Chr1,
    Chr2,
    Chr3,
    Chr4,
    Chr5,
    Chr6,
    Chr7,
    Chr8,
    Chr9,
    Chr10,
    Chr11,
    Chr12,
    Chr13,
    Chr14,
    Chr15,
    Chr16,
    Chr17,
    Chr18,
    Chr19,
    Chr20,
    Chr21,
    Chr22,
    ChrX,
    ChrY,
    ChrMT,
}

impl Chromosome {
    pub fn size_mb(&self) -> f64 {
        match self {
            Chromosome::Chr1 => 249.0,
            Chromosome::Chr2 => 242.0,
            Chromosome::Chr3 => 198.0,
            Chromosome::Chr4 => 191.0,
            Chromosome::Chr5 => 181.0,
            Chromosome::Chr6 => 171.0,
            Chromosome::Chr7 => 159.0,
            Chromosome::Chr8 => 146.0,
            Chromosome::Chr9 => 138.0,
            Chromosome::Chr10 => 134.0,
            Chromosome::Chr11 => 135.0,
            Chromosome::Chr12 => 133.0,
            Chromosome::Chr13 => 115.0,
            Chromosome::Chr14 => 107.0,
            Chromosome::Chr15 => 102.0,
            Chromosome::Chr16 => 90.0,
            Chromosome::Chr17 => 83.0,
            Chromosome::Chr18 => 80.0,
            Chromosome::Chr19 => 59.0,
            Chromosome::Chr20 => 63.0,
            Chromosome::Chr21 => 48.0,
            Chromosome::Chr22 => 51.0,
            Chromosome::ChrX => 155.0,
            Chromosome::ChrY => 59.0,
            Chromosome::ChrMT => 0.016,
        }
    }

    pub fn gene_count(&self) -> u32 {
        match self {
            Chromosome::Chr1 => 2058,
            Chromosome::Chr2 => 1309,
            Chromosome::Chr3 => 1078,
            Chromosome::Chr4 => 752,
            Chromosome::Chr5 => 876,
            Chromosome::Chr6 => 1048,
            Chromosome::Chr7 => 989,
            Chromosome::Chr8 => 677,
            Chromosome::Chr9 => 786,
            Chromosome::Chr10 => 733,
            Chromosome::Chr11 => 1298,
            Chromosome::Chr12 => 1034,
            Chromosome::Chr13 => 327,
            Chromosome::Chr14 => 598,
            Chromosome::Chr15 => 602,
            Chromosome::Chr16 => 873,
            Chromosome::Chr17 => 1197,
            Chromosome::Chr18 => 270,
            Chromosome::Chr19 => 1472,
            Chromosome::Chr20 => 544,
            Chromosome::Chr21 => 234,
            Chromosome::Chr22 => 488,
            Chromosome::ChrX => 842,
            Chromosome::ChrY => 63,
            Chromosome::ChrMT => 37,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Nucleotide {
    A,
    T,
    G,
    C,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GenomicLocation {
    pub chromosome: Chromosome,
    pub position: u64,
    pub reference_allele: Nucleotide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Zygosity {
    Homozygous,
    Heterozygous,
    Hemizygous,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Variant {
    pub location: GenomicLocation,
    pub alternate_allele: Nucleotide,
    pub zygosity: Zygosity,
    pub rsid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gene {
    pub symbol: String,
    pub name: String,
    pub chromosome: Chromosome,
    pub start_position: u64,
    pub end_position: u64,
    pub strand: Strand,
    pub function: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Strand {
    Forward,
    Reverse,
}

impl Gene {
    pub fn length(&self) -> u64 {
        self.end_position - self.start_position
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genome {
    pub variants: Vec<Variant>,
    pub genes: HashMap<String, Gene>,
    pub total_variants: u64,
    pub sequencing_coverage: f64,
}

impl Genome {
    pub fn new() -> Self {
        Self {
            variants: Vec::new(),
            genes: HashMap::new(),
            total_variants: 0,
            sequencing_coverage: 0.0,
        }
    }

    pub fn add_variant(&mut self, variant: Variant) {
        self.variants.push(variant);
        self.total_variants += 1;
    }

    pub fn get_variants_on_chromosome(&self, chr: Chromosome) -> Vec<&Variant> {
        self.variants
            .iter()
            .filter(|v| v.location.chromosome == chr)
            .collect()
    }

    pub fn heterozygosity_rate(&self) -> f64 {
        if self.variants.is_empty() {
            return 0.0;
        }
        let het_count = self
            .variants
            .iter()
            .filter(|v| v.zygosity == Zygosity::Heterozygous)
            .count();
        het_count as f64 / self.variants.len() as f64
    }
}

impl Default for Genome {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chromosome_size() {
        assert_eq!(Chromosome::Chr1.size_mb(), 249.0);
        assert_eq!(Chromosome::ChrY.size_mb(), 59.0);
    }

    #[test]
    fn test_gene_length() {
        let gene = Gene {
            symbol: "BRCA1".to_string(),
            name: "Breast Cancer 1".to_string(),
            chromosome: Chromosome::Chr17,
            start_position: 43044295,
            end_position: 43125483,
            strand: Strand::Reverse,
            function: "DNA repair".to_string(),
        };
        assert_eq!(gene.length(), 81188);
    }

    #[test]
    fn test_genome_variants() {
        let mut genome = Genome::new();
        let variant = Variant {
            location: GenomicLocation {
                chromosome: Chromosome::Chr1,
                position: 12345,
                reference_allele: Nucleotide::A,
            },
            alternate_allele: Nucleotide::G,
            zygosity: Zygosity::Heterozygous,
            rsid: Some("rs12345".to_string()),
        };

        genome.add_variant(variant);
        assert_eq!(genome.total_variants, 1);
        assert_eq!(genome.get_variants_on_chromosome(Chromosome::Chr1).len(), 1);
    }
}
