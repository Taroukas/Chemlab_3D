use rand::Rng;
use crate::chimie::{evaluer_reaction, Espece, Phase, CATALOGUE_REACTIFS};

#[derive(Clone, Copy, Default, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn add(self, o: Self) -> Self {
        Self::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }

    pub fn sub(self, o: Self) -> Self {
        Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }

    pub fn scale(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn dot(self, o: Self) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn length_sq(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(self) -> Self {
        let l = self.length_sq().sqrt().max(1e-6);
        self.scale(1.0 / l)
    }

    pub fn rotate_euler_3d(self, rot: [f32; 3]) -> Self {
        let (sy, cy) = rot[0].sin_cos();
        let (sp, cp) = rot[1].sin_cos();
        let (sr, cr) = rot[2].sin_cos();

        let x0 = self.x * cr - self.y * sr;
        let y0 = self.x * sr + self.y * cr;
        let z0 = self.z;

        let y1 = y0 * cp - z0 * sp;
        let z1 = y0 * sp + z0 * cp;

        let x2 = x0 * cy + z1 * sy;
        let z2 = -x0 * sy + z1 * cy;
        Self::new(x2, y1, z2)
    }
}

pub struct Particule {
    pub pos: Vec3,
    pub vel: Vec3,
    pub rot: [f32; 3],
    pub rot_vel: [f32; 3],
    pub espece: Espece,
}

pub struct GrilleSpatiale3D {
    pub dim: usize,
    pub cellules: Vec<Vec<usize>>,
}

impl GrilleSpatiale3D {
    pub fn new(dim: usize) -> Self {
        Self {
            dim,
            cellules: vec![Vec::with_capacity(12); dim * dim * dim],
        }
    }

    pub fn vider(&mut self) {
        for cell in &mut self.cellules {
            cell.clear();
        }
    }

    pub fn inserer(&mut self, index: usize, pos: Vec3) {
        let d = self.dim as f32;
        let nx = ((pos.x + 1.0) * 0.5 * d) as usize;
        let ny = ((pos.y + 1.0) * 0.5 * d) as usize;
        let nz = ((pos.z + 1.0) * 0.5 * d) as usize;
        let cx = nx.clamp(0, self.dim - 1);
        let cy = ny.clamp(0, self.dim - 1);
        let cz = nz.clamp(0, self.dim - 1);
        self.cellules[(cz * self.dim + cy) * self.dim + cx].push(index);
    }
}

pub struct MoteurPhysique {
    pub particules: Vec<Particule>,
    pub grille: GrilleSpatiale3D,
    pub rpm_agitateur: f32,
    pub angle_barreau: f32,
    pub burette_active: bool,
    pub index_catalogue_burette: usize,
    pub timer_goutte: f32,
    pub volume_ml: f32,
}

impl Default for MoteurPhysique {
    fn default() -> Self {
        Self {
            particules: Vec::with_capacity(2000),
            grille: GrilleSpatiale3D::new(14),
            rpm_agitateur: 420.0,
            angle_barreau: 0.0,
            burette_active: false,
            index_catalogue_burette: 2,
            timer_goutte: 0.0,
            volume_ml: 150.0,
        }
    }
}

impl MoteurPhysique {
    pub fn y_surface_liquide(&self) -> f32 {
        let y_fond = -0.82_f32;
        let y_250ml = 0.48_f32;
        let frac = (self.volume_ml / 250.0).clamp(0.35, 1.08);
        y_fond + frac * (y_250ml - y_fond)
    }

    pub fn verser_espece(&mut self, espece: Espece, quantite: usize, goutte_centree: bool) {
        let mut rng = rand::thread_rng();
        let y_surf = self.y_surface_liquide();
        for _ in 0..quantite {
            let (r_min, r_max, y_min, y_max) = if goutte_centree {
                (0.02, 0.22, y_surf - 0.12, y_surf - 0.02)
            } else {
                (0.08, 0.74, y_surf - 0.35, y_surf - 0.04)
            };
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let u: f32 = rng.gen_range(0.0..1.0);
            let rayon = (r_min * r_min + u * (r_max * r_max - r_min * r_min)).sqrt();
            self.particules.push(Particule {
                pos: Vec3::new(rayon * angle.cos(), rng.gen_range(y_min..y_max), rayon * angle.sin()),
                vel: Vec3::new(
                    rng.gen_range(-0.35..0.35),
                    rng.gen_range(-0.55..-0.15),
                    rng.gen_range(-0.35..0.35),
                ),
                rot: [rng.gen_range(0.0..6.28), rng.gen_range(0.0..6.28), rng.gen_range(0.0..6.28)],
                rot_vel: [rng.gen_range(-2.4..2.4), rng.gen_range(-2.4..2.4), rng.gen_range(-2.4..2.4)],
                espece,
            });
        }
    }

    pub fn verser_reactif_catalogue(&mut self, index_cat: usize, facteur_dose: usize, goutte_centree: bool) {
        if let Some(reac) = CATALOGUE_REACTIFS.get(index_cat) {
            let ajout_ml = if goutte_centree { 0.8 } else { (facteur_dose as f32) * 0.45 };
            self.volume_ml = (self.volume_ml + ajout_ml).min(260.0);
            for &(esp, stoech) in reac.ions {
                self.verser_espece(esp, stoech * facteur_dose, goutte_centree);
            }
        }
    }

    pub fn compter_especes(&self) -> [usize; 20] {
        let mut c = [0usize; 20];
        for p in &self.particules {
            c[p.espece.index()] += 1;
        }
        c
    }

    pub fn mettre_a_jour(&mut self, dt: f32, temperature_c: f64, mmol_par_particule: f64) -> f64 {
        let mut rng = rand::thread_rng();

        if self.burette_active {
            self.timer_goutte += dt;
            if self.timer_goutte >= 0.18 {
                self.timer_goutte = 0.0;
                self.verser_reactif_catalogue(self.index_catalogue_burette, 2, true);
            }
        }

        let vitesse_angulaire = (self.rpm_agitateur / 60.0) * std::f32::consts::TAU;
        self.angle_barreau = (self.angle_barreau + vitesse_angulaire * dt) % std::f32::consts::TAU;
        let intensite_vortex = (self.rpm_agitateur / 600.0).clamp(0.0, 2.0);
        let agitation_thermique = ((temperature_c as f32 + 273.15) / 298.15).sqrt();

        let rayon_cylindre = 0.78_f32;
        let y_fond = -0.78_f32;
        let y_surface = self.y_surface_liquide();

        for p in &mut self.particules {
            for k in 0..3 {
                p.rot[k] = (p.rot[k] + p.rot_vel[k] * agitation_thermique * dt) % std::f32::consts::TAU;
            }

            let r_xz = (p.pos.x * p.pos.x + p.pos.z * p.pos.z).sqrt().max(0.001);
            let nx = p.pos.x / r_xz;
            let nz = p.pos.z / r_xz;

            let force_radiale = if r_xz < 0.22 {
                0.55 * (0.22 - r_xz)
            } else if r_xz > 0.65 {
                -1.10 * (r_xz - 0.65)
            } else {
                0.0
            };

            let f_vortex_x = (-nz * 0.65 * r_xz.max(0.2) + nx * force_radiale) * intensite_vortex;
            let f_vortex_z = (nx * 0.65 * r_xz.max(0.2) + nz * force_radiale) * intensite_vortex;
            let f_vortex_y = (0.42 - r_xz) * intensite_vortex * 0.45;

            match p.espece.phase() {
                Phase::Aqueuse => {
                    p.vel.x += (rng.gen_range(-2.2..2.2) * agitation_thermique + f_vortex_x) * dt;
                    p.vel.y += (rng.gen_range(-2.0..2.0) * agitation_thermique + f_vortex_y - 0.05) * dt;
                    p.vel.z += (rng.gen_range(-2.2..2.2) * agitation_thermique + f_vortex_z) * dt;
                    p.vel = p.vel.scale(0.94);
                }
                Phase::Solide => {
                    let remise_en_suspension = if p.pos.y < -0.58 && r_xz < 0.50 {
                        intensite_vortex * 1.10
                    } else {
                        0.0
                    };
                    p.vel.y += (-1.05 + remise_en_suspension) * dt;
                    p.vel.x += (f_vortex_x * 0.65 + rng.gen_range(-0.5..0.5)) * dt;
                    p.vel.z += (f_vortex_z * 0.65 + rng.gen_range(-0.5..0.5)) * dt;
                    p.vel = p.vel.scale(0.90);
                }
                Phase::Gazeuse => {
                    p.vel.y += 1.85 * dt;
                    p.vel.x += (f_vortex_x * 0.4 + rng.gen_range(-0.9..0.9)) * dt;
                    p.vel.z += (f_vortex_z * 0.4 + rng.gen_range(-0.9..0.9)) * dt;
                    p.vel = p.vel.scale(0.95);
                }
            }

            p.pos = p.pos.add(p.vel.scale(dt));

            let r_nouveau = (p.pos.x * p.pos.x + p.pos.z * p.pos.z).sqrt();
            if r_nouveau > rayon_cylindre {
                let wnx = p.pos.x / r_nouveau;
                let wnz = p.pos.z / r_nouveau;
                p.pos.x = wnx * (rayon_cylindre - 0.01);
                p.pos.z = wnz * (rayon_cylindre - 0.01);
                let v_dot_n = p.vel.x * wnx + p.vel.z * wnz;
                if v_dot_n > 0.0 {
                    p.vel.x -= 1.7 * v_dot_n * wnx;
                    p.vel.z -= 1.7 * v_dot_n * wnz;
                }
            }

            if p.pos.y < y_fond {
                p.pos.y = y_fond;
                p.vel.y = p.vel.y.abs() * 0.40;
            }
            if p.pos.y > y_surface - 0.02 && p.espece.phase() != Phase::Gazeuse {
                p.pos.y = y_surface - 0.02;
                p.vel.y = -p.vel.y.abs() * 0.55;
            }
        }

        self.particules.retain(|p| !(p.espece == Espece::CO2Gaz && p.pos.y > y_surface + 0.03));

        self.grille.vider();
        for (idx, p) in self.particules.iter().enumerate() {
            self.grille.inserer(idx, p.pos);
        }

        let n = self.particules.len();
        let mut a_reagi = vec![false; n];
        let mut a_supprimer = vec![false; n];
        let mut chaleur_totale_joules = 0.0;

        let rayon_choc_sq = 0.145 * 0.145;
        let r_gaz_parfaits = 8.314_f32;
        let temp_kelvin = (temperature_c as f32 + 273.15).max(273.15);
        let d = self.grille.dim;

        for cz in 0..d {
            for cy in 0..d {
                for cx in 0..d {
                    let cell_idx = (cz * d + cy) * d + cx;
                    let len_courante = self.grille.cellules[cell_idx].len();
                    if len_courante == 0 {
                        continue;
                    }

                    for a_idx in 0..len_courante {
                        let i = self.grille.cellules[cell_idx][a_idx];
                        if a_reagi[i] {
                            continue;
                        }

                        for dz in 0..=1 {
                            for dy in 0..=1 {
                                for dx in 0..=1 {
                                    let vx = (cx + dx).min(d - 1);
                                    let vy = (cy + dy).min(d - 1);
                                    let vz = (cz + dz).min(d - 1);
                                    if dx > 0 && vx == cx || dy > 0 && vy == cy || dz > 0 && vz == cz {
                                        continue;
                                    }
                                    let v_idx = (vz * d + vy) * d + vx;
                                    let len_voisin = self.grille.cellules[v_idx].len();

                                    for b_idx in 0..len_voisin {
                                        let j = self.grille.cellules[v_idx][b_idx];
                                        if i >= j || a_reagi[j] {
                                            continue;
                                        }

                                        if self.particules[i].pos.sub(self.particules[j].pos).length_sq() < rayon_choc_sq {
                                            let esp_i = self.particules[i].espece;
                                            let esp_j = self.particules[j].espece;

                                            if let Some(reac) = evaluer_reaction(esp_i, esp_j) {
                                                let prob = reac.facteur_pre_exp
                                                    * (-reac.energie_activation / (r_gaz_parfaits * temp_kelvin)).exp();

                                                if rng.gen::<f32>() < prob {
                                                    a_reagi[i] = true;
                                                    a_reagi[j] = true;

                                                    match reac.produit_a {
                                                        Some(prod) => self.particules[i].espece = prod,
                                                        None => a_supprimer[i] = true,
                                                    }
                                                    match reac.produit_b {
                                                        Some(prod) => self.particules[j].espece = prod,
                                                        None => a_supprimer[j] = true,
                                                    }

                                                    chaleur_totale_joules += -reac.delta_h_kj_mol * mmol_par_particule;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    if a_reagi[i] {
                                        break;
                                    }
                                }
                                if a_reagi[i] {
                                    break;
                                }
                            }
                            if a_reagi[i] {
                                break;
                            }
                        }
                    }
                }
            }
        }

        let mut idx = 0;
        self.particules.retain(|_| {
            let garder = !a_supprimer[idx];
            idx += 1;
            garder
        });

        chaleur_totale_joules
    }
}