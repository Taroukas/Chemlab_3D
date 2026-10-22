use eframe::egui::Color32;

#[derive(Clone, Copy)]
pub struct AtomeLocal {
    pub offset: [f32; 3],
    pub rayon: f32,
    pub couleur: Color32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Espece {
    HPlus,
    OHMoins,
    NH3Aq,
    CO3DeuxMoins,
    SO4DeuxMoins,
    ClMoins,
    IMoins,
    Cu2Plus,
    AgPlus,
    Fe3Plus,
    Pb2Plus,
    Ba2Plus,
    CuNH34Complexe,
    CuOH2Solide,
    CuCO3Solide,
    AgClSolide,
    PbI2Solide,
    FeOH3Solide,
    BaSO4Solide,
    CO2Gaz,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Aqueuse,
    Solide,
    Gazeuse,
}

impl Espece {
    pub const TOUTES: [Espece; 20] = [
        Espece::HPlus,
        Espece::OHMoins,
        Espece::NH3Aq,
        Espece::CO3DeuxMoins,
        Espece::SO4DeuxMoins,
        Espece::ClMoins,
        Espece::IMoins,
        Espece::Cu2Plus,
        Espece::AgPlus,
        Espece::Fe3Plus,
        Espece::Pb2Plus,
        Espece::Ba2Plus,
        Espece::CuNH34Complexe,
        Espece::CuOH2Solide,
        Espece::CuCO3Solide,
        Espece::AgClSolide,
        Espece::PbI2Solide,
        Espece::FeOH3Solide,
        Espece::BaSO4Solide,
        Espece::CO2Gaz,
    ];

    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn phase(&self) -> Phase {
        match self {
            Espece::CuOH2Solide
            | Espece::CuCO3Solide
            | Espece::AgClSolide
            | Espece::PbI2Solide
            | Espece::FeOH3Solide
            | Espece::BaSO4Solide => Phase::Solide,
            Espece::CO2Gaz => Phase::Gazeuse,
            _ => Phase::Aqueuse,
        }
    }

    pub fn symbole(&self) -> &'static str {
        match self {
            Espece::HPlus => "H3O+ (aq)",
            Espece::OHMoins => "OH- (aq)",
            Espece::NH3Aq => "NH3 (aq)",
            Espece::CO3DeuxMoins => "CO3(2-) (aq)",
            Espece::SO4DeuxMoins => "SO4(2-) (aq)",
            Espece::ClMoins => "Cl- (aq)",
            Espece::IMoins => "I- (aq)",
            Espece::Cu2Plus => "Cu2+ (aq)",
            Espece::AgPlus => "Ag+ (aq)",
            Espece::Fe3Plus => "Fe3+ (aq)",
            Espece::Pb2Plus => "Pb2+ (aq)",
            Espece::Ba2Plus => "Ba2+ (aq)",
            Espece::CuNH34Complexe => "[Cu(NH3)4]2+ (aq)",
            Espece::CuOH2Solide => "Cu(OH)2 (s)",
            Espece::CuCO3Solide => "CuCO3 (s)",
            Espece::AgClSolide => "AgCl (s)",
            Espece::PbI2Solide => "PbI2 (s) Golden",
            Espece::FeOH3Solide => "Fe(OH)3 (s)",
            Espece::BaSO4Solide => "BaSO4 (s)",
            Espece::CO2Gaz => "CO2 (g)",
        }
    }

