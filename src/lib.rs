use std::fs;
use std::io::Cursor;
use psd::{Psd};
use image::RgbaImage;

/****************
 * Generate png *
 ****************/
pub fn generate(j: usize, m: usize, a: usize, age: usize, path_psd: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let calques = calcl(j, m, a, age);
    let psd_bytes = fs::read(&path_psd)
        .map_err(|e| format!("Impossible de lire le PSD à '{}': {}", &path_psd, e))?;
    let psd = Psd::from_bytes(&psd_bytes).unwrap();
    let (doc_w, doc_h) = (psd.width(), psd.height());
    let final_image: Vec<u8> = psd.rgba();
    let mut final_image_img = RgbaImage::from_raw(doc_w, doc_h, final_image)
        .expect("Le buffer RGBA ne correspond pas aux dimensions w*h*4");
    for calque in calques.iter() {
        for layer in psd.layers().iter().filter(|x| x.name() == calque.to_string()){
            let name = layer.name();
            //if name != "" {
            //    println!("Layer name: {}", name);
            //}

            let pixels: Vec<u8> = layer.rgba();

            // Dimensions document en usize pour l'indexation
            let (dw, dh) = (doc_w as usize, doc_h as usize);

            // Sécurité: on vérifie la cohérence avec la taille du document
            if pixels.len() != dw * dh * 4 {
                eprintln!(
                    "Taille de buffer inattendue (document) pour le calque '{}': {} != {}",
                    name,
                    pixels.len(),
                    dw * dh * 4
                );
                continue;
            }

            // Superposition à l'origine (0,0) sur toute la surface du document
            for y in 0..dh {
                for x in 0..dw {
                    let src_idx = ((y * dw + x) * 4) as usize;

                    let sr = pixels[src_idx] as f32;
                    let sg = pixels[src_idx + 1] as f32;
                    let sb = pixels[src_idx + 2] as f32;
                    let sa = pixels[src_idx + 3] as f32 / 255.0;

                    if sa == 0.0 {
                        continue;
                    }
                    let dst_px = final_image_img.get_pixel_mut((x) as u32, (y) as u32);

                    let dr = dst_px[0] as f32;
                    let dg = dst_px[1] as f32;
                    let db = dst_px[2] as f32;
                    let da = dst_px[3] as f32 / 255.0;

                    // Alpha-over (SRC over DST)
                    let out_a = sa + da * (1.0 - sa);
                    let (out_r, out_g, out_b) = if out_a > 0.0 {
                        (
                            (sr * sa + dr * da * (1.0 - sa)) / out_a,
                            (sg * sa + dg * da * (1.0 - sa)) / out_a,
                            (sb * sa + db * da * (1.0 - sa)) / out_a,
                        )
                    } else {
                        (0.0, 0.0, 0.0)
                    };

                    *dst_px = image::Rgba([
                        out_r.clamp(0.0, 255.0) as u8,
                        out_g.clamp(0.0, 255.0) as u8,
                        out_b.clamp(0.0, 255.0) as u8,
                        (out_a * 255.0).clamp(0.0, 255.0) as u8,
                    ]);
                }
            }
        }
    }
    //final_image_img.save("./tmp/cycles.png").unwrap();
    let mut buf = Vec::new();
    final_image_img
        .write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
        .expect("Échec encodage PNG");
    Ok(buf)
}

/**************************
 * Réduction théosophique *
 * sw_9 = false -> 22     *
 * sw_9 = true -> 9       *
 **************************/
fn reduction_theosophique(mut n: usize, sw_9: bool) -> usize {
    let t = if sw_9 { 9 } else { 22 };
    while n > t {
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }
        n = sum;
    }
    n
}

/********************************
 * Calcul des calques Photoshop *
 ********************************/
