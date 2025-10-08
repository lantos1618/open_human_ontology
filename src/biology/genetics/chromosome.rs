use serde::{Deserialize, Serialize};
use super::dna::DNASequence;
use super::gene::Gene;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chromosome {
    pub number: u8,
    pub sequence: DNASequence,
    pub genes: Vec<Gene>,
    pub centromere_position: usize,
    pub telomere_length: usize,
    pub is_sex_chromosome: bool,
}

impl Chromosome {
    pub fn new(number: u8, sequence: DNASequence, centromere_position: usize) -> Self {
        Chromosome {
            number,
            sequence,
            genes: Vec::new(),
            centromere_position,
            telomere_length: 15000,
            is_sex_chromosome: number == 23 || number == 24,
        }
    }

    pub fn autosome(number: u8, sequence: DNASequence, centromere_position: usize) -> Self {
        assert!(number >= 1 && number <= 22);
        let mut chr = Self::new(number, sequence, centromere_position);
        chr.is_sex_chromosome = false;
        chr
    }

    pub fn x_chromosome(sequence: DNASequence, centromere_position: usize) -> Self {
        let mut chr = Self::new(23, sequence, centromere_position);
        chr.is_sex_chromosome = true;
        chr
    }

    pub fn y_chromosome(sequence: DNASequence, centromere_position: usize) -> Self {
        let mut chr = Self::new(24, sequence, centromere_position);
        chr.is_sex_chromosome = true;
        chr
    }

    pub fn length(&self) -> usize {
        self.sequence.len()
    }

    pub fn p_arm_length(&self) -> usize {
        self.centromere_position
    }

    pub fn q_arm_length(&self) -> usize {
        self.length() - self.centromere_position
    }

    pub fn add_gene(&mut self, gene: Gene) {
        self.genes.push(gene);
        self.genes.sort_by_key(|g| g.start_position);
    }

    pub fn gene_count(&self) -> usize {
        self.genes.len()
    }

    pub fn gene_density(&self) -> f64 {
        if self.length() == 0 {
            return 0.0;
        }
        self.genes.len() as f64 / self.length() as f64 * 1_000_000.0
    }

    pub fn find_gene_by_position(&self, position: usize) -> Option<&Gene> {
        self.genes.iter()
            .find(|g| position >= g.start_position && position <= g.end_position)
    }

    pub fn find_gene_by_symbol(&self, symbol: &str) -> Option<&Gene> {
        self.genes.iter()
            .find(|g| g.symbol == symbol)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Karyotype {
    pub autosomes: Vec<(Chromosome, Chromosome)>,
    pub sex_chromosomes: SexChromosomes,
    pub total_chromosome_count: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SexChromosomes {
    XX(Chromosome, Chromosome),
    XY(Chromosome, Chromosome),
    X(Chromosome),
    XXY(Chromosome, Chromosome, Chromosome),
    XYY(Chromosome, Chromosome, Chromosome),
    XXX(Chromosome, Chromosome, Chromosome),
}

impl Karyotype {
    pub fn normal_male() -> Self {
        Karyotype {
            autosomes: Vec::new(),
            sex_chromosomes: SexChromosomes::XY(
                Chromosome::x_chromosome(
                    DNASequence::from_str("ATGC").unwrap(),
                    1000,
                ),
                Chromosome::y_chromosome(
                    DNASequence::from_str("ATGC").unwrap(),
                    500,
                ),
            ),
            total_chromosome_count: 46,
        }
    }

    pub fn normal_female() -> Self {
        Karyotype {
            autosomes: Vec::new(),
            sex_chromosomes: SexChromosomes::XX(
                Chromosome::x_chromosome(
                    DNASequence::from_str("ATGC").unwrap(),
                    1000,
                ),
                Chromosome::x_chromosome(
                    DNASequence::from_str("ATGC").unwrap(),
                    1000,
                ),
            ),
            total_chromosome_count: 46,
        }
    }

    pub fn is_male(&self) -> bool {
        matches!(self.sex_chromosomes,
            SexChromosomes::XY(_, _) |
            SexChromosomes::XYY(_, _, _))
    }

    pub fn is_female(&self) -> bool {
        matches!(self.sex_chromosomes,
            SexChromosomes::XX(_, _) |
            SexChromosomes::XXX(_, _, _))
    }

    pub fn has_chromosomal_abnormality(&self) -> bool {
        self.total_chromosome_count != 46
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chromosome_creation() {
        let seq = DNASequence::from_str("ATGCATGCATGC").unwrap();
        let chr = Chromosome::autosome(1, seq, 5000);
        assert_eq!(chr.number, 1);
        assert!(!chr.is_sex_chromosome);
    }

    #[test]
    fn test_arm_lengths() {
        let seq = DNASequence::from_str("ATGCATGCATGC").unwrap();
        let chr = Chromosome::new(1, seq, 6);
        assert_eq!(chr.p_arm_length(), 6);
        assert_eq!(chr.q_arm_length(), 6);
    }

    #[test]
    fn test_karyotype_normal_male() {
        let karyotype = Karyotype::normal_male();
        assert!(karyotype.is_male());
        assert!(!karyotype.is_female());
        assert!(!karyotype.has_chromosomal_abnormality());
    }

    #[test]
    fn test_karyotype_normal_female() {
        let karyotype = Karyotype::normal_female();
        assert!(karyotype.is_female());
        assert!(!karyotype.is_male());
        assert!(!karyotype.has_chromosomal_abnormality());
    }
}
