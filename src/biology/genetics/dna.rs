use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Nucleotide {
    Adenine,
    Thymine,
    Guanine,
    Cytosine,
}

impl Nucleotide {
    pub fn complement(&self) -> Self {
        match self {
            Nucleotide::Adenine => Nucleotide::Thymine,
            Nucleotide::Thymine => Nucleotide::Adenine,
            Nucleotide::Guanine => Nucleotide::Cytosine,
            Nucleotide::Cytosine => Nucleotide::Guanine,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Nucleotide::Adenine => 'A',
            Nucleotide::Thymine => 'T',
            Nucleotide::Guanine => 'G',
            Nucleotide::Cytosine => 'C',
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'A' => Some(Nucleotide::Adenine),
            'T' => Some(Nucleotide::Thymine),
            'G' => Some(Nucleotide::Guanine),
            'C' => Some(Nucleotide::Cytosine),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DNASequence {
    sequence: Vec<Nucleotide>,
}

impl DNASequence {
    pub fn new(sequence: Vec<Nucleotide>) -> Self {
        DNASequence { sequence }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let sequence: Option<Vec<Nucleotide>> = s.chars()
            .map(Nucleotide::from_char)
            .collect();
        sequence.map(DNASequence::new)
    }

    pub fn to_string(&self) -> String {
        self.sequence.iter()
            .map(|n| n.to_char())
            .collect()
    }

    pub fn len(&self) -> usize {
        self.sequence.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sequence.is_empty()
    }

    pub fn complement(&self) -> Self {
        DNASequence {
            sequence: self.sequence.iter()
                .map(|n| n.complement())
                .collect(),
        }
    }

    pub fn reverse_complement(&self) -> Self {
        DNASequence {
            sequence: self.sequence.iter()
                .rev()
                .map(|n| n.complement())
                .collect(),
        }
    }

    pub fn get(&self, index: usize) -> Option<Nucleotide> {
        self.sequence.get(index).copied()
    }

    pub fn slice(&self, start: usize, end: usize) -> Option<Self> {
        if start <= end && end <= self.sequence.len() {
            Some(DNASequence {
                sequence: self.sequence[start..end].to_vec(),
            })
        } else {
            None
        }
    }

    pub fn gc_content(&self) -> f64 {
        if self.sequence.is_empty() {
            return 0.0;
        }
        let gc_count = self.sequence.iter()
            .filter(|n| matches!(n, Nucleotide::Guanine | Nucleotide::Cytosine))
            .count();
        gc_count as f64 / self.sequence.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nucleotide_complement() {
        assert_eq!(Nucleotide::Adenine.complement(), Nucleotide::Thymine);
        assert_eq!(Nucleotide::Guanine.complement(), Nucleotide::Cytosine);
    }

    #[test]
    fn test_dna_sequence_from_str() {
        let seq = DNASequence::from_str("ATGC").unwrap();
        assert_eq!(seq.len(), 4);
        assert_eq!(seq.to_string(), "ATGC");
    }

    #[test]
    fn test_reverse_complement() {
        let seq = DNASequence::from_str("ATGC").unwrap();
        let rc = seq.reverse_complement();
        assert_eq!(rc.to_string(), "GCAT");
    }

    #[test]
    fn test_gc_content() {
        let seq = DNASequence::from_str("ATGC").unwrap();
        assert_eq!(seq.gc_content(), 0.5);
    }

    #[test]
    fn test_slice() {
        let seq = DNASequence::from_str("ATGCATGC").unwrap();
        let slice = seq.slice(2, 6).unwrap();
        assert_eq!(slice.to_string(), "GCAT");
    }
}
