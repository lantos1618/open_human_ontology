use std::time::Duration;

/// Represents a collagen crosslink and its formation process
pub struct Crosslink {
    /// Type of crosslink
    crosslink_type: CrosslinkType,
    /// Current maturation state
    maturity: MaturityState,
    /// Location in collagen molecule
    location: CrosslinkSite,
    /// Physical properties
    properties: CrosslinkProperties,
    /// Formation time
    formation_time: Duration,
}

/// Types of crosslinks in collagen
#[derive(Clone, Copy, PartialEq)]
pub enum CrosslinkType {
    /// Immature divalent crosslinks
    DHLNL,
    HLNL,
    /// Mature trivalent crosslinks
    Pyridinoline,
    Deoxypyridinoline,
}

/// Maturation states of crosslinks
#[derive(Clone, Copy, PartialEq)]
pub enum MaturityState {
    Immature,
    Intermediate,
    Mature,
}

/// Location of crosslink in collagen
pub struct CrosslinkSite {
    /// Telopeptide position
    telopeptide: TelopeptideLocation,
    /// Helix position
    helix: HelicalPosition,
    /// Involved residues
    residues: [AminoAcid; 2],
}

/// Telopeptide regions
#[derive(Clone, Copy)]
pub enum TelopeptideLocation {
    NTerminal,
    CTerminal,
}

/// Position in helical region
pub struct HelicalPosition {
    /// Residue number
    residue: usize,
    /// Chain number
    chain: usize,
}

/// Amino acid types involved in crosslinking
#[derive(Clone, Copy)]
pub enum AminoAcid {
    Lysine,
    Hydroxylysine,
    AllysineAldehyde,
    HydroxyallysinAldehyde,
}

/// Physical and chemical properties
pub struct CrosslinkProperties {
    /// Bond strength (relative units)
    strength: f64,
    /// Chemical stability
    stability: f64,
    /// Mechanical contribution
    mechanical_effect: f64,
}

impl Crosslink {
    /// Create a new crosslink of specified type
    pub fn new(crosslink_type: CrosslinkType, location: CrosslinkSite) -> Self {
        Crosslink {
            crosslink_type,
            maturity: MaturityState::Immature,
            location,
            properties: CrosslinkProperties {
                strength: Self::initial_strength(crosslink_type),
                stability: Self::initial_stability(crosslink_type),
                mechanical_effect: 1.0,
            },
            formation_time: Duration::from_secs(0),
        }
    }

    /// Initial strength based on crosslink type
    fn initial_strength(crosslink_type: CrosslinkType) -> f64 {
        match crosslink_type {
            CrosslinkType::DHLNL | CrosslinkType::HLNL => 1.0,
            CrosslinkType::Pyridinoline | CrosslinkType::Deoxypyridinoline => 2.0,
        }
    }

    /// Initial stability based on crosslink type
    fn initial_stability(crosslink_type: CrosslinkType) -> f64 {
        match crosslink_type {
            CrosslinkType::DHLNL | CrosslinkType::HLNL => 0.7,
            CrosslinkType::Pyridinoline | CrosslinkType::Deoxypyridinoline => 0.9,
        }
    }

    /// Progress crosslink maturation
    pub fn mature(&mut self, time: Duration) {
        self.formation_time += time;
        
        // Update maturity state based on time
        self.maturity = match self.formation_time.as_secs() {
            t if t < 7 * 24 * 3600 => MaturityState::Immature,
            t if t < 30 * 24 * 3600 => MaturityState::Intermediate,
            _ => MaturityState::Mature,
        };

        // Update properties based on maturity
        self.update_properties();
    }

    /// Update properties based on maturity
    fn update_properties(&mut self) {
        let maturity_factor = match self.maturity {
            MaturityState::Immature => 1.0,
            MaturityState::Intermediate => 1.5,
            MaturityState::Mature => 2.0,
        };

        self.properties.strength *= maturity_factor;
        self.properties.stability *= maturity_factor;
        self.properties.mechanical_effect *= maturity_factor;
    }

    /// Calculate contribution to tissue strength
    pub fn calculate_strength_contribution(&self) -> f64 {
        self.properties.strength * self.properties.mechanical_effect
    }

