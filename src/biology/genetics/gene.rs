use super::dna::DNASequence;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeneType {
    ProteinCoding,
    RibosomalRNA,
    TransferRNA,
    MicroRNA,
    LongNonCoding,
    Pseudogene,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exon {
    pub start: usize,
    pub end: usize,
    pub sequence: DNASequence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Intron {
    pub start: usize,
    pub end: usize,
    pub sequence: DNASequence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Promoter {
    pub position: usize,
    pub strength: u8,
    pub elements: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gene {
    pub name: String,
    pub symbol: String,
    pub gene_type: GeneType,
    pub chromosome: u8,
    pub start_position: usize,
    pub end_position: usize,
    pub strand: Strand,
    pub sequence: DNASequence,
    pub exons: Vec<Exon>,
    pub introns: Vec<Intron>,
    pub promoter: Option<Promoter>,
    pub regulatory_regions: Vec<RegulatoryRegion>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Strand {
    Forward,
    Reverse,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegulatoryRegion {
    pub region_type: RegulatoryType,
    pub start: usize,
    pub end: usize,
    pub binding_factors: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegulatoryType {
    Enhancer,
    Silencer,
    Insulator,
    LCR,
}

impl Gene {
    pub fn new(
        name: String,
        symbol: String,
        gene_type: GeneType,
        chromosome: u8,
        start_position: usize,
        end_position: usize,
        strand: Strand,
        sequence: DNASequence,
    ) -> Self {
        Gene {
            name,
            symbol,
            gene_type,
            chromosome,
            start_position,
            end_position,
            strand,
            sequence,
            exons: Vec::new(),
            introns: Vec::new(),
            promoter: None,
            regulatory_regions: Vec::new(),
        }
    }

    pub fn length(&self) -> usize {
        self.end_position - self.start_position
    }

    pub fn coding_sequence(&self) -> DNASequence {
        if self.exons.is_empty() {
            return self.sequence.clone();
        }

        let mut coding_seq = Vec::new();
        for exon in &self.exons {
            coding_seq.extend_from_slice(&exon.sequence.to_string().chars().collect::<Vec<_>>());
        }

        DNASequence::from_str(&coding_seq.iter().collect::<String>())
            .unwrap_or_else(|| self.sequence.clone())
    }

    pub fn exon_count(&self) -> usize {
        self.exons.len()
    }

    pub fn intron_count(&self) -> usize {
        self.introns.len()
    }

    pub fn add_exon(&mut self, exon: Exon) {
        self.exons.push(exon);
        self.exons.sort_by_key(|e| e.start);
    }

    pub fn add_intron(&mut self, intron: Intron) {
        self.introns.push(intron);
        self.introns.sort_by_key(|i| i.start);
    }

    pub fn is_protein_coding(&self) -> bool {
        matches!(self.gene_type, GeneType::ProteinCoding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gene_creation() {
        let seq = DNASequence::from_str("ATGCATGCATGC").unwrap();
        let gene = Gene::new(
            "Test Gene".to_string(),
            "TG1".to_string(),
            GeneType::ProteinCoding,
            1,
            1000,
            2000,
            Strand::Forward,
            seq,
        );

        assert_eq!(gene.length(), 1000);
        assert!(gene.is_protein_coding());
    }

    #[test]
    fn test_add_exon() {
        let seq = DNASequence::from_str("ATGCATGCATGC").unwrap();
        let mut gene = Gene::new(
            "Test Gene".to_string(),
            "TG1".to_string(),
            GeneType::ProteinCoding,
            1,
            1000,
            2000,
            Strand::Forward,
            seq.clone(),
        );

        let exon = Exon {
            start: 1000,
            end: 1100,
            sequence: seq,
        };

        gene.add_exon(exon);
        assert_eq!(gene.exon_count(), 1);
    }
}
