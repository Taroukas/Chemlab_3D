mod chimie;
mod physique;
mod solveur;

use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
use eframe::egui;
use egui::epaint::{Mesh, Vertex};
use egui::{Align2, Color32, FontId, Margin, Pos2, RichText, Rounding, Stroke, TextureId, Vec2};
use egui_plot::{Legend, Line, Plot, PlotPoints};

use chimie::{CategorieCatalogue, Espece, IndicateurPH, CATALOGUE_REACTIFS};
use physique::{MoteurPhysique, Vec3};
use solveur::EtatThermochimique;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1420.0, 840.0])
            .with_min_inner_size([1300.0, 750.0])
            .with_title("ChemLab Studio 3D — True 3D Molecular Meshes & Chemical Catalog"),
        ..Default::default()
    };
    eframe::run_native(
        "ChemLab Studio 3D",
        options,
        Box::new(|cc| {
            configurer_theme_moderne(&cc.egui_ctx);
            Box::new(LaboratoireChimie::default())
        }),
    )
}

fn configurer_theme_moderne(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = Vec2::new(7.0, 5.0);
    style.spacing.button_padding = Vec2::new(9.0, 5.0);

    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = Color32::from_rgb(14, 18, 26);
    visuals.window_fill = Color32::from_rgb(20, 25, 36);
    visuals.extreme_bg_color = Color32::from_rgb(9, 12, 18);

    let arrondi = Rounding::same(6.0);
    visuals.widgets.noninteractive.rounding = arrondi;
    visuals.widgets.inactive.rounding = arrondi;
    visuals.widgets.hovered.rounding = arrondi;
    visuals.widgets.active.rounding = arrondi;

    visuals.widgets.inactive.bg_fill = Color32::from_rgb(28, 36, 52);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(44, 58, 84);
    visuals.widgets.active.bg_fill = Color32::from_rgb(55, 95, 165);
    visuals.selection.bg_fill = Color32::from_rgb(45, 115, 210);

    style.visuals = visuals;
    ctx.set_style(style);
}

#[derive(Clone, Copy)]
struct Camera3D {
    yaw: f32,
    pitch: f32,
    zoom: f32,
    pan: Vec2,
    auto_rotate: bool,
}

impl Default for Camera3D {
    fn default() -> Self {
        Self {
            yaw: 0.50,
            pitch: 0.34,
            zoom: 1.0,
            pan: Vec2::ZERO,
            auto_rotate: false,
        }
    }
}

impl Camera3D {
    fn vers_espace_camera(&self, p: Vec3) -> Vec3 {
        let (sy, cy) = self.yaw.sin_cos();
        let (sp, cp) = self.pitch.sin_cos();

        let x1 = p.x * cy + p.z * sy;
        let z1 = -p.x * sy + p.z * cy;

        let y2 = p.y * cp - z1 * sp;
        let z2 = p.y * sp + z1 * cp;
        Vec3::new(x1, y2, z2)
    }

    fn projeter_cam(&self, p_cam: Vec3, centre_ecran: Pos2, echelle_base: f32) -> Pos2 {
        let distance_cam = 3.6_f32;
        let z_profondeur = (distance_cam - p_cam.z).max(0.35);
        let perspective = (distance_cam / z_profondeur) * self.zoom;

        let sx = centre_ecran.x + self.pan.x + p_cam.x * echelle_base * perspective;
        let sy_screen = centre_ecran.y + self.pan.y - p_cam.y * echelle_base * perspective;
        Pos2::new(sx, sy_screen)
    }

    fn projeter(&self, p: Vec3, centre_ecran: Pos2, echelle_base: f32) -> (Pos2, f32) {
        let p_cam = self.vers_espace_camera(p);
        (self.projeter_cam(p_cam, centre_ecran, echelle_base), p_cam.z)
    }
}

enum Element3D {
    Atome { centre_cam: Vec3, rayon_monde: f32, couleur: Color32 },
    Liaison { a_cam: Vec3, b_cam: Vec3, rayon_monde: f32, couleur_a: Color32, couleur_b: Color32 },
}

impl Element3D {
    fn z_cle(&self) -> f32 {
        match self {
            Element3D::Atome { centre_cam, .. } => centre_cam.z,
            Element3D::Liaison { a_cam, b_cam, .. } => (a_cam.z + b_cam.z) * 0.5,
        }
    }
}

fn eclairer_phong_3d(col: Color32, n: Vec3) -> Color32 {
    let lumiere = Vec3::new(-0.38, 0.58, 0.72).normalize();
    let ndotl = n.dot(lumiere).max(0.0);

    let vue = Vec3::new(0.0, 0.0, 1.0);
    let h = lumiere.add(vue).normalize();
    let ndoth = n.dot(h).max(0.0);

    let ambiant = 0.32;
    let diffus = 0.68 * ndotl;
    let speculaire = ndoth.powi(18) * 145.0;
    let fresnel = 0.85 + 0.15 * n.z.max(0.0);

    let facteur = (ambiant + diffus) * fresnel;
    let r = (col.r() as f32 * facteur + speculaire).clamp(0.0, 255.0) as u8;
    let g = (col.g() as f32 * facteur + speculaire).clamp(0.0, 255.0) as u8;
    let b = (col.b() as f32 * facteur + speculaire).clamp(0.0, 255.0) as u8;
    Color32::from_rgba_unmultiplied(r, g, b, col.a())
}