fn calcl(j: usize, m: usize, a: usize, age: usize) -> Vec<String> {
    let caipa = reduction_theosophique(j, false);
    let caisa = reduction_theosophique(caipa, true);
    //---
    let intpa = reduction_theosophique(m, false);
    let mut intpb = 0;
    if j > 22 {
        let mut mm = intpa + 1;
        if mm > 12 {
            mm = 1;
        }
        intpb = reduction_theosophique(mm, false);
    }
    let intsa = reduction_theosophique(intpa, true);
    let mut intsb = 0;
    if intsa > 0 {
        if j > 22 {
            let mut mm = intpa + 1;
            if mm > 12 {
                mm = 1;
            }
            intsb = reduction_theosophique(mm, true);
        }
    }
    //---
    let caepa = reduction_theosophique(a, false);
    let caesa = reduction_theosophique(caepa, true);
    //---
    let coipa = reduction_theosophique(caipa + intpa, false);
    let coipb = reduction_theosophique(caipa + intpb, false);
    let coisa = reduction_theosophique(caisa + intsa, false);
    let coisb = reduction_theosophique(caisa + intsb, false);
    //---
    let coepa = reduction_theosophique(caepa + intpa, false);
    let coepb = reduction_theosophique(caepa + intpb, false);
    let coesa = reduction_theosophique(caesa + intsa, false);
    let coesb = reduction_theosophique(caesa + intsb, false);
    //---
    let nempa = reduction_theosophique(caipa + caepa, false);
    let nemsa = reduction_theosophique(caisa + caesa, false);
    //---
    let pexpa = reduction_theosophique(coipa + coepa, false);
    let pexpb = reduction_theosophique(coipb + coepb, false);
    let pexsa = reduction_theosophique(coisa + coesa, false);
    let pexsb = reduction_theosophique(coisb + coesb, false);
    //---
    let pprpa = reduction_theosophique(caipa + intpa + caepa, false);
    let pprpb = reduction_theosophique(caipa + intpb + caepa, false);
    let pprsa = reduction_theosophique(caisa + intsa + caesa, false);
    let pprsb = reduction_theosophique(caisa + intsb + caesa, false);
    //println!("J: {:4} PA: {:4} SA: {:4}", j, caipa, caisa);
    //println!("M: {:4} PA: {:4} SA: {:4} PB: {:4} SB: {:4}", m, intpa, intsa, intpb, intsb);
    //println!("A: {:4} PA: {:4} SA: {:4}", a, caepa, caesa);
    //println!("COI:    PA: {:4} SA: {:4} PB: {:4} SB: {:4}", coipa, coipb, coisa, coisb);
    //println!("COE:    PA: {:4} SA: {:4} PB: {:4} SB: {:4}", coepa, coepb, coesa, coesb);
    //println!("NEM:    PA: {:4} SA: {:4}", nempa, nemsa);
    //println!("PEX:    PA: {:4} SA: {:4} PB: {:4} SB: {:4}", pexpa, pexpb, pexsa, pexsb);
    //println!("PPR:    PA: {:4} SA: {:4} PB: {:4} SB: {:4}", pprpa, pprpb, pprsa, pprsb);
    //---
    let _f_caipa = Some(caipa);
    let f_caisa = if caipa == caisa { None } else { Some(caisa) };
    let _f_intpa = Some(intpa);
    let f_intpb = if intpa == intpb || intpb == 0 { None } else { Some(intpb) };
    let f_intsa = if intpa == intsa { None } else { Some(intsa) };
    let f_intsb = if intsa == intsb || intsb == 0 { None } else { Some(intsb) };
    let _f_caepa = Some(caepa);
    let f_caesa = if caepa == caesa { None } else { Some(caesa) };
    //---
    let _f_coipa = Some(coipa);
    let f_coipb = if f_intpb.is_some() { Some(coipb) } else { None };
    let f_coisa = if f_caisa.is_some() || f_intsa.is_some() { Some(coisa) } else { None };
    let f_coisb = if f_caisa.is_some() || f_intsb.is_some() { Some(coisb) } else { None };
    let _f_coepa = Some(coepa);
    let f_coepb = if f_intpb.is_some() { Some(coepb) } else { None };
    let f_coesa = if f_caesa.is_some() || f_intsa.is_some() { Some(coesa) } else { None };
    let f_coesb = if f_caesa.is_some() || f_intsb.is_some() { Some(coesb) } else { None };

    let f_nempa = Some(nempa);
    let f_nemsa = if f_caisa.is_some() || f_caesa.is_some() { Some(nemsa) } else { None };

    let f_pexpa = Some(pexpa);
    let f_pexpb = if f_coipb.is_some() || f_coepb.is_some() { Some(pexpb) } else { None };
    let f_pexsa = if f_coisa.is_some() || f_coesa.is_some() { Some(pexsa) } else { None };
    let f_pexsb = if f_coisb.is_some() || f_coesb.is_some() { Some(pexsb) } else { None };

    let f_pprpa = Some(pprpa);
    let f_pprpb = if f_intpb.is_some() { Some(pprpb) } else { None };
    let f_pprsa = if f_intsa.is_some() || f_caisa.is_some() || f_caesa.is_some() { Some(pprsa) } else { None };
    let f_pprsb = if f_intsb.is_some() || f_caisa.is_some() || f_caesa.is_some() { Some(pprsb) } else { None };

    //println!("Final CAI: {:?}/{:?}", f_caipa, f_caisa);
    //println!("Final INT: {:?}/{:?}/{:?}/{:?}", f_intpa, f_intpb, f_intsa, f_intsb);
    //println!("Final CAE: {:?}/{:?}", f_caepa, f_caesa);
    //println!("Final COI: {:?}/{:?}/{:?}/{:?}", f_coipa, f_coipb, f_coisa, f_coisb);
    //println!("Final COE: {:?}/{:?}/{:?}/{:?}", f_coepa, f_coepb, f_coesa, f_coesb);
    //println!("Final NEM: {:?}/{:?}", f_nempa, f_nemsa);
    //println!("Final PEX: {:?}/{:?}/{:?}/{:?}", f_pexpa, f_pexpb, f_pexsa, f_pexsb);
    //println!("Final PPR: {:?}/{:?}/{:?}/{:?}", f_pprpa, f_pprpb, f_pprsa, f_pprsb);
    //---
    let pulsion_pprpa = reduction_theosophique(age + 1, false);
    let f_pulsion_pprpa = Some(pulsion_pprpa);
    let f_pulsion_pprsa = if f_pprsa.is_some() { f_pulsion_pprpa } else { None };
    //println!("PulsioPPR: {:?}/{:?}", f_pulsion_pprpa, f_pulsion_pprsa);
    let action_pprpa = reduction_theosophique(reduction_theosophique(age + 1, false) + pprpa, false);
    let action_pprpb = reduction_theosophique(reduction_theosophique(age + 1, false) + pprpb, false);
    let action_pprsa = reduction_theosophique(reduction_theosophique(age + 1, false) + pprsa, false);
    let action_pprsb = reduction_theosophique(reduction_theosophique(age + 1, false) + pprsb, false);
    let f_action_pprpa = Some(action_pprpa);
    let f_action_pprpb = if f_pprpb.is_some() { Some(action_pprpb) } else { None };
    let f_action_pprsa = if f_pprsa.is_some() { Some(action_pprsa) } else { None };
    let f_action_pprsb = if f_pprsb.is_some() { Some(action_pprsb) } else { None };
    //println!("ActionPPR: {:?}/{:?}/{:?}/{:?}", f_action_pprpa, f_action_pprpb, f_action_pprsa, f_action_pprsb);
    let reaction_pprpa = reduction_theosophique(action_pprpa + pprpa, false);
    let reaction_pprpb = reduction_theosophique(action_pprpb + pprpb, false);
    let reaction_pprsa = reduction_theosophique(action_pprsa + pprsa, false);
    let reaction_pprsb = reduction_theosophique(action_pprsb + pprsb, false);
    let f_reaction_pprpa = Some(reaction_pprpa);
    let f_reaction_pprpb = if f_pprpb.is_some() { Some(reaction_pprpb) } else { None };
    let f_reaction_pprsa = if f_pprsa.is_some() { Some(reaction_pprsa) } else { None };
    let f_reaction_pprsb = if f_pprsb.is_some() { Some(reaction_pprsb) } else { None };
    //println!("RéactiPPR: {:?}/{:?}/{:?}/{:?}", f_reaction_pprpa, f_reaction_pprpb, f_reaction_pprsa, f_reaction_pprsb);
    //---
    //println!("ActionNEM: {:?}/{:?}", f_action_nempa, f_action_nemsa);
    let f_pulsion_nempa = f_pulsion_pprpa;
    let f_pulsion_nemsa = if f_nemsa.is_some() { f_pulsion_pprsa } else { None };
    //println!("PulsioNEM: {:?}/{:?}", f_pulsion_nempa, f_pulsion_nemsa);
    let action_nempa = reduction_theosophique(reduction_theosophique(age + 1, false) + nempa, false);
    let action_nemsa = reduction_theosophique(reduction_theosophique(age + 1, false) + nemsa, false);
    let f_action_nempa = Some(action_nempa);
    let f_action_nemsa = if f_nemsa.is_some() { Some(action_nemsa) } else { None };
    //println!("ActionNEM: {:?}/{:?}", f_action_nempa, f_action_nemsa);
    let reaction_nempa = reduction_theosophique(action_nempa + nempa, false);
    let reaction_nemsa = reduction_theosophique(action_nemsa + nemsa, false);
    let f_reaction_nempa = Some(reaction_nempa);
    let f_reaction_nemsa = if f_nemsa.is_some() { Some(reaction_nemsa) } else { None };
    //println!("RéactiNEM: {:?}/{:?}", f_reaction_nempa, f_reaction_nemsa);
    //---
    let f_pulsion_pexpa = f_pulsion_pprpa;
    let f_pulsion_pexsa = f_pulsion_pprsa;
    //println!("PulsioPEX: {:?}/{:?}", f_pulsion_pexpa, f_pulsion_pexsa);
    let action_pexpa = reduction_theosophique(reduction_theosophique(age + 1, false) + pexpa, false);
    let action_pexpb = reduction_theosophique(reduction_theosophique(age + 1, false) + pexpb, false);
    let action_pexsa = reduction_theosophique(reduction_theosophique(age + 1, false) + pexsa, false);
    let action_pexsb = reduction_theosophique(reduction_theosophique(age + 1, false) + pexsb, false);
    let f_action_pexpa = Some(action_pexpa);
    let f_action_pexpb = if f_pexpb.is_some() { Some(action_pexpb) } else { None };
    let f_action_pexsa = if f_pexsa.is_some() { Some(action_pexsa) } else { None };
    let f_action_pexsb = if f_pexsb.is_some() { Some(action_pexsb) } else { None };
    //println!("ActionPEX: {:?}/{:?}/{:?}/{:?}", f_action_pexpa, f_action_pexpb, f_action_pexsa, f_action_pexsb);
    let reaction_pexpa = reduction_theosophique(action_pexpa + pexpa, false);
    let reaction_pexpb = reduction_theosophique(action_pexpb + pexpb, false);
    let reaction_pexsa = reduction_theosophique(action_pexsa + pexsa, false);
    let reaction_pexsb = reduction_theosophique(action_pexsb + pexsb, false);
    let f_reaction_pexpa = Some(reaction_pexpa);
    let f_reaction_pexpb = if f_pexpb.is_some() { Some(reaction_pexpb) } else { None };
    let f_reaction_pexsa = if f_pexsa.is_some() { Some(reaction_pexsa) } else { None };
    let f_reaction_pexsb = if f_pexsb.is_some() { Some(reaction_pexsb) } else { None };
    //println!("RéactiPEX: {:?}/{:?}/{:?}/{:?}", f_reaction_pexpa, f_reaction_pexpb, f_reaction_pexsa, f_reaction_pexsb);

    //---
    let mut calque_a: Vec<String> = vec![];
    let mut calque_b: Vec<String> = vec![];
    //---
    match f_pprpa {
        Some(x) => {
            calque_a.push(format!("PPRPA{:02}", x));
        },
        None => {}
    }
    match f_pprpb {
        Some(x) => {
            calque_b.push(format!("PPRPB{:02}", x));
        },
        None => {}
    }
    match f_pprsa {
        Some(x) => {
            calque_a.push(format!("PPRSA{:02}", x));
        },
        None => {}
    }
    match f_pprsb {
        Some(x) => {
            calque_b.push(format!("PPRSB{:02}", x));
        },
        None => {}
    }
    match f_pulsion_pprpa {
        Some(x) => {
            calque_a.push(format!("PPPPA{:02}", x));
        },
        None => {}
    }
    match f_pulsion_pprsa {
        Some(x) => {
            calque_a.push(format!("PPPSA{:02}", x));
        },
        None => {}
    }
    match f_action_pprpa {
        Some(x) => {
            calque_a.push(format!("APPPA{:02}", x));
        },
        None => {}
    }
    match f_action_pprpb {
        Some(x) => {
            calque_b.push(format!("APPPB{:02}", x));
        },
        None => {}
    }
    match f_action_pprsa {
        Some(x) => {
            calque_a.push(format!("APPSA{:02}", x));
        },
        None => {}
    }
    match f_action_pprsb {
        Some(x) => {
            calque_b.push(format!("APPSB{:02}", x));
        },
        None => {}
    }
    match f_reaction_pprpa {
        Some(x) => {
            calque_a.push(format!("RPPPA{:02}", x));
        },
        None => {}
    }
    match f_reaction_pprpb {
        Some(x) => {
            calque_b.push(format!("RPPPB{:02}", x));
        },
        None => {}
    }
    match f_reaction_pprsa {
        Some(x) => {
            calque_a.push(format!("RPPSA{:02}", x));
        },
        None => {}
    }
    match f_reaction_pprsb {
        Some(x) => {
            calque_b.push(format!("RPPSB{:02}", x));
        },
        None => {}
    }
    //---
    match f_nempa {
        Some(x) => {
            calque_a.push(format!("NEMPA{:02}", x));
        },
        None => {}
    }
    match f_nemsa {
        Some(x) => {
            calque_a.push(format!("NEMSA{:02}", x));
        },
        None => {}
    }
    match f_pulsion_nempa {
        Some(x) => {
            calque_a.push(format!("PNEPA{:02}", x));
        },
        None => {}
    }
    match f_pulsion_nemsa {
        Some(x) => {
            calque_a.push(format!("PNESA{:02}", x));
        },
        None => {}
    }
    match f_action_nempa {
        Some(x) => {
            calque_a.push(format!("ANEPA{:02}", x));
        },
        None => {}
    }
    match f_action_nemsa {
        Some(x) => {
            calque_a.push(format!("ANESA{:02}", x));
        },
        None => {}
    }
    match f_reaction_nempa {
        Some(x) => {
            calque_a.push(format!("RNEPA{:02}", x));
        },
        None => {}
    }
    match f_reaction_nemsa {
        Some(x) => {
            calque_a.push(format!("RNESA{:02}", x));
        },
        None => {}
    }
    //---
    match f_pexpa {
        Some(x) => {
            calque_a.push(format!("PEXPA{:02}", x));
        },
        None => {}
    }
    match f_pexpb {
        Some(x) => {
            calque_b.push(format!("PEXPB{:02}", x));
        },
        None => {}
    }
    match f_pexsa {
        Some(x) => {
            calque_a.push(format!("PEXSA{:02}", x));
        },
        None => {}
    }
    match f_pexsb {
        Some(x) => {
            calque_b.push(format!("PEXSB{:02}", x));
        },
        None => {}
    }
    match f_pulsion_pexpa {
        Some(x) => {
            calque_a.push(format!("PPEPA{:02}", x));
        },
        None => {}
    }
    match f_pulsion_pexsa {
        Some(x) => {
            calque_a.push(format!("PPESA{:02}", x));
        },
        None => {}
    }
    match f_action_pexpa {
        Some(x) => {
            calque_a.push(format!("APEPA{:02}", x));
        },
        None => {}
    }
    match f_action_pexpb {
        Some(x) => {
            calque_b.push(format!("APEPB{:02}", x));
        },
        None => {}
    }
    match f_action_pexsa {
        Some(x) => {
            calque_a.push(format!("APESA{:02}", x));
        },
        None => {}
    }
    match f_action_pexsb {
        Some(x) => {
            calque_b.push(format!("APESB{:02}", x));
        },
        None => {}
    }
    match f_reaction_pexpa {
        Some(x) => {
            calque_a.push(format!("RPEPA{:02}", x));
        },
        None => {}
    }
    match f_reaction_pexpb {
        Some(x) => {
            calque_b.push(format!("RPEPB{:02}", x));
        },
        None => {}
    }
    match f_reaction_pexsa {
        Some(x) => {
            calque_a.push(format!("RPESA{:02}", x));
        },
        None => {}
    }
    match f_reaction_pexsb {
        Some(x) => {
            calque_b.push(format!("RPESB{:02}", x));
        },
        None => {}
    }
    //---
    let calque_ac: Vec<String> = calque_a.iter().filter(|x| { trouver_cadre(x).is_some() }).map(|x| trouver_cadre(x).unwrap()).collect::<Vec<_>>();
    let calque_bc: Vec<String> = calque_b.iter().filter(|x| { trouver_cadre(x).is_some() }).map(|x| trouver_cadre(x).unwrap()).collect::<Vec<_>>();
    let mut calques: Vec<String> = calque_bc.clone();
    calques.extend(calque_b.iter().cloned());
    calques.extend(calque_ac.iter().cloned());
    calques.extend(calque_a.iter().cloned());
    calques
}

