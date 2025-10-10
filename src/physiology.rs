use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegratedPhysiology {
    pub metabolic_state: MetabolicState,
    pub hydration: HydrationStatus,
    pub electrolytes: ElectrolyteBalance,
    pub acid_base: AcidBaseBalance,
    pub oxygen_delivery: OxygenDeliverySystem,
    pub temperature_regulation: ThermoregulationSystem,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MetabolicState {
    Fasting,
    Fed,
    Postprandial,
    Exercise,
    Sleep,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydrationStatus {
    pub total_body_water_l: f64,
    pub intracellular_fluid_l: f64,
    pub extracellular_fluid_l: f64,
    pub plasma_volume_l: f64,
    pub osmolality_mosm_per_kg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectrolyteBalance {
    pub sodium_meq_per_l: f64,
    pub potassium_meq_per_l: f64,
    pub chloride_meq_per_l: f64,
    pub bicarbonate_meq_per_l: f64,
    pub calcium_mg_per_dl: f64,
    pub magnesium_mg_per_dl: f64,
    pub phosphate_mg_per_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcidBaseBalance {
    pub ph: f64,
    pub pco2_mmhg: f64,
    pub po2_mmhg: f64,
    pub hco3_meq_per_l: f64,
    pub base_excess_meq_per_l: f64,
    pub anion_gap_meq_per_l: f64,
    pub status: AcidBaseStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcidBaseStatus {
    Normal,
    MetabolicAcidosis,
    MetabolicAlkalosis,
    RespiratoryAcidosis,
    RespiratoryAlkalosis,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxygenDeliverySystem {
    pub arterial_o2_content_ml_per_dl: f64,
    pub venous_o2_content_ml_per_dl: f64,
    pub o2_extraction_ratio: f64,
    pub tissue_oxygen_delivery_ml_per_min: f64,
    pub vo2_max_ml_kg_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermoregulationSystem {
    pub core_temperature_c: f64,
    pub skin_temperature_c: f64,
    pub metabolic_heat_production_w: f64,
    pub heat_loss_mechanisms: HeatLossMechanisms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatLossMechanisms {
    pub radiation_w: f64,
    pub convection_w: f64,
    pub evaporation_w: f64,
    pub conduction_w: f64,
}

impl IntegratedPhysiology {
    pub fn new_adult_normal(body_weight_kg: f64) -> Self {
        Self {
            metabolic_state: MetabolicState::Fed,
            hydration: HydrationStatus::new_normal(body_weight_kg),
            electrolytes: ElectrolyteBalance::new_normal(),
            acid_base: AcidBaseBalance::new_normal(),
            oxygen_delivery: OxygenDeliverySystem::new_normal(body_weight_kg),
            temperature_regulation: ThermoregulationSystem::new_normal(),
        }
    }

    pub fn is_homeostatic(&self) -> bool {
        self.hydration.is_normal()
            && self.electrolytes.is_normal()
            && self.acid_base.is_normal()
            && self.temperature_regulation.is_normal()
    }

    pub fn total_osmolality(&self) -> f64 {
        2.0 * self.electrolytes.sodium_meq_per_l
            + self.hydration.osmolality_mosm_per_kg / 18.0
            + 0.055 * 100.0
    }
}

impl HydrationStatus {
    pub fn new_normal(body_weight_kg: f64) -> Self {
        let total_water = body_weight_kg * 0.6;
        let icf = total_water * 0.67;
        let ecf = total_water * 0.33;
        let plasma = ecf * 0.25;

        Self {
            total_body_water_l: total_water,
            intracellular_fluid_l: icf,
            extracellular_fluid_l: ecf,
            plasma_volume_l: plasma,
            osmolality_mosm_per_kg: 285.0,
        }
    }

    pub fn is_normal(&self) -> bool {
        self.osmolality_mosm_per_kg >= 275.0 && self.osmolality_mosm_per_kg <= 295.0
    }

    pub fn hydration_status(&self) -> &str {
        if self.osmolality_mosm_per_kg < 275.0 {
            "Overhydrated"
        } else if self.osmolality_mosm_per_kg > 295.0 {
            "Dehydrated"
        } else {
            "Normal"
        }
    }
}

impl ElectrolyteBalance {
    pub fn new_normal() -> Self {
        Self {
            sodium_meq_per_l: 140.0,
            potassium_meq_per_l: 4.0,
            chloride_meq_per_l: 105.0,
            bicarbonate_meq_per_l: 24.0,
            calcium_mg_per_dl: 9.5,
            magnesium_mg_per_dl: 2.0,
            phosphate_mg_per_dl: 3.5,
        }
    }

    pub fn is_normal(&self) -> bool {
        (135.0..=145.0).contains(&self.sodium_meq_per_l)
            && (3.5..=5.0).contains(&self.potassium_meq_per_l)
            && (98.0..=108.0).contains(&self.chloride_meq_per_l)
            && (22.0..=28.0).contains(&self.bicarbonate_meq_per_l)
    }

    pub fn anion_gap(&self) -> f64 {
        self.sodium_meq_per_l - self.chloride_meq_per_l - self.bicarbonate_meq_per_l
    }
}

impl AcidBaseBalance {
    pub fn new_normal() -> Self {
        Self {
            ph: 7.40,
            pco2_mmhg: 40.0,
            po2_mmhg: 95.0,
            hco3_meq_per_l: 24.0,
            base_excess_meq_per_l: 0.0,
            anion_gap_meq_per_l: 12.0,
            status: AcidBaseStatus::Normal,
        }
    }

    pub fn is_normal(&self) -> bool {
        (7.35..=7.45).contains(&self.ph)
            && (35.0..=45.0).contains(&self.pco2_mmhg)
            && (22.0..=26.0).contains(&self.hco3_meq_per_l)
    }

    pub fn classify_status(&self) -> AcidBaseStatus {
        let acidotic = self.ph < 7.35;
        let alkalotic = self.ph > 7.45;
        let respiratory_acid = self.pco2_mmhg > 45.0;
        let respiratory_alk = self.pco2_mmhg < 35.0;
        let metabolic_acid = self.hco3_meq_per_l < 22.0;
        let metabolic_alk = self.hco3_meq_per_l > 26.0;

        if acidotic && respiratory_acid && !metabolic_acid {
            AcidBaseStatus::RespiratoryAcidosis
        } else if acidotic && metabolic_acid && !respiratory_acid {
            AcidBaseStatus::MetabolicAcidosis
        } else if alkalotic && respiratory_alk && !metabolic_alk {
            AcidBaseStatus::RespiratoryAlkalosis
        } else if alkalotic && metabolic_alk && !respiratory_alk {
            AcidBaseStatus::MetabolicAlkalosis
        } else if acidotic || alkalotic {
            AcidBaseStatus::Mixed
        } else {
            AcidBaseStatus::Normal
        }
    }
}

impl OxygenDeliverySystem {
    pub fn new_normal(_body_weight_kg: f64) -> Self {
        let cardiac_output_l_min = 5.0;
        let arterial_content = 20.0;
        let venous_content = 15.0;
        let extraction = (arterial_content - venous_content) / arterial_content;
        let do2 = cardiac_output_l_min * arterial_content * 10.0;

        Self {
            arterial_o2_content_ml_per_dl: arterial_content,
            venous_o2_content_ml_per_dl: venous_content,
            o2_extraction_ratio: extraction,
            tissue_oxygen_delivery_ml_per_min: do2,
            vo2_max_ml_kg_min: 35.0,
        }
    }

    pub fn oxygen_consumption_ml_per_min(&self) -> f64 {
        (self.arterial_o2_content_ml_per_dl - self.venous_o2_content_ml_per_dl) * 5.0 * 10.0
    }

    pub fn is_adequate(&self) -> bool {
        self.o2_extraction_ratio < 0.4 && self.tissue_oxygen_delivery_ml_per_min > 500.0
    }
}

impl ThermoregulationSystem {
    pub fn new_normal() -> Self {
        Self {
            core_temperature_c: 37.0,
            skin_temperature_c: 33.0,
            metabolic_heat_production_w: 80.0,
            heat_loss_mechanisms: HeatLossMechanisms::new_normal(),
        }
    }

    pub fn is_normal(&self) -> bool {
        (36.5..=37.5).contains(&self.core_temperature_c)
    }

    pub fn thermal_status(&self) -> &str {
        if self.core_temperature_c < 35.0 {
            "Hypothermic"
        } else if self.core_temperature_c < 36.5 {
            "Mild hypothermia"
        } else if self.core_temperature_c > 38.0 {
            "Febrile"
        } else if self.core_temperature_c > 40.0 {
            "Hyperthermic"
        } else {
            "Normothermic"
        }
    }

    pub fn total_heat_loss_w(&self) -> f64 {
        self.heat_loss_mechanisms.total_heat_loss()
    }

    pub fn heat_balance_w(&self) -> f64 {
        self.metabolic_heat_production_w - self.total_heat_loss_w()
    }
}

impl HeatLossMechanisms {
    pub fn new_normal() -> Self {
        Self {
            radiation_w: 40.0,
            convection_w: 15.0,
            evaporation_w: 20.0,
            conduction_w: 5.0,
        }
    }

    pub fn total_heat_loss(&self) -> f64 {
        self.radiation_w + self.convection_w + self.evaporation_w + self.conduction_w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integrated_physiology_creation() {
        let phys = IntegratedPhysiology::new_adult_normal(70.0);
        assert!(phys.is_homeostatic());
    }

    #[test]
    fn test_hydration_status() {
        let status = HydrationStatus::new_normal(70.0);
        assert!(status.is_normal());
        assert_eq!(status.hydration_status(), "Normal");
        assert!((status.total_body_water_l - 42.0).abs() < 1.0);
    }

    #[test]
    fn test_electrolyte_balance() {
        let elec = ElectrolyteBalance::new_normal();
        assert!(elec.is_normal());
        let ag = elec.anion_gap();
        assert!((ag - 11.0).abs() < 2.0);
    }

    #[test]
    fn test_acid_base_normal() {
        let ab = AcidBaseBalance::new_normal();
        assert!(ab.is_normal());
        assert_eq!(ab.status, AcidBaseStatus::Normal);
    }

    #[test]
    fn test_oxygen_delivery() {
        let o2 = OxygenDeliverySystem::new_normal(70.0);
        assert!(o2.is_adequate());
        let vo2 = o2.oxygen_consumption_ml_per_min();
        assert!(vo2 > 200.0);
    }

    #[test]
    fn test_thermoregulation() {
        let thermo = ThermoregulationSystem::new_normal();
        assert!(thermo.is_normal());
        assert_eq!(thermo.thermal_status(), "Normothermic");
        let balance = thermo.heat_balance_w();
        assert!(balance.abs() < 5.0);
    }
}