fn generer_sphere_lisse_3d(
    mesh: &mut Mesh,
    cam: &Camera3D,
    centre_ecran: Pos2,
    echelle_base: f32,
    centre_cam: Vec3,
    rayon_monde: f32,
    couleur: Color32,
) {
    const ANNEAUX: usize = 4;
    const SEGMENTS: usize = 14;

    let idx_pole = mesh.vertices.len() as u32;
    let n_pole = Vec3::new(0.0, 0.0, 1.0);
    let p_pole = cam.projeter_cam(centre_cam.add(n_pole.scale(rayon_monde)), centre_ecran, echelle_base);
    mesh.vertices.push(Vertex {
        pos: p_pole,
        uv: egui::epaint::WHITE_UV,
        color: eclairer_phong_3d(couleur, n_pole),
    });

    for r in 1..=ANNEAUX {
        let theta = (r as f32 / ANNEAUX as f32) * std::f32::consts::FRAC_PI_2;
        let sin_t = theta.sin();
        let cos_t = theta.cos();

        for s in 0..SEGMENTS {
            let phi = (s as f32 / SEGMENTS as f32) * std::f32::consts::TAU;
            let nx = sin_t * phi.cos();
            let ny = sin_t * phi.sin();
            let nz = cos_t;
            let normale = Vec3::new(nx, ny, nz);
            let p_cam = centre_cam.add(normale.scale(rayon_monde));
            let pos_2d = cam.projeter_cam(p_cam, centre_ecran, echelle_base);

            mesh.vertices.push(Vertex {
                pos: pos_2d,
                uv: egui::epaint::WHITE_UV,
                color: eclairer_phong_3d(couleur, normale),
            });
        }
    }

    let segs = SEGMENTS as u32;
    for s in 0..segs {
        let s_next = (s + 1) % segs;
        mesh.indices.extend_from_slice(&[idx_pole, idx_pole + 1 + s, idx_pole + 1 + s_next]);
    }

    for r in 1..(ANNEAUX as u32) {
        let anneau_prec = idx_pole + 1 + (r - 1) * segs;
        let anneau_cour = idx_pole + 1 + r * segs;
        for s in 0..segs {
            let s_next = (s + 1) % segs;
            let a = anneau_prec + s;
            let b = anneau_prec + s_next;
            let c = anneau_cour + s_next;
            let d = anneau_cour + s;
            mesh.indices.extend_from_slice(&[a, d, c, a, c, b]);
        }
    }
}

fn generer_cylindre_liaison_3d(
    mesh: &mut Mesh,
    cam: &Camera3D,
    centre_ecran: Pos2,
    echelle_base: f32,
    a_cam: Vec3,
    b_cam: Vec3,
    rayon_monde: f32,
    col_a: Color32,
    col_b: Color32,
) {
    let axe = b_cam.sub(a_cam);
    if axe.length_sq() < 1e-8 {
        return;
    }

    let perp = Vec3::new(-axe.y, axe.x, 0.0).normalize();
    let milieu_cam = a_cam.add(b_cam).scale(0.5);

    const TRANCHES: usize = 5;
    let base_idx = mesh.vertices.len() as u32;

    let col_neutre = Color32::from_rgb(205, 215, 230);
    let melanger = |c1: Color32, c2: Color32, t: f32| -> Color32 {
        Color32::from_rgb(
            (c1.r() as f32 * (1.0 - t) + c2.r() as f32 * t) as u8,
            (c1.g() as f32 * (1.0 - t) + c2.g() as f32 * t) as u8,
            (c1.b() as f32 * (1.0 - t) + c2.b() as f32 * t) as u8,
        )
    };
    let teinte_a = melanger(col_a, col_neutre, 0.55);
    let teinte_b = melanger(col_b, col_neutre, 0.55);

    for i in 0..=TRANCHES {
        let frac = (i as f32 / TRANCHES as f32) * 2.0 - 1.0;
        let nz = (1.0 - frac * frac).max(0.0).sqrt();
        let normale = perp.scale(frac).add(Vec3::new(0.0, 0.0, nz)).normalize();
        let offset = perp.scale(frac * rayon_monde).add(Vec3::new(0.0, 0.0, nz * rayon_monde));

        let pa = cam.projeter_cam(a_cam.add(offset), centre_ecran, echelle_base);
        let pm = cam.projeter_cam(milieu_cam.add(offset), centre_ecran, echelle_base);
        let pb = cam.projeter_cam(b_cam.add(offset), centre_ecran, echelle_base);

        mesh.vertices.push(Vertex { pos: pa, uv: egui::epaint::WHITE_UV, color: eclairer_phong_3d(teinte_a, normale) });
        mesh.vertices.push(Vertex { pos: pm, uv: egui::epaint::WHITE_UV, color: eclairer_phong_3d(col_neutre, normale) });
        mesh.vertices.push(Vertex { pos: pb, uv: egui::epaint::WHITE_UV, color: eclairer_phong_3d(teinte_b, normale) });
    }

    for i in 0..(TRANCHES as u32) {
        let r0 = base_idx + i * 3;
        let r1 = base_idx + (i + 1) * 3;
        mesh.indices.extend_from_slice(&[r0, r0 + 1, r1 + 1, r0, r1 + 1, r1]);
        mesh.indices.extend_from_slice(&[r0 + 1, r0 + 2, r1 + 2, r0 + 1, r1 + 2, r1 + 1]);
    }
}

fn rendre_scene_moleculaire_3d(
    painter: &egui::Painter,
    cam: &Camera3D,
    centre_ecran: Pos2,
    echelle_base: f32,
    elements: &mut Vec<Element3D>,
) {
    elements.sort_by(|a, b| a.z_cle().partial_cmp(&b.z_cle()).unwrap_or(Ordering::Equal));

    let mut mesh = Mesh::with_texture(TextureId::default());
    mesh.vertices.reserve(elements.len() * 60);
    mesh.indices.reserve(elements.len() * 160);

    for elem in elements.iter() {
        match *elem {
            Element3D::Atome { centre_cam, rayon_monde, couleur } => {
                generer_sphere_lisse_3d(&mut mesh, cam, centre_ecran, echelle_base, centre_cam, rayon_monde, couleur);
            }
            Element3D::Liaison { a_cam, b_cam, rayon_monde, couleur_a, couleur_b } => {
                generer_cylindre_liaison_3d(&mut mesh, cam, centre_ecran, echelle_base, a_cam, b_cam, rayon_monde, couleur_a, couleur_b);
            }
        }
    }

    painter.add(egui::Shape::mesh(mesh));
}