/***********************************
 * Trouver cadre dans le photoshop *
 * En fonction de ma manière de    *
 * nommer les layers               *
 ***********************************/
fn trouver_cadre(claque: &str) -> Option<String> {
    // Personalité Profonde PA
    if claque.starts_with("PPRPA") {
        return Some("PPRPA-R".to_string())
    }
    // Personalité Profonde PB
    if claque.starts_with("PPRPB") {
        return Some("PPRPB-R".to_string())
    }
    // Personalité Profonde SA
    if claque.starts_with("PPRSA") {
        return Some("PPRSA-R".to_string())
    }
    // Personalité Profonde SB
    if claque.starts_with("PPRSB") {
        return Some("PPRSB-R".to_string())
    }
    // Personalité Profonde Pulsion PA
    if claque.starts_with("PPPPA") {
        return Some("PPPPA-R".to_string())
    }
    // Personalité Profonde Pulsion SA
    if claque.starts_with("PPRPA") {
        return Some("PPPSA-R".to_string())
    }
    // Personalité Profonde Action PA
    if claque.starts_with("APPPA") {
        return Some("APPPA-R".to_string())
    }
    // Personalité Profonde Action PB
    if claque.starts_with("APPPB") {
        return Some("APPPB-R".to_string())
    }
    // Personalité Profonde Action SA
    if claque.starts_with("APPSA") {
        return Some("APPSA-R".to_string())
    }
    // Personalité Profonde Action SB
    if claque.starts_with("APPSB") {
        return Some("APPSB-R".to_string())
    }
    // Personalité Profonde Réaction PA
    if claque.starts_with("RPPPA") {
        return Some("RPPPA-R".to_string())
    }
    // Personalité Profonde Réaction PB
    if claque.starts_with("RPPPB") {
        return Some("RPPPB-R".to_string())
    }
    // Personalité Profonde Réaction SA
    if claque.starts_with("RPPSA") {
        return Some("RPPSA-R".to_string())
    }
    // Personalité Profonde Réaction SB
    if claque.starts_with("RPPSB") {
        return Some("RPPSB-R".to_string())
    }
    // NEM PA
    if claque.starts_with("NEMPA") {
        return Some("NEMPA-R".to_string())
    }
    // NEM SA
    if claque.starts_with("NEMSA") {
        return Some("NEMSA-R".to_string())
    }
    // NEM Pulsion PA
    if claque.starts_with("PNEPA") {
        return Some("PNEPA-R".to_string())
    }
    // NEM Pulsion SA
    if claque.starts_with("PNESA") {
        return Some("PNESA-R".to_string())
    }
    // NEM Action PA
    if claque.starts_with("ANEPA") {
        return Some("ANEPA-R".to_string())
    }
    // NEM Action SA
    if claque.starts_with("ANESA") {
        return Some("ANESA-R".to_string())
    }
    // NEM Réaction PA
    if claque.starts_with("RNEPA") {
        return Some("RNEPA-R".to_string())
    }
    // NEM Réaction SA
    if claque.starts_with("RNESA") {
        return Some("RNESA-R".to_string())
    }
    // Personalité Extérieur PA
    if claque.starts_with("PEXPA") {
        return Some("PEXPA-R".to_string())
    }
    // Personalité Extérieur PB
    if claque.starts_with("PEXPB") {
        return Some("PEXPB-R".to_string())
    }
    // Personalité Extérieur SA
    if claque.starts_with("PEXSA") {
        return Some("PEXSA-R".to_string())
    }
    // Personalité Extérieur SB
    if claque.starts_with("PEXSB") {
        return Some("PEXSB-R".to_string())
    }
    // Personalité Extérieur Pulsion PA
    if claque.starts_with("PPEPA") {
        return Some("PPEPA-R".to_string())
    }
    // Personalité Extérieur Pulsion SA
    if claque.starts_with("PPESA") {
        return Some("PPESA-R".to_string())
    }
    // Personalité Extérieur Action PA
    if claque.starts_with("APEPA") {
        return Some("APEPA-R".to_string())
    }
    // Personalité Extérieur Action PB
    if claque.starts_with("APEPB") {
        return Some("APEPB-R".to_string())
    }
    // Personalité Extérieur Action SA
    if claque.starts_with("APESA") {
        return Some("APESA-R".to_string())
    }
    // Personalité Extérieur Action SB
    if claque.starts_with("APESB") {
        return Some("APESB-R".to_string())
    }
    // Personalité Extérieur Réaction PA
    if claque.starts_with("RPEPA") {
        return Some("RPEPA-R".to_string())
    }
    // Personalité Extérieur Réaction PB
    if claque.starts_with("RPEPB") {
        return Some("RPEPB-R".to_string())
    }
    // Personalité Extérieur Réaction SA
    if claque.starts_with("RPESA") {
        return Some("RPESA-R".to_string())
    }
    // Personalité Extérieur Réaction SB
    if claque.starts_with("RPESB") {
        return Some("RPESB-R".to_string())
    }
    None
}