    /// Check if crosslink is chemically stable
    pub fn is_stable(&self, ph: f64, temperature: f64) -> bool {
        // pH stability (optimal around 7.4)
        let ph_factor = 1.0 - (ph - 7.4).abs() / 7.4;
        
        // Temperature stability (optimal around 37°C)
        let temp_factor = 1.0 - (temperature - 37.0).abs() / 37.0;
        
        let stability = self.properties.stability * ph_factor * temp_factor;
        stability > 0.5
    }
}

/// Represents the enzymatic formation of crosslinks
pub struct CrosslinkFormation {
    /// Enzyme activity
    enzyme_activity: f64,
    /// Environmental conditions
    conditions: FormationConditions,
    /// Formation rate
    rate: f64,
}

/// Environmental conditions for formation
pub struct FormationConditions {
    /// pH level
    ph: f64,
    /// Temperature in Celsius
    temperature: f64,
    /// Oxygen availability
    oxygen_level: f64,
}

impl CrosslinkFormation {
    /// Create new formation process
    pub fn new(enzyme_activity: f64, conditions: FormationConditions) -> Self {
        let rate = Self::calculate_rate(enzyme_activity, &conditions);
        
        CrosslinkFormation {
            enzyme_activity,
            conditions,
            rate,
        }
    }

    /// Calculate formation rate based on conditions
    fn calculate_rate(enzyme_activity: f64, conditions: &FormationConditions) -> f64 {
        let ph_factor = 1.0 - (conditions.ph - 7.4).abs() / 7.4;
        let temp_factor = 1.0 - (conditions.temperature - 37.0).abs() / 37.0;
        let oxygen_factor = conditions.oxygen_level;

        enzyme_activity * ph_factor * temp_factor * oxygen_factor
    }

    /// Initiate crosslink formation
    pub fn form_crosslink(&self, site: CrosslinkSite) -> Option<Crosslink> {
        if self.rate > 0.5 {
            // Determine crosslink type based on conditions
            let crosslink_type = if self.conditions.oxygen_level > 0.8 {
                CrosslinkType::DHLNL
            } else {
                CrosslinkType::HLNL
            };

            Some(Crosslink::new(crosslink_type, site))
        } else {
            None
        }
    }

    /// Update formation conditions
    pub fn update_conditions(&mut self, new_conditions: FormationConditions) {
        self.conditions = new_conditions;
        self.rate = Self::calculate_rate(self.enzyme_activity, &self.conditions);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crosslink_maturation() {
        let site = CrosslinkSite {
            telopeptide: TelopeptideLocation::NTerminal,
            helix: HelicalPosition { residue: 87, chain: 1 },
            residues: [AminoAcid::Lysine, AminoAcid::Hydroxylysine],
        };
        
        let mut crosslink = Crosslink::new(CrosslinkType::DHLNL, site);
        assert_eq!(crosslink.maturity, MaturityState::Immature);
        
        crosslink.mature(Duration::from_secs(31 * 24 * 3600));
        assert_eq!(crosslink.maturity, MaturityState::Mature);
    }

    #[test]
    fn test_crosslink_formation() {
        let conditions = FormationConditions {
            ph: 7.4,
            temperature: 37.0,
            oxygen_level: 0.9,
        };
        
        let formation = CrosslinkFormation::new(1.0, conditions);
        let site = CrosslinkSite {
            telopeptide: TelopeptideLocation::NTerminal,
            helix: HelicalPosition { residue: 87, chain: 1 },
            residues: [AminoAcid::Lysine, AminoAcid::Hydroxylysine],
        };
        
        let crosslink = formation.form_crosslink(site);
        assert!(crosslink.is_some());
    }

    #[test]
    fn test_stability_conditions() {
        let site = CrosslinkSite {
            telopeptide: TelopeptideLocation::NTerminal,
            helix: HelicalPosition { residue: 87, chain: 1 },
            residues: [AminoAcid::Lysine, AminoAcid::Hydroxylysine],
        };
        
        let crosslink = Crosslink::new(CrosslinkType::Pyridinoline, site);
        assert!(crosslink.is_stable(7.4, 37.0));
        assert!(!crosslink.is_stable(4.0, 60.0));
    }
} 