struct LaboratoireChimie {
    physique: MoteurPhysique,
    thermo: EtatThermochimique,
    indicateur: IndicateurPH,
    camera: Camera3D,
    categorie_filtre: CategorieCatalogue,
    recherche_catalogue: String,
    index_reactif_selectionne: usize,
    dose_particules: usize,
    echelle_taille_molecules: f32,
    afficher_coque_polyedre: bool,
    en_pause: bool,
    zoom_interactif_graphes: bool,
    temps: f64,
    angle_apercu_3d: f32,
    statut_export: String,
    hist_especes: Vec<Vec<[f64; 2]>>,
    hist_ph: Vec<[f64; 2]>,
    hist_cond: Vec<[f64; 2]>,
    hist_temp: Vec<[f64; 2]>,
}

impl Default for LaboratoireChimie {
    fn default() -> Self {
        Self {
            physique: MoteurPhysique::default(),
            thermo: EtatThermochimique::default(),
            indicateur: IndicateurPH::BromothymolBlue,
            camera: Camera3D::default(),
            categorie_filtre: CategorieCatalogue::Tous,
            recherche_catalogue: String::new(),
            index_reactif_selectionne: 0,
            dose_particules: 45,
            echelle_taille_molecules: 1.0,
            afficher_coque_polyedre: true,
            en_pause: false,
            zoom_interactif_graphes: false,
            temps: 0.0,
            angle_apercu_3d: 0.0,
            statut_export: String::new(),
            hist_especes: vec![Vec::new(); 20],
            hist_ph: Vec::new(),
            hist_cond: Vec::new(),
            hist_temp: Vec::new(),
        }
    }
}

impl LaboratoireChimie {
    fn charger_preset(&mut self, id: usize) {
        let cam_actuelle = self.camera;
        *self = Self::default();
        self.camera = cam_actuelle;
        match id {
            1 => {
                self.indicateur = IndicateurPH::BromothymolBlue;
                self.physique.verser_reactif_catalogue(0, 50, false);
                self.physique.index_catalogue_burette = 2;
                self.physique.burette_active = true;
            }
            2 => {
                self.indicateur = IndicateurPH::None;
                self.physique.verser_reactif_catalogue(8, 45, false);
                self.physique.verser_reactif_catalogue(10, 65, false);
            }
            3 => {
                self.indicateur = IndicateurPH::None;
                self.physique.verser_reactif_catalogue(5, 45, false);
                self.physique.verser_reactif_catalogue(3, 55, false);
            }
            4 => {
                self.indicateur = IndicateurPH::BromothymolBlue;
                self.physique.verser_reactif_catalogue(5, 40, false);
                self.physique.verser_reactif_catalogue(4, 55, false);
                self.physique.index_catalogue_burette = 0;
                self.physique.burette_active = true;
            }
            _ => {}
        }
    }

    fn exporter_csv(&mut self) {
        let chemin = "chemlab_3d_results.csv";
        if let Ok(mut f) = File::create(chemin) {
            let _ = writeln!(f, "Time_s,pH,Temperature_C,Conductivity_mS_cm");
            for idx in 0..self.hist_ph.len() {
                let t = self.hist_ph[idx][0];
                let ph = self.hist_ph[idx][1];
                let temp = self.hist_temp.get(idx).map(|x| x[1]).unwrap_or(25.0);
                let cond = self.hist_cond.get(idx).map(|x| x[1]).unwrap_or(0.0);
                let _ = writeln!(f, "{:.3},{:.3},{:.3},{:.3}", t, ph, temp, cond);
            }
            self.statut_export = format!("Saved to {}", chemin);
        }
    }
}

fn carte_ui<R>(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> R) -> R {
    egui::Frame::none()
        .fill(Color32::from_rgb(22, 28, 40))
        .stroke(Stroke::new(1.0, Color32::from_rgb(42, 54, 76)))
        .rounding(Rounding::same(8.0))
        .inner_margin(Margin::same(10.0))
        .show(ui, add_contents)
        .inner
}

fn dessiner_anneau_3d(
    painter: &egui::Painter,
    cam: &Camera3D,
    centre: Pos2,
    echelle: f32,
    y: f32,
    rayon: f32,
    segments: usize,
    stroke_avant: Stroke,
    stroke_arriere: Stroke,
) {
    let mut pts = Vec::with_capacity(segments + 1);
    for i in 0..=segments {
        let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let p3d = Vec3::new(rayon * angle.cos(), y, rayon * angle.sin());
        pts.push(cam.projeter(p3d, centre, echelle));
    }
    for i in 0..segments {
        let (p0, z0) = pts[i];
        let (p1, z1) = pts[i + 1];
        let stroke = if (z0 + z1) * 0.5 >= 0.0 { stroke_avant } else { stroke_arriere };
        painter.line_segment([p0, p1], stroke);
    }
}

