use crate::chimie::Espece;

pub struct EtatThermochimique {
    pub volume_litres: f64,
    pub mmol_par_particule: f64,
    pub temperature_c: f64,
    pub temp_ambiante_c: f64,
    pub puissance_thermostat: f64,
    pub ph: f64,
    pub conductivite_ms_cm: f64,
    pub enthalpie_cumulee_j: f64,
}

impl Default for EtatThermochimique {
    fn default() -> Self {
        Self {
            volume_litres: 0.25,
            mmol_par_particule: 0.50,
            temperature_c: 25.0,
            temp_ambiante_c: 25.0,
            puissance_thermostat: 0.0,
            ph: 7.0,
            conductivite_ms_cm: 0.05,
            enthalpie_cumulee_j: 0.0,
        }
    }
}

impl EtatThermochimique {
    pub fn mettre_a_jour_rk4(
        &mut self,
        dt: f64,
        chaleur_reaction_joules: f64,
        compteurs: &[usize; 20],
    ) {
        self.enthalpie_cumulee_j += chaleur_reaction_joules;

        let c_p_effectif = 120.0;
        let coeff_pertes_h = 4.5;
        let q_point_reac = chaleur_reaction_joules / dt.max(1e-5);

        let derivee_temp = |temp: f64| -> f64 {
            let pertes = coeff_pertes_h * (temp - self.temp_ambiante_c);
            (q_point_reac + self.puissance_thermostat - pertes) / c_p_effectif
        };

        let t0 = self.temperature_c;
        let k1 = derivee_temp(t0);
        let k2 = derivee_temp(t0 + 0.5 * dt * k1);
        let k3 = derivee_temp(t0 + 0.5 * dt * k2);
        let k4 = derivee_temp(t0 + dt * k3);

        self.temperature_c = (t0 + (dt / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4)).clamp(0.0, 100.0);

        let facteur_mol_l = (self.mmol_par_particule * 1e-3) / self.volume_litres;
        let c_h = compteurs[Espece::HPlus.index()] as f64 * facteur_mol_l;
        let c_oh = compteurs[Espece::OHMoins.index()] as f64 * facteur_mol_l;
        let c_nh3 = compteurs[Espece::NH3Aq.index()] as f64 * facteur_mol_l;
        let c_co3 = compteurs[Espece::CO3DeuxMoins.index()] as f64 * facteur_mol_l;
        let c_fe = compteurs[Espece::Fe3Plus.index()] as f64 * facteur_mol_l;

        let bilan_protons = c_h + 0.04 * c_fe - (c_oh + 0.15 * c_co3 + 0.12 * c_nh3);
        let echelle = 0.008;
        let ph_cible = (7.0 - 2.6 * (bilan_protons / echelle).sinh().asinh()).clamp(0.4, 13.8);

        self.ph += (ph_cible - self.ph) * (6.0 * dt).min(1.0);
        self.ph = self.ph.clamp(0.0, 14.0);

        let mut sigma_cible = 0.08;
        for esp in Espece::TOUTES {
            let c_i = compteurs[esp.index()] as f64 * facteur_mol_l;
            sigma_cible += c_i * esp.conductivite_molaire();
        }

        let corr_temp = 1.0 + 0.02 * (self.temperature_c - 25.0);
        let sigma_temp = (sigma_cible * corr_temp).max(0.05);
        self.conductivite_ms_cm += (sigma_temp - self.conductivite_ms_cm) * (8.0 * dt).min(1.0);
    }
}