    pub fn nom_geometrie(&self) -> &'static str {
        match self {
            Espece::HPlus => "Trigonal Pyramidal (107°)",
            Espece::OHMoins => "Linear Diatomic (180°)",
            Espece::NH3Aq => "Trigonal Pyramidal (107°)",
            Espece::CO3DeuxMoins => "Trigonal Planar (120°)",
            Espece::SO4DeuxMoins => "Tetrahedral (109.5°)",
            Espece::ClMoins => "Solvated Chloride",
            Espece::IMoins => "Solvated Iodide",
            Espece::Cu2Plus => "Octahedral (90°)",
            Espece::AgPlus => "Linear Dicoordinate (180°)",
            Espece::Fe3Plus => "Octahedral (90°)",
            Espece::Pb2Plus => "Hemidirected Pyramidal",
            Espece::Ba2Plus => "Octahedral (90°)",
            Espece::CuNH34Complexe => "Square Planar (90°)",
            Espece::CuOH2Solide => "Bent Crystalline Lattice",
            Espece::CuCO3Solide => "Malachite Crystal",
            Espece::AgClSolide => "Cubic Rock-Salt Crystal",
            Espece::PbI2Solide => "Hexagonal Golden Platelet",
            Espece::FeOH3Solide => "Octahedral Rust Lattice",
            Espece::BaSO4Solide => "Orthorhombic Barite",
            Espece::CO2Gaz => "Linear Triatomic (180°)",
        }
    }

    pub fn couleur(&self) -> Color32 {
        match self {
            Espece::HPlus => Color32::from_rgb(255, 75, 95),
            Espece::OHMoins => Color32::from_rgb(155, 100, 255),
            Espece::NH3Aq => Color32::from_rgb(80, 140, 255),
            Espece::CO3DeuxMoins => Color32::from_rgb(255, 205, 55),
            Espece::SO4DeuxMoins => Color32::from_rgb(245, 230, 70),
            Espece::ClMoins => Color32::from_rgb(80, 235, 120),
            Espece::IMoins => Color32::from_rgb(200, 110, 240),
            Espece::Cu2Plus => Color32::from_rgb(35, 185, 255),
            Espece::AgPlus => Color32::from_rgb(210, 220, 235),
            Espece::Fe3Plus => Color32::from_rgb(235, 140, 40),
            Espece::Pb2Plus => Color32::from_rgb(160, 175, 195),
            Espece::Ba2Plus => Color32::from_rgb(130, 235, 195),
            Espece::CuNH34Complexe => Color32::from_rgb(45, 75, 255),
            Espece::CuOH2Solide => Color32::from_rgb(25, 95, 235),
            Espece::CuCO3Solide => Color32::from_rgb(30, 205, 130),
            Espece::AgClSolide => Color32::from_rgb(240, 245, 255),
            Espece::PbI2Solide => Color32::from_rgb(255, 215, 20),
            Espece::FeOH3Solide => Color32::from_rgb(195, 75, 30),
            Espece::BaSO4Solide => Color32::from_rgb(235, 238, 245),
            Espece::CO2Gaz => Color32::from_rgb(225, 240, 255),
        }
    }

    pub fn conductivite_molaire(&self) -> f64 {
        match self {
            Espece::HPlus => 34.98,
            Espece::OHMoins => 19.80,
            Espece::SO4DeuxMoins => 16.00,
            Espece::CO3DeuxMoins => 13.86,
            Espece::Fe3Plus => 20.40,
            Espece::Ba2Plus => 12.72,
            Espece::Pb2Plus => 14.20,
            Espece::Cu2Plus | Espece::CuNH34Complexe => 10.72,
            Espece::AgPlus => 6.19,
            Espece::ClMoins => 7.63,
            Espece::IMoins => 7.68,
            Espece::NH3Aq => 0.40,
            _ => 0.0,
        }
    }

    pub fn geometrie_vsepr(&self) -> &'static [AtomeLocal] {
        const H_W: Color32 = Color32::from_rgb(242, 246, 255);
        const O_R: Color32 = Color32::from_rgb(255, 70, 90);
        const O_V: Color32 = Color32::from_rgb(160, 100, 255);
        const N_B: Color32 = Color32::from_rgb(65, 125, 255);
        const C_G: Color32 = Color32::from_rgb(95, 105, 120);
        const S_Y: Color32 = Color32::from_rgb(250, 225, 50);
        const CL_G: Color32 = Color32::from_rgb(70, 235, 115);
        const I_P: Color32 = Color32::from_rgb(190, 90, 235);
        const CU_C: Color32 = Color32::from_rgb(35, 185, 255);
        const AG_S: Color32 = Color32::from_rgb(215, 225, 240);
        const FE_O: Color32 = Color32::from_rgb(235, 130, 35);
        const PB_G: Color32 = Color32::from_rgb(145, 160, 180);
        const BA_T: Color32 = Color32::from_rgb(110, 230, 185);
        const GOLD: Color32 = Color32::from_rgb(255, 215, 15);
        const CU_COMPLEX: Color32 = Color32::from_rgb(35, 70, 255);
        const CU_ROYAL: Color32 = Color32::from_rgb(25, 85, 235);
        const CU_MALACHITE: Color32 = Color32::from_rgb(30, 205, 130);
        const FE_RUST: Color32 = Color32::from_rgb(195, 70, 25);

        match self {
            Espece::HPlus => &[
                AtomeLocal { offset: [0.0, 0.012, 0.0], rayon: 0.024, couleur: O_R },
                AtomeLocal { offset: [0.032, -0.014, 0.0], rayon: 0.015, couleur: H_W },
                AtomeLocal { offset: [-0.016, -0.014, 0.028], rayon: 0.015, couleur: H_W },
                AtomeLocal { offset: [-0.016, -0.014, -0.028], rayon: 0.015, couleur: H_W },
            ],
            Espece::OHMoins => &[
                AtomeLocal { offset: [-0.014, 0.0, 0.0], rayon: 0.025, couleur: O_V },
                AtomeLocal { offset: [0.022, 0.0, 0.0], rayon: 0.016, couleur: H_W },
            ],
            Espece::NH3Aq => &[
                AtomeLocal { offset: [0.0, 0.012, 0.0], rayon: 0.024, couleur: N_B },
                AtomeLocal { offset: [0.031, -0.014, 0.0], rayon: 0.015, couleur: H_W },
                AtomeLocal { offset: [-0.015, -0.014, 0.027], rayon: 0.015, couleur: H_W },
                AtomeLocal { offset: [-0.015, -0.014, -0.027], rayon: 0.015, couleur: H_W },
            ],
            Espece::CO3DeuxMoins => &[
                AtomeLocal { offset: [0.0, 0.0, 0.0], rayon: 0.021, couleur: C_G },
                AtomeLocal { offset: [0.036, 0.0, 0.0], rayon: 0.019, couleur: S_Y },
                AtomeLocal { offset: [-0.018, 0.0, 0.031], rayon: 0.019, couleur: S_Y },
                AtomeLocal { offset: [-0.018, 0.0, -0.031], rayon: 0.019, couleur: S_Y },
            ],
            Espece::SO4DeuxMoins => &[
                AtomeLocal { offset: [0.0, 0.0, 0.0], rayon: 0.024, couleur: S_Y },
                AtomeLocal { offset: [0.0, 0.036, 0.0], rayon: 0.018, couleur: O_R },
                AtomeLocal { offset: [0.034, -0.012, 0.0], rayon: 0.018, couleur: O_R },
                AtomeLocal { offset: [-0.017, -0.012, 0.029], rayon: 0.018, couleur: O_R },
                AtomeLocal { offset: [-0.017, -0.012, -0.029], rayon: 0.018, couleur: O_R },
            ],
            Espece::ClMoins => &[
                AtomeLocal { offset: [0.0, 0.0, 0.0], rayon: 0.025, couleur: CL_G },
                AtomeLocal { offset: [0.032, 0.0, 0.0], rayon: 0.012, couleur: H_W },
                AtomeLocal { offset: [-0.032, 0.0, 0.0], rayon: 0.012, couleur: H_W },
            ],
            Espece::IMoins => &[
                AtomeLocal { offset: [0.0, 0.0, 0.0], rayon: 0.028, couleur: I_P },
                AtomeLocal { offset: [0.0, 0.035, 0.0], rayon: 0.013, couleur: H_W },
                AtomeLocal { offset: [0.0, -0.035, 0.0], rayon: 0.013, couleur: H_W },
            ],
            Espece::Cu2Plus => &[
                AtomeLocal { offset: [0.0, 0.0, 0.0], rayon: 0.026, couleur: CU_C },
                AtomeLocal { offset: [0.036, 0.0, 0.0], rayon: 0.014, couleur: H_W },
                AtomeLocal { offset: [-0.036, 0.0, 0.0], rayon: 0.014, couleur: H_W },
                AtomeLocal { offset: [0.0, 0.0, 0.036], rayon: 0.014, couleur: H_W },
                AtomeLocal { offset: [0.0, 0.0, -0.036], rayon: 0.014, couleur: H_W },
                AtomeLocal { offset: [0.0, 0.036, 0.0], rayon: 0.014, couleur: H_W },
                AtomeLocal { offset: [0.0, -0.036, 0.0], rayon: 0.014, couleur: H_W },
            ],
            Espece::AgPlus => &[
                AtomeLocal { offset: [0.0, 0.0, 0.0], rayon: 0.026, couleur: AG_S },
                AtomeLocal { offset: [0.036, 0.0, 0.0], rayon: 0.015, couleur: H_W },
                AtomeLocal { offset: [-0.036, 0.0, 0.0], rayon: 0.015, couleur: H_W },
            ],
            Espece::Fe3Plus => &[
                AtomeLocal { offset: [0.0, 0.0, 0.0], rayon: 0.026, couleur: FE_O },
                AtomeLocal { offset: [0.036, 0.0, 0.0], rayon: 0.014, couleur: O_R },
                AtomeLocal { offset: [-0.036, 0.0, 0.0], rayon: 0.014, couleur: O_R },
                AtomeLocal { offset: [0.0, 0.0, 0.036], rayon: 0.014, couleur: O_R },
                AtomeLocal { offset: [0.0, 0.0, -0.036], rayon: 0.014, couleur: O_R },
                AtomeLocal { offset: [0.0, 0.036, 0.0], rayon: 0.014, couleur: O_R },
                AtomeLocal { offset: [0.0, -0.036, 0.0], rayon: 0.014, couleur: O_R },
            ],
            Espece::Pb2Plus => &[
                AtomeLocal { offset: [0.0, 0.014, 0.0], rayon: 0.028, couleur: PB_G },
                AtomeLocal { offset: [0.034, -0.016, 0.0], rayon: 0.015, couleur: O_R },
                AtomeLocal { offset: [-0.017, -0.016, 0.029], rayon: 0.015, couleur: O_R },
                AtomeLocal { offset: [-0.017, -0.016, -0.029], rayon: 0.015, couleur: O_R },
            ],
            Espece::Ba2Plus => &[
                AtomeLocal { offset: [0.0, 0.0, 0.0], rayon: 0.027, couleur: BA_T },
                AtomeLocal { offset: [0.036, 0.0, 0.0], rayon: 0.014, couleur: H_W },
                AtomeLocal { offset: [-0.036, 0.0, 0.0], rayon: 0.014, couleur: H_W },
                AtomeLocal { offset: [0.0, 0.0, 0.036], rayon: 0.014, couleur: H_W },
                AtomeLocal { offset: [0.0, 0.0, -0.036], rayon: 0.014, couleur: H_W },
            ],
            Espece::CuNH34Complexe => &[
                AtomeLocal { offset: [0.0, 0.0, 0.0], rayon: 0.028, couleur: CU_COMPLEX },
                AtomeLocal { offset: [0.038, 0.0, 0.0], rayon: 0.018, couleur: N_B },
                AtomeLocal { offset: [-0.038, 0.0, 0.0], rayon: 0.018, couleur: N_B },
                AtomeLocal { offset: [0.0, 0.0, 0.038], rayon: 0.018, couleur: N_B },
                AtomeLocal { offset: [0.0, 0.0, -0.038], rayon: 0.018, couleur: N_B },
            ],
            Espece::CuOH2Solide => &[
                AtomeLocal { offset: [0.0, 0.010, 0.0], rayon: 0.028, couleur: CU_ROYAL },
                AtomeLocal { offset: [0.034, -0.016, 0.014], rayon: 0.020, couleur: O_V },
                AtomeLocal { offset: [-0.034, -0.016, -0.014], rayon: 0.020, couleur: O_V },
                AtomeLocal { offset: [0.050, -0.004, 0.022], rayon: 0.014, couleur: H_W },
                AtomeLocal { offset: [-0.050, -0.004, -0.022], rayon: 0.014, couleur: H_W },
            ],
            Espece::CuCO3Solide => &[
                AtomeLocal { offset: [-0.018, 0.0, 0.0], rayon: 0.028, couleur: CU_MALACHITE },
                AtomeLocal { offset: [0.022, 0.0, 0.0], rayon: 0.020, couleur: C_G },
                AtomeLocal { offset: [0.005, 0.028, 0.015], rayon: 0.018, couleur: S_Y },
                AtomeLocal { offset: [0.005, -0.028, -0.015], rayon: 0.018, couleur: S_Y },
                AtomeLocal { offset: [0.048, 0.0, 0.0], rayon: 0.018, couleur: S_Y },
            ],
            Espece::AgClSolide => &[
                AtomeLocal { offset: [-0.018, -0.018, 0.0], rayon: 0.024, couleur: AG_S },
                AtomeLocal { offset: [0.018, -0.018, 0.0], rayon: 0.024, couleur: CL_G },
                AtomeLocal { offset: [0.018, 0.018, 0.0], rayon: 0.024, couleur: AG_S },
                AtomeLocal { offset: [-0.018, 0.018, 0.0], rayon: 0.024, couleur: CL_G },
            ],
            Espece::PbI2Solide => &[
                AtomeLocal { offset: [0.0, 0.0, 0.0], rayon: 0.027, couleur: GOLD },
                AtomeLocal { offset: [0.038, 0.012, 0.0], rayon: 0.023, couleur: GOLD },
                AtomeLocal { offset: [-0.038, -0.012, 0.0], rayon: 0.023, couleur: GOLD },
                AtomeLocal { offset: [0.0, 0.012, 0.038], rayon: 0.020, couleur: I_P },
                AtomeLocal { offset: [0.0, -0.012, -0.038], rayon: 0.020, couleur: I_P },
            ],
            Espece::FeOH3Solide => &[
                AtomeLocal { offset: [0.0, 0.0, 0.0], rayon: 0.028, couleur: FE_RUST },
                AtomeLocal { offset: [0.036, 0.012, 0.0], rayon: 0.020, couleur: O_R },
                AtomeLocal { offset: [-0.018, 0.012, 0.031], rayon: 0.020, couleur: O_R },
                AtomeLocal { offset: [-0.018, 0.012, -0.031], rayon: 0.020, couleur: O_R },
            ],
            Espece::BaSO4Solide => &[
                AtomeLocal { offset: [-0.020, 0.0, 0.0], rayon: 0.027, couleur: H_W },
                AtomeLocal { offset: [0.020, 0.0, 0.0], rayon: 0.024, couleur: S_Y },
                AtomeLocal { offset: [0.020, 0.030, 0.020], rayon: 0.018, couleur: O_R },
                AtomeLocal { offset: [0.020, -0.030, -0.020], rayon: 0.018, couleur: O_R },
            ],
            Espece::CO2Gaz => &[
                AtomeLocal { offset: [0.0, 0.0, 0.0], rayon: 0.021, couleur: C_G },
                AtomeLocal { offset: [0.038, 0.0, 0.0], rayon: 0.021, couleur: H_W },
                AtomeLocal { offset: [-0.038, 0.0, 0.0], rayon: 0.021, couleur: H_W },
            ],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CategorieCatalogue {
    Tous,
    Acides,
    Bases,
    Metaux,
    Sels,
}

pub struct ReactifCatalogue {
    pub nom: &'static str,
    pub formule: &'static str,
    pub categorie: CategorieCatalogue,
    pub description_vsepr: &'static str,
    pub ions: &'static [(Espece, usize)],
}

pub const CATALOGUE_REACTIFS: &[ReactifCatalogue] = &[
    ReactifCatalogue {
        nom: "Hydrochloric Acid",
        formule: "HCl (aq)",
        categorie: CategorieCatalogue::Acides,
        description_vsepr: "Releases Pyramidal H3O+ & Cl-",
        ions: &[(Espece::HPlus, 1), (Espece::ClMoins, 1)],
    },
    ReactifCatalogue {
        nom: "Sulfuric Acid",
        formule: "H2SO4 (aq)",
        categorie: CategorieCatalogue::Acides,
        description_vsepr: "Diprotic: 2 H3O+ & Tetrahedral SO4(2-)",
        ions: &[(Espece::HPlus, 2), (Espece::SO4DeuxMoins, 1)],
    },
    ReactifCatalogue {
        nom: "Sodium Hydroxide",
        formule: "NaOH (aq)",
        categorie: CategorieCatalogue::Bases,
        description_vsepr: "Strong Base: Linear OH- (180°)",
        ions: &[(Espece::OHMoins, 1)],
    },
    ReactifCatalogue {
        nom: "Aqueous Ammonia",
        formule: "NH3 (aq)",
        categorie: CategorieCatalogue::Bases,
        description_vsepr: "Trigonal Pyramidal NH3 (107°) & Weak OH-",
        ions: &[(Espece::NH3Aq, 2), (Espece::OHMoins, 1)],
    },
    ReactifCatalogue {
        nom: "Sodium Carbonate",
        formule: "Na2CO3 (aq)",
        categorie: CategorieCatalogue::Bases,
        description_vsepr: "Trigonal Planar CO3(2-) (120°)",
        ions: &[(Espece::CO3DeuxMoins, 1)],
    },
    ReactifCatalogue {
        nom: "Copper(II) Sulfate",
        formule: "CuSO4 (aq)",
        categorie: CategorieCatalogue::Metaux,
        description_vsepr: "Octahedral Cu2+ (90°) & Tetrahedral SO4(2-)",
        ions: &[(Espece::Cu2Plus, 1), (Espece::SO4DeuxMoins, 1)],
    },
    ReactifCatalogue {
        nom: "Silver Nitrate",
        formule: "AgNO3 (aq)",
        categorie: CategorieCatalogue::Metaux,
        description_vsepr: "Linear Coordinated Ag+ (180°)",
        ions: &[(Espece::AgPlus, 1)],
    },
    ReactifCatalogue {
        nom: "Iron(III) Chloride",
        formule: "FeCl3 (aq)",
        categorie: CategorieCatalogue::Metaux,
        description_vsepr: "Octahedral Fe3+ & Chloride Cl-",
        ions: &[(Espece::Fe3Plus, 1), (Espece::ClMoins, 2)],
    },
    ReactifCatalogue {
        nom: "Lead(II) Nitrate",
        formule: "Pb(NO3)2 (aq)",
        categorie: CategorieCatalogue::Metaux,
        description_vsepr: "Hemidirected Pyramidal Pb2+ (Golden Rain)",
        ions: &[(Espece::Pb2Plus, 1)],
    },
    ReactifCatalogue {
        nom: "Barium Chloride",
        formule: "BaCl2 (aq)",
        categorie: CategorieCatalogue::Sels,
        description_vsepr: "Octahedral Ba2+ & Chloride Cl-",
        ions: &[(Espece::Ba2Plus, 1), (Espece::ClMoins, 2)],
    },
    ReactifCatalogue {
        nom: "Potassium Iodide",
        formule: "KI (aq)",
        categorie: CategorieCatalogue::Sels,
        description_vsepr: "Halide I- (Reacts with Pb2+ or Ag+)",
        ions: &[(Espece::IMoins, 1)],
    },
    ReactifCatalogue {
        nom: "Sodium Chloride",
        formule: "NaCl (aq)",
        categorie: CategorieCatalogue::Sels,
        description_vsepr: "Halide Cl- (Precipitates Ag+ as AgCl)",
        ions: &[(Espece::ClMoins, 1)],
    },
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IndicateurPH {
    BromothymolBlue,
    Phenolphthalein,
    MethylOrange,
    None,
}

impl IndicateurPH {
    pub fn label(&self) -> &'static str {
        match self {
            IndicateurPH::BromothymolBlue => "Bromothymol Blue (pH 6.0 - 7.6)",
            IndicateurPH::Phenolphthalein => "Phenolphthalein (pH 8.2 - 10.0)",
            IndicateurPH::MethylOrange => "Methyl Orange (pH 3.1 - 4.4)",
            IndicateurPH::None => "None (Natural Solution Tint)",
        }
    }

    pub fn couleur_selon_ph(&self, ph: f64) -> Option<Color32> {
        match self {
            IndicateurPH::None => None,
            IndicateurPH::BromothymolBlue => {
                if ph < 6.0 {
                    let t = (ph / 6.0).clamp(0.0, 1.0) as f32;
                    Some(Color32::from_rgb(255, (180.0 + 45.0 * t) as u8, 35))
                } else if ph <= 7.6 {
                    let t = ((ph - 6.0) / 1.6) as f32;
                    let r = (255.0 * (1.0 - t) + 30.0 * t) as u8;
                    let g = (225.0 * (1.0 - t) + 185.0 * t) as u8;
                    let b = (35.0 * (1.0 - t) + 140.0 * t) as u8;
                    Some(Color32::from_rgb(r, g, b))
                } else {
                    Some(Color32::from_rgb(35, 125, 255))
                }
            }
            IndicateurPH::Phenolphthalein => {
                if ph < 8.2 {
                    Some(Color32::from_rgb(200, 225, 245))
                } else {
                    let t = ((ph - 8.2) / 1.8).clamp(0.0, 1.0) as f32;
                    let r = (200.0 * (1.0 - t) + 255.0 * t) as u8;
                    let g = (225.0 * (1.0 - t) + 40.0 * t) as u8;
                    let b = (245.0 * (1.0 - t) + 170.0 * t) as u8;
                    Some(Color32::from_rgb(r, g, b))
                }
            }
            IndicateurPH::MethylOrange => {
                if ph < 3.1 {
                    Some(Color32::from_rgb(255, 55, 55))
                } else if ph <= 4.4 {
                    Some(Color32::from_rgb(255, 145, 35))
                } else {
                    Some(Color32::from_rgb(255, 225, 50))
                }
            }
        }
    }
}

pub struct ReactionResultat {
    pub produit_a: Option<Espece>,
    pub produit_b: Option<Espece>,
    pub energie_activation: f32,
    pub facteur_pre_exp: f32,
    pub delta_h_kj_mol: f64,
}

pub fn evaluer_reaction(a: Espece, b: Espece) -> Option<ReactionResultat> {
    use Espece::*;
    match (a, b) {
        (HPlus, OHMoins) | (OHMoins, HPlus) => Some(ReactionResultat {
            produit_a: None,
            produit_b: None,
            energie_activation: 8_000.0,
            facteur_pre_exp: 35.0,
            delta_h_kj_mol: -57.3,
        }),
        (HPlus, NH3Aq) | (NH3Aq, HPlus) => Some(ReactionResultat {
            produit_a: None,
            produit_b: None,
            energie_activation: 9_000.0,
            facteur_pre_exp: 30.0,
            delta_h_kj_mol: -52.2,
        }),
        (Cu2Plus, OHMoins) | (OHMoins, Cu2Plus) => Some(ReactionResultat {
            produit_a: Some(CuOH2Solide),
            produit_b: None,
            energie_activation: 12_000.0,
            facteur_pre_exp: 95.0,
            delta_h_kj_mol: -65.0,
        }),
        (Cu2Plus, NH3Aq) | (NH3Aq, Cu2Plus) => Some(ReactionResultat {
            produit_a: Some(CuNH34Complexe),
            produit_b: None,
            energie_activation: 11_500.0,
            facteur_pre_exp: 90.0,
            delta_h_kj_mol: -88.0,
        }),
        (CuOH2Solide, NH3Aq) | (NH3Aq, CuOH2Solide) => Some(ReactionResultat {
            produit_a: Some(CuNH34Complexe),
            produit_b: Some(OHMoins),
            energie_activation: 13_000.0,
            facteur_pre_exp: 100.0,
            delta_h_kj_mol: -45.0,
        }),
        (Cu2Plus, CO3DeuxMoins) | (CO3DeuxMoins, Cu2Plus) => Some(ReactionResultat {
            produit_a: Some(CuCO3Solide),
            produit_b: None,
            energie_activation: 12_500.0,
            facteur_pre_exp: 95.0,
            delta_h_kj_mol: -42.0,
        }),
        (AgPlus, ClMoins) | (ClMoins, AgPlus) => Some(ReactionResultat {
            produit_a: Some(AgClSolide),
            produit_b: None,
            energie_activation: 9_500.0,
            facteur_pre_exp: 85.0,
            delta_h_kj_mol: -65.5,
        }),
        (AgClSolide, NH3Aq) | (NH3Aq, AgClSolide) => Some(ReactionResultat {
            produit_a: Some(AgPlus),
            produit_b: Some(ClMoins),
            energie_activation: 13_500.0,
            facteur_pre_exp: 95.0,
            delta_h_kj_mol: -38.0,
        }),
        (Pb2Plus, IMoins) | (IMoins, Pb2Plus) => Some(ReactionResultat {
            produit_a: Some(PbI2Solide),
            produit_b: None,
            energie_activation: 10_500.0,
            facteur_pre_exp: 95.0,
            delta_h_kj_mol: -78.0,
        }),
        (Fe3Plus, OHMoins) | (OHMoins, Fe3Plus) => Some(ReactionResultat {
            produit_a: Some(FeOH3Solide),
            produit_b: None,
            energie_activation: 11_500.0,
            facteur_pre_exp: 95.0,
            delta_h_kj_mol: -72.0,
        }),
        (Ba2Plus, SO4DeuxMoins) | (SO4DeuxMoins, Ba2Plus) => Some(ReactionResultat {
            produit_a: Some(BaSO4Solide),
            produit_b: None,
            energie_activation: 10_000.0,
            facteur_pre_exp: 90.0,
            delta_h_kj_mol: -26.0,
        }),
        (HPlus, CO3DeuxMoins) | (CO3DeuxMoins, HPlus) => Some(ReactionResultat {
            produit_a: Some(CO2Gaz),
            produit_b: None,
            energie_activation: 11_000.0,
            facteur_pre_exp: 70.0,
            delta_h_kj_mol: -25.0,
        }),
        (CuOH2Solide, HPlus) | (HPlus, CuOH2Solide) => Some(ReactionResultat {
            produit_a: Some(Cu2Plus),
            produit_b: None,
            energie_activation: 14_000.0,
            facteur_pre_exp: 110.0,
            delta_h_kj_mol: -32.0,
        }),
        (FeOH3Solide, HPlus) | (HPlus, FeOH3Solide) => Some(ReactionResultat {
            produit_a: Some(Fe3Plus),
            produit_b: None,
            energie_activation: 14_000.0,
            facteur_pre_exp: 110.0,
            delta_h_kj_mol: -35.0,
        }),
        (CuCO3Solide, HPlus) | (HPlus, CuCO3Solide) => Some(ReactionResultat {
            produit_a: Some(Cu2Plus),
            produit_b: Some(CO2Gaz),
            energie_activation: 13_500.0,
            facteur_pre_exp: 105.0,
            delta_h_kj_mol: -28.0,
        }),
        _ => None,
    }
}