fn dessiner_cylindre_liquide_3d(
    painter: &egui::Painter,
    cam: &Camera3D,
    centre: Pos2,
    echelle: f32,
    y_fond: f32,
    y_surface: f32,
    rayon: f32,
    r: u8,
    g: u8,
    b: u8,
) {
    let segs = 36;
    let mut mesh = Mesh::with_texture(TextureId::default());

    let col_fond = Color32::from_rgba_unmultiplied((r as f32 * 0.75) as u8, (g as f32 * 0.75) as u8, (b as f32 * 0.75) as u8, 65);
    let col_corps = Color32::from_rgba_unmultiplied(r, g, b, 42);
    let col_bord = Color32::from_rgba_unmultiplied(r, g, b, 72);
    let col_surface = Color32::from_rgba_unmultiplied(r, g, b, 58);

    let (p_centre_top, _) = cam.projeter(Vec3::new(0.0, y_surface, 0.0), centre, echelle);
    let (p_centre_bot, _) = cam.projeter(Vec3::new(0.0, y_fond, 0.0), centre, echelle);

    let idx_centre_top = mesh.vertices.len() as u32;
    mesh.vertices.push(Vertex { pos: p_centre_top, uv: egui::epaint::WHITE_UV, color: col_surface });
    let idx_centre_bot = mesh.vertices.len() as u32;
    mesh.vertices.push(Vertex { pos: p_centre_bot, uv: egui::epaint::WHITE_UV, color: col_fond });

    for i in 0..=segs {
        let theta = (i as f32 / segs as f32) * std::f32::consts::TAU;
        let x = rayon * theta.cos();
        let z = rayon * theta.sin();
        let (p_top, z_cam) = cam.projeter(Vec3::new(x, y_surface, z), centre, echelle);
        let (p_bot, _) = cam.projeter(Vec3::new(x, y_fond, z), centre, echelle);

        let col_mur = if z_cam.abs() < rayon * 0.40 { col_bord } else { col_corps };
        mesh.vertices.push(Vertex { pos: p_top, uv: egui::epaint::WHITE_UV, color: col_mur });
        mesh.vertices.push(Vertex { pos: p_bot, uv: egui::epaint::WHITE_UV, color: col_fond });
    }

    for i in 0..segs as u32 {
        let top_i = 2 + i * 2;
        let bot_i = top_i + 1;
        let top_next = top_i + 2;
        let bot_next = top_i + 3;

        mesh.indices.extend_from_slice(&[idx_centre_bot, bot_i, bot_next]);
        mesh.indices.extend_from_slice(&[top_i, bot_i, bot_next, top_i, bot_next, top_next]);
        mesh.indices.extend_from_slice(&[idx_centre_top, top_i, top_next]);
    }

    painter.add(egui::Shape::mesh(mesh));
}

fn ajouter_molecule_vsepr_3d(
    elements: &mut Vec<Element3D>,
    cam: &Camera3D,
    pos_mol: Vec3,
    rot_mol: [f32; 3],
    espece: Espece,
    echelle_atome: f32,
    afficher_liaisons: bool,
) {
    let modele = espece.geometrie_vsepr();
    let off0 = Vec3::new(modele[0].offset[0], modele[0].offset[1], modele[0].offset[2])
        .scale(echelle_atome)
        .rotate_euler_3d(rot_mol);
    let monde0 = pos_mol.add(off0);
    let cam0 = cam.vers_espace_camera(monde0);
    let r0 = modele[0].rayon * echelle_atome;

    elements.push(Element3D::Atome {
        centre_cam: cam0,
        rayon_monde: r0,
        couleur: modele[0].couleur,
    });

    for atome in &modele[1..] {
        let off_i = Vec3::new(atome.offset[0], atome.offset[1], atome.offset[2])
            .scale(echelle_atome)
            .rotate_euler_3d(rot_mol);
        let monde_i = pos_mol.add(off_i);
        let cam_i = cam.vers_espace_camera(monde_i);
        let r_i = atome.rayon * echelle_atome;

        if afficher_liaisons {
            let vec_liaison = cam_i.sub(cam0);
            let dist = vec_liaison.length_sq().sqrt().max(1e-5);
            let dir = vec_liaison.scale(1.0 / dist);
            let debut = cam0.add(dir.scale(r0 * 0.72));
            let fin = cam_i.sub(dir.scale(r_i * 0.72));

            elements.push(Element3D::Liaison {
                a_cam: debut,
                b_cam: fin,
                rayon_monde: 0.0065 * echelle_atome,
                couleur_a: modele[0].couleur,
                couleur_b: atome.couleur,
            });
        }

        elements.push(Element3D::Atome {
            centre_cam: cam_i,
            rayon_monde: r_i,
            couleur: atome.couleur,
        });
    }
}

impl eframe::App for LaboratoireChimie {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dt = ctx.input(|i| i.stable_dt).min(0.04);
        self.angle_apercu_3d = (self.angle_apercu_3d + 1.15 * dt) % std::f32::consts::TAU;

        if self.camera.auto_rotate {
            self.camera.yaw = (self.camera.yaw + 0.35 * dt) % std::f32::consts::TAU;
        }

        if !self.en_pause {
            self.temps += dt as f64;
            let chaleur_j = self.physique.mettre_a_jour(
                dt,
                self.thermo.temperature_c,
                self.thermo.mmol_par_particule,
            );
            let compteurs = self.physique.compter_especes();
            self.thermo.mettre_a_jour_rk4(dt as f64, chaleur_j, &compteurs);

            if !self.physique.particules.is_empty() {
                let f = self.thermo.mmol_par_particule;
                for esp in Espece::TOUTES {
                    let idx = esp.index();
                    self.hist_especes[idx].push([self.temps, compteurs[idx] as f64 * f]);
                }
                self.hist_ph.push([self.temps, self.thermo.ph]);
                self.hist_cond.push([self.temps, self.thermo.conductivite_ms_cm]);
                self.hist_temp.push([self.temps, self.thermo.temperature_c]);
            }
        }

        egui::TopBottomPanel::top("top_bar")
            .exact_height(46.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.label(
                        RichText::new("CHEMLAB STUDIO 3D")
                            .size(16.0)
                            .strong()
                            .color(Color32::from_rgb(90, 195, 255)),
                    );
                    ui.separator();

                    ui.label(RichText::new("Quick 3D Presets:").color(Color32::from_rgb(170, 185, 205)));
                    if ui.button("1. Acid-Base Titration").clicked() {
                        self.charger_preset(1);
                    }
                    if ui.button("2. Golden Rain (PbI2)").clicked() {
                        self.charger_preset(2);
                    }
                    if ui.button("3. [Cu(NH3)4]2+ Complex").clicked() {
                        self.charger_preset(3);
                    }
                    if ui.button("4. CO2 Effervescence").clicked() {
                        self.charger_preset(4);
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Export CSV").clicked() {
                            self.exporter_csv();
                        }
                        if ui.button("Reset Reactor").clicked() {
                            *self = Self::default();
                        }
                        let txt_pause = if self.en_pause { "Resume" } else { "Pause" };
                        let col_pause = if self.en_pause {
                            Color32::from_rgb(40, 140, 80)
                        } else {
                            Color32::from_rgb(55, 70, 95)
                        };
                        if ui.add(egui::Button::new(RichText::new(txt_pause).strong()).fill(col_pause)).clicked() {
                            self.en_pause = !self.en_pause;
                        }
                        if !self.statut_export.is_empty() {
                            ui.colored_label(Color32::from_rgb(90, 230, 135), &self.statut_export);
                        }
                    });
                });
            });

        egui::SidePanel::left("left_catalog_v3")
            .resizable(false)
            .exact_width(355.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(4.0);

                    carte_ui(ui, |ui| {
                        ui.label(RichText::new("CHEMICAL REAGENT CATALOG (12 BOTTLES)").strong().size(13.0));
                        ui.add_space(3.0);

                        ui.horizontal(|ui| {
                            for (cat, lbl) in [
                                (CategorieCatalogue::Tous, "All"),
                                (CategorieCatalogue::Acides, "Acids"),
                                (CategorieCatalogue::Bases, "Bases"),
                                (CategorieCatalogue::Metaux, "Metals"),
                                (CategorieCatalogue::Sels, "Salts"),
                            ] {
                                ui.selectable_value(&mut self.categorie_filtre, cat, lbl);
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Search:");
                            ui.text_edit_singleline(&mut self.recherche_catalogue);
                        });

                        ui.add_space(4.0);

                        egui::ScrollArea::vertical()
                            .id_source("scroll_reagents")
                            .max_height(185.0)
                            .show(ui, |ui| {
                                let filtre_txt = self.recherche_catalogue.to_lowercase();
                                for (idx, r) in CATALOGUE_REACTIFS.iter().enumerate() {
                                    if self.categorie_filtre != CategorieCatalogue::Tous && r.categorie != self.categorie_filtre {
                                        continue;
                                    }
                                    if !filtre_txt.is_empty()
                                        && !r.nom.to_lowercase().contains(&filtre_txt)
                                        && !r.formule.to_lowercase().contains(&filtre_txt)
                                    {
                                        continue;
                                    }

                                    let esp_principale = r.ions[0].0;
                                    let col = esp_principale.couleur();
                                    let est_sel = self.index_reactif_selectionne == idx;

                                    let bg = if est_sel {
                                        Color32::from_rgba_unmultiplied(col.r() / 3, col.g() / 3, col.b() / 3, 240)
                                    } else {
                                        Color32::from_rgb(28, 35, 50)
                                    };

                                    egui::Frame::none()
                                        .fill(bg)
                                        .stroke(Stroke::new(if est_sel { 1.5 } else { 0.8 }, col))
                                        .rounding(Rounding::same(5.0))
                                        .inner_margin(Margin::symmetric(8.0, 5.0))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                if ui
                                                    .selectable_label(
                                                        est_sel,
                                                        RichText::new(format!("{}  [{}]", r.nom, r.formule)).strong(),
                                                    )
                                                    .clicked()
                                                {
                                                    self.index_reactif_selectionne = idx;
                                                }
                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    if ui.small_button("+ Pour").clicked() {
                                                        self.index_reactif_selectionne = idx;
                                                        self.physique.verser_reactif_catalogue(idx, self.dose_particules, false);
                                                    }
                                                });
                                            });
                                        });
                                }
                            });

                        ui.add_space(6.0);
                        let r_sel = &CATALOGUE_REACTIFS[self.index_reactif_selectionne];
                        let esp_apercu = r_sel.ions[0].0;

                        ui.horizontal(|ui| {
                            let (rect_prev, _) = ui.allocate_exact_size(Vec2::new(86.0, 78.0), egui::Sense::hover());
                            let painter_p = ui.painter_at(rect_prev);
                            painter_p.rect_filled(rect_prev, Rounding::same(6.0), Color32::from_rgb(11, 15, 22));
                            painter_p.rect_stroke(rect_prev, Rounding::same(6.0), Stroke::new(1.0, Color32::from_rgb(45, 60, 85)));

                            let cam_prev = Camera3D { yaw: 0.0, pitch: 0.25, zoom: 1.0, pan: Vec2::ZERO, auto_rotate: false };
                            let mut elems_prev = Vec::with_capacity(16);
                            let rot_p = [self.angle_apercu_3d, 0.45, 0.25];
                            ajouter_molecule_vsepr_3d(&mut elems_prev, &cam_prev, Vec3::new(0.0, 0.0, 0.0), rot_p, esp_apercu, 1.15, true);
                            rendre_scene_moleculaire_3d(&painter_p, &cam_prev, rect_prev.center(), 430.0, &mut elems_prev);

                            ui.vertical(|ui| {
                                ui.label(RichText::new(format!("{} ({})", r_sel.nom, r_sel.formule)).strong().color(esp_apercu.couleur()));
                                ui.label(RichText::new(esp_apercu.nom_geometrie()).size(11.5));
                                ui.label(RichText::new(r_sel.description_vsepr).size(11.0).color(Color32::from_rgb(160, 175, 195)));
                            });
                        });

                        ui.add(egui::Slider::new(&mut self.dose_particules, 15..=90).text("Dose Volume (mL eq.)"));
                        ui.horizontal(|ui| {
                            if ui
                                .add(
                                    egui::Button::new(RichText::new(format!("+ Pour {} into Beaker", r_sel.formule)).strong())
                                        .fill(Color32::from_rgb(40, 95, 170)),
                                )
                                .clicked()
                            {
                                self.physique.verser_reactif_catalogue(self.index_reactif_selectionne, self.dose_particules, false);
                            }
                            if ui.button("Load in Burette").clicked() {
                                self.physique.index_catalogue_burette = self.index_reactif_selectionne;
                                self.physique.burette_active = true;
                            }
                        });
                    });

                    ui.add_space(6.0);

                    carte_ui(ui, |ui| {
                        ui.label(RichText::new("BURETTE, INDICATOR & REACTOR").strong().size(13.0));
                        ui.checkbox(&mut self.physique.burette_active, "Continuous Burette Drip");
                        ui.horizontal(|ui| {
                            ui.label("Burette:");
                            egui::ComboBox::from_id_source("burette_cat")
                                .selected_text(CATALOGUE_REACTIFS[self.physique.index_catalogue_burette].formule)
                                .show_ui(ui, |ui| {
                                    for (i, r) in CATALOGUE_REACTIFS.iter().enumerate() {
                                        ui.selectable_value(&mut self.physique.index_catalogue_burette, i, format!("{} ({})", r.nom, r.formule));
                                    }
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("pH Dye:");
                            egui::ComboBox::from_id_source("ind_select")
                                .selected_text(self.indicateur.label())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.indicateur, IndicateurPH::BromothymolBlue, IndicateurPH::BromothymolBlue.label());
                                    ui.selectable_value(&mut self.indicateur, IndicateurPH::Phenolphthalein, IndicateurPH::Phenolphthalein.label());
                                    ui.selectable_value(&mut self.indicateur, IndicateurPH::MethylOrange, IndicateurPH::MethylOrange.label());
                                    ui.selectable_value(&mut self.indicateur, IndicateurPH::None, IndicateurPH::None.label());
                                });
                        });

                        ui.add(egui::Slider::new(&mut self.physique.rpm_agitateur, 0.0..=1200.0).text("3D Stirrer (RPM)"));
                        ui.add(egui::Slider::new(&mut self.thermo.puissance_thermostat, -350.0..=500.0).text("Thermostat (W)"));
                        ui.add(egui::Slider::new(&mut self.echelle_taille_molecules, 0.7..=1.8).text("3D Atom Scale"));
                    });

                    ui.add_space(6.0);

                    carte_ui(ui, |ui| {
                        ui.label(RichText::new("LIVE TELEMETRY & ACTIVE 3D SPECIES").strong().size(13.0));
                        let c = self.physique.compter_especes();
                        let f = self.thermo.mmol_par_particule;

                        let fraction_ph = (self.thermo.ph / 14.0).clamp(0.0, 1.0) as f32;
                        let col_ph = IndicateurPH::BromothymolBlue
                            .couleur_selon_ph(self.thermo.ph)
                            .unwrap_or(Color32::from_rgb(80, 200, 255));
                        ui.add(
                            egui::ProgressBar::new(fraction_ph)
                                .fill(col_ph)
                                .text(RichText::new(format!("pH Meter: {:.2}", self.thermo.ph)).strong().color(Color32::BLACK)),
                        );

                        ui.horizontal(|ui| {
                            ui.colored_label(Color32::from_rgb(255, 130, 90), format!("Temp: {:.1} °C", self.thermo.temperature_c));
                            ui.separator();
                            ui.colored_label(Color32::from_rgb(255, 210, 80), format!("Cond: {:.2} mS/cm", self.thermo.conductivite_ms_cm));
                            ui.separator();
                            ui.label(format!("Vol: {:.0} mL", self.physique.volume_ml));
                        });

                        ui.separator();
                        for esp in Espece::TOUTES {
                            let nb = c[esp.index()];
                            if nb > 0 {
                                ui.horizontal(|ui| {
                                    let (rect, _) = ui.allocate_exact_size(Vec2::new(10.0, 10.0), egui::Sense::hover());
                                    ui.painter().circle_filled(rect.center(), 4.5, esp.couleur());
                                    ui.label(
                                        RichText::new(format!("{:<18} {:>5.1} mmol  ({})", esp.symbole(), nb as f64 * f, esp.nom_geometrie()))
                                            .size(11.0),
                                    );
                                });
                            }
                        }
                    });
                });
            });

        // ==================== RIGHT PANEL : FORCED FLEXIBLE WIDTH TO PREVENT OVERLAP ====================
        egui::SidePanel::right("right_analytics_v4")
            .resizable(true)
            .default_width(430.0)
            .width_range(380.0..=500.0)
            .show(ctx, |ui| {
                egui::Frame::none()
                    .inner_margin(Margin { left: 10.0, right: 10.0, top: 6.0, bottom: 6.0 })
                    .show(ui, |ui| {
                        let largeur_graphe = (ui.available_width() - 6.0).max(300.0);
                        ui.set_max_width(largeur_graphe);

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("ANALYTICAL CURVES").strong().size(13.5));
                            ui.separator();
                            ui.checkbox(&mut self.zoom_interactif_graphes, "Manual Zoom");
                        });
                        ui.separator();

                        let hauteur_totale = ui.available_height();
                        let hauteur_graphe = ((hauteur_totale - 86.0) / 3.0).max(105.0);
                        let drag_actif = self.zoom_interactif_graphes;
                        let compteurs = self.physique.compter_especes();

                        ui.label(RichText::new("1. Chemical Kinetics — Quantities (mmol)").strong().size(11.5));
                        Plot::new("plot_kinetics_v4")
                            .legend(Legend::default())
                            .width(largeur_graphe)
                            .height(hauteur_graphe)
                            .allow_drag(drag_actif)
                            .allow_scroll(drag_actif)
                            .include_y(0.0)
                            .include_y(35.0)
                            .show(ui, |plot_ui| {
                                let ordre_especes = [
                                    Espece::ClMoins,
                                    Espece::SO4DeuxMoins,
                                    Espece::IMoins,
                                    Espece::Ba2Plus,
                                    Espece::AgPlus,
                                    Espece::Pb2Plus,
                                    Espece::Fe3Plus,
                                    Espece::CO3DeuxMoins,
                                    Espece::NH3Aq,
                                    Espece::Cu2Plus,
                                    Espece::OHMoins,
                                    Espece::HPlus,
                                    Espece::CuNH34Complexe,
                                    Espece::CuOH2Solide,
                                    Espece::CuCO3Solide,
                                    Espece::AgClSolide,
                                    Espece::PbI2Solide,
                                    Espece::FeOH3Solide,
                                    Espece::BaSO4Solide,
                                ];
                                for esp in ordre_especes {
                                    let idx = esp.index();
                                    let a_donnees = compteurs[idx] > 0 || self.hist_especes[idx].iter().any(|p| p[1] > 0.1);
                                    if a_donnees {
                                        let est_spectateur = matches!(esp, Espece::ClMoins | Espece::SO4DeuxMoins);
                                        let epaisseur = if est_spectateur { 1.4 } else { 2.6 };
                                        plot_ui.line(
                                            Line::new(PlotPoints::from(self.hist_especes[idx].clone()))
                                                .name(esp.symbole())
                                                .color(esp.couleur())
                                                .width(epaisseur),
                                        );
                                    }
                                }
                            });

                        ui.add_space(4.0);

                        ui.label(RichText::new("2. Titration — pH & Conductivity").strong().size(11.5));
                        Plot::new("plot_ph_cond_v4")
                            .legend(Legend::default())
                            .width(largeur_graphe)
                            .height(hauteur_graphe)
                            .allow_drag(drag_actif)
                            .allow_scroll(drag_actif)
                            .include_y(0.0)
                            .include_y(14.5)
                            .show(ui, |plot_ui| {
                                plot_ui.line(
                                    Line::new(PlotPoints::from(self.hist_ph.clone()))
                                        .name("pH (0 - 14)")
                                        .color(Color32::from_rgb(70, 220, 255))
                                        .width(2.5),
                                );
                                plot_ui.line(
                                    Line::new(PlotPoints::from(self.hist_cond.clone()))
                                        .name("Conductivity (mS/cm)")
                                        .color(Color32::from_rgb(255, 200, 60))
                                        .width(2.0),
                                );
                            });

                        ui.add_space(4.0);

                        ui.label(RichText::new("3. Thermodynamics — Temp (°C)").strong().size(11.5));
                        Plot::new("plot_temp_v4")
                            .legend(Legend::default())
                            .width(largeur_graphe)
                            .height(hauteur_graphe)
                            .allow_drag(drag_actif)
                            .allow_scroll(drag_actif)
                            .include_y(20.0)
                            .include_y(45.0)
                            .show(ui, |plot_ui| {
                                plot_ui.line(
                                    Line::new(PlotPoints::from(self.hist_temp.clone()))
                                        .name("Temperature (°C)")
                                        .color(Color32::from_rgb(255, 95, 75))
                                        .width(2.2),
                                );
                            });
                    });
            });

        // ==================== CENTER PANEL : STRICT CLIPPED 3D REACTOR ====================
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("3D INTERACTIVE REACTOR").strong().size(14.0));
                ui.checkbox(&mut self.afficher_coque_polyedre, "Show 3D VSEPR Bonds");
                ui.checkbox(&mut self.camera.auto_rotate, "Auto-Orbit 3D");
                if ui.button("Reset 3D Camera").clicked() {
                    self.camera = Camera3D::default();
                }
            });
            ui.separator();

            let (rect_dispo, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
            // Clip strict pour empêcher tout débordement graphique sur les panneaux adjacents
            let painter = ui.painter().with_clip_rect(rect_dispo);

            if response.dragged_by(egui::PointerButton::Primary) {
                let delta = response.drag_delta();
                self.camera.yaw += delta.x * 0.008;
                self.camera.pitch = (self.camera.pitch + delta.y * 0.008).clamp(-1.15, 1.25);
            }
            if response.dragged_by(egui::PointerButton::Secondary) {
                self.camera.pan += response.drag_delta();
            }
            if response.hovered() {
                let scroll = ctx.input(|i| i.raw_scroll_delta.y);
                if scroll.abs() > 0.1 {
                    self.camera.zoom = (self.camera.zoom * (1.0 + scroll * 0.0015)).clamp(0.45, 4.0);
                }
            }

            let centre_ecran = rect_dispo.center() + Vec2::new(0.0, 12.0);
            let echelle_base = rect_dispo.height().min(rect_dispo.width()) * 0.35;

            let r_becher = 0.82_f32;
            let y_haut = 0.78_f32;
            let y_250ml = 0.48_f32;
            let y_fond = -0.82_f32;
            let y_surface = self.physique.y_surface_liquide();

            let col_socle = if self.thermo.puissance_thermostat > 5.0 {
                Color32::from_rgba_unmultiplied(255, 90, 50, 140)
            } else if self.thermo.puissance_thermostat < -5.0 {
                Color32::from_rgba_unmultiplied(60, 175, 255, 140)
            } else {
                Color32::from_rgba_unmultiplied(80, 95, 120, 110)
            };
            dessiner_anneau_3d(
                &painter,
                &self.camera,
                centre_ecran,
                echelle_base,
                y_fond - 0.06,
                r_becher * 1.05,
                40,
                Stroke::new(3.5, col_socle),
                Stroke::new(2.0, col_socle),
            );

            let compteurs = self.physique.compter_especes();
            let couleur_base = self
                .indicateur
                .couleur_selon_ph(self.thermo.ph)
                .unwrap_or(Color32::from_rgb(130, 190, 235));
            let c_cu = (compteurs[Espece::Cu2Plus.index()] + compteurs[Espece::CuNH34Complexe.index()] * 2) as f32;
            let poids_cu = (c_cu / 120.0).clamp(0.0, 0.75);
            let r_liq = (couleur_base.r() as f32 * (1.0 - poids_cu) + 30.0 * poids_cu) as u8;
            let g_liq = (couleur_base.g() as f32 * (1.0 - poids_cu) + 125.0 * poids_cu) as u8;
            let b_liq = (couleur_base.b() as f32 * (1.0 - poids_cu) + 255.0 * poids_cu) as u8;

            dessiner_cylindre_liquide_3d(
                &painter,
                &self.camera,
                centre_ecran,
                echelle_base,
                y_fond,
                y_surface,
                r_becher * 0.99,
                r_liq,
                g_liq,
                b_liq,
            );

            let mut elements = Vec::with_capacity(self.physique.particules.len() * 10 + 4);

            let l_barreau = 0.22_f32;
            let r_barreau = 0.032_f32;
            let bx = l_barreau * self.physique.angle_barreau.cos();
            let bz = l_barreau * self.physique.angle_barreau.sin();
            let b_a = self.camera.vers_espace_camera(Vec3::new(-bx, y_fond + r_barreau, -bz));
            let b_b = self.camera.vers_espace_camera(Vec3::new(bx, y_fond + r_barreau, bz));
            let blanc_teflon = Color32::from_rgb(240, 245, 255);
            elements.push(Element3D::Liaison {
                a_cam: b_a,
                b_cam: b_b,
                rayon_monde: r_barreau,
                couleur_a: blanc_teflon,
                couleur_b: blanc_teflon,
            });
            elements.push(Element3D::Atome { centre_cam: b_a, rayon_monde: r_barreau, couleur: blanc_teflon });
            elements.push(Element3D::Atome { centre_cam: b_b, rayon_monde: r_barreau, couleur: blanc_teflon });

            let facteur_echelle = self.echelle_taille_molecules;
            for p in &self.physique.particules {
                ajouter_molecule_vsepr_3d(
                    &mut elements,
                    &self.camera,
                    p.pos,
                    p.rot,
                    p.espece,
                    facteur_echelle,
                    self.afficher_coque_polyedre,
                );
            }

            rendre_scene_moleculaire_3d(&painter, &self.camera, centre_ecran, echelle_base, &mut elements);

            let verre_avant = Stroke::new(2.4, Color32::from_rgba_unmultiplied(210, 230, 250, 215));
            let verre_arriere = Stroke::new(1.4, Color32::from_rgba_unmultiplied(150, 175, 200, 90));

            dessiner_anneau_3d(&painter, &self.camera, centre_ecran, echelle_base, y_haut, r_becher, 48, verre_avant, verre_arriere);
            dessiner_anneau_3d(&painter, &self.camera, centre_ecran, echelle_base, y_fond, r_becher, 48, verre_avant, verre_arriere);

            let menisque_avant = Stroke::new(2.2, Color32::from_rgba_unmultiplied(r_liq, g_liq, b_liq, 235));
            let menisque_arriere = Stroke::new(1.2, Color32::from_rgba_unmultiplied(r_liq, g_liq, b_liq, 110));
            dessiner_anneau_3d(&painter, &self.camera, centre_ecran, echelle_base, y_surface, r_becher * 0.99, 48, menisque_avant, menisque_arriere);

            let angle_bord = -self.camera.yaw;
            for (i, ml) in [50, 100, 150, 200, 250].iter().enumerate() {
                let frac = (i as f32 + 1.0) / 5.0;
                let y_grad = y_fond + frac * (y_250ml - y_fond);
                let p_bord = Vec3::new(-r_becher * angle_bord.cos(), y_grad, -r_becher * angle_bord.sin());
                let p_tick = Vec3::new(-r_becher * (angle_bord + 0.14).cos(), y_grad, -r_becher * (angle_bord + 0.14).sin());
                let (pos0, _) = self.camera.projeter(p_bord, centre_ecran, echelle_base);
                let (pos1, _) = self.camera.projeter(p_tick, centre_ecran, echelle_base);
                painter.line_segment([pos0, pos1], Stroke::new(1.8, Color32::from_rgba_unmultiplied(220, 235, 250, 190)));
                painter.text(
                    pos0 + Vec2::new(-34.0, 0.0),
                    Align2::LEFT_CENTER,
                    format!("{} mL", ml),
                    FontId::proportional(11.5),
                    Color32::from_rgba_unmultiplied(225, 240, 255, 215),
                );
            }

            for signe in [-1.0_f32, 1.0_f32] {
                let sx = signe * r_becher * angle_bord.cos();
                let sz = signe * r_becher * angle_bord.sin();
                let (p_top, _) = self.camera.projeter(Vec3::new(sx, y_haut, sz), centre_ecran, echelle_base);
                let (p_bot, _) = self.camera.projeter(Vec3::new(sx, y_fond, sz), centre_ecran, echelle_base);
                painter.line_segment([p_top, p_bot], verre_avant);
            }

            painter.text(
                Pos2::new(rect_dispo.left() + 14.0, rect_dispo.bottom() - 14.0),
                Align2::LEFT_BOTTOM,
                "Controls: Left-Drag = Orbit 3D | Scroll Wheel = True 3D Zoom | Right-Drag = Pan",
                FontId::proportional(11.5),
                Color32::from_rgb(145, 165, 195),
            );
        });

        ctx.request_repaint();
    }
}