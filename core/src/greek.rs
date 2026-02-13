struct ConsumeResult {
    result: Vec<Greek>,
    consumed: usize,
}

use crate::CapitalisationMode;

use super::Sound;

fn do_the_job(input: &[Sound]) -> Vec<Greek> {
    let mut result = vec![];
    let mut i = 0;

    while i < input.len() {
        let consume_result = consume_greek(&input[i..]);
        result.extend(consume_result.result);
        i += consume_result.consumed;
    }

    result
}

pub fn to_greek(input: &Vec<Sound>) -> String {
    let greek_chars = do_the_job(input);
    greek_chars.iter().map(|&g| to_char(g)).collect()
}

fn consume_naive(input: Sound) -> &'static [Greek] {
    use Greek::*;
    match input {
        Sound::A => &[Alpha],
        Sound::B => &[Mu, Pi],
        Sound::C => &[Tau, Sigma],
        Sound::Ch => &[Theta],
        Sound::D => &[Delta],
        Sound::Dx => &[Delta, Acute],
        Sound::E => &[Epsilon],
        Sound::Ex => &[Eta],
        Sound::F => &[Phi],
        Sound::G => &[Gamma],
        Sound::H => &[Chi],
        Sound::I => &[IotaAcute],
        Sound::J => &[Acute],
        Sound::K => &[Kappa],
        Sound::L => &[Lambda, Acute],
        Sound::Lx => &[Lambda],
        Sound::M => &[Mu],
        Sound::N => &[Nu],
        Sound::Nx => &[Nu, Acute],
        Sound::O => &[Omicron],
        Sound::Ox => &[Omega],
        Sound::Ou => &[Omicron, Upsilon],
        Sound::P => &[Pi],
        Sound::R => &[Rho],
        Sound::S => &[Sigma],
        Sound::Sx => &[Sigma, Acute],
        Sound::Sh => &[Psi],
        Sound::T => &[Tau],
        Sound::Tx => &[Tau, Acute],
        Sound::U => &[Upsilon],
        Sound::W => &[Beta],
        Sound::Y => &[Iota],
        Sound::Z => &[Zeta],
        Sound::Zx => &[Zeta, Acute],
        Sound::Zh => &[Xi],
        Sound::Rx => &[Rho, Acute],
        Sound::Dz => &[Delta, Zeta],
        Sound::Dh => &[Delta, Xi],
    }
}

fn softened_sound_to_base_greek(s: Sound) -> Greek {
    use Greek::*;
    use Sound::*;
    match s {
        Sx => Sigma,
        Zx => Zeta,
        L => Lambda,
        Tx => Tau,
        Nx => Nu,
        I => Iota,
        Rx => Rho,
        Dx => Delta,
        _ => panic!("Unexpected softened sound: {:?}", s),
    }
}

fn soften_vowel(sound: Sound) -> &'static [Greek] {
    use Greek::*;
    use Sound::*;
    match sound {
        A => &[AlphaAcute],
        E => &[EpsilonAcute],
        Ex => &[EtaAcute],
        I => &[IotaAcute],
        O => &[OmicronAcute],
        Ou => &[OmicronAcute, Upsilon],
        Ox => &[OmegaAcute],
        U => &[UpsilonAcute],
        _ => panic!("Unexpected vowel for softening: {:?}", sound),
    }
}

fn consume_greek(input: &[Sound]) -> ConsumeResult {
    if input.len() == 0 {
        return ConsumeResult {
            result: vec![],
            consumed: 0,
        };
    }

    let i0 = input[0];

    if input.len() == 1 {
        return ConsumeResult {
            result: consume_naive(i0).to_vec(),
            consumed: 1,
        };
    }

    let mut softened_count = 0;
    while softened_count < input.len()
        && input[softened_count].is_softened()
        && input[softened_count] != Sound::I
    {
        softened_count += 1;
    }

    if softened_count > 0 {
        let following_vowel = input.get(softened_count).cloned().filter(|x| x.is_vowel());
        let total_consumed = softened_count + following_vowel.map_or(0, |_| 1);

        let last_greek: &[Greek] = match following_vowel {
            Some(Y) => &[Greek::IotaAcute], // SPECIAL CASE FOR "RZY", TODO MAKE SURE "RZ" IS PRESENT
            Some(vowel) => soften_vowel(vowel),
            None => &[Greek::Acute],
        };

        let mut result = input[..softened_count]
            .iter()
            .map(|&s| softened_sound_to_base_greek(s))
            .collect::<Vec<Greek>>();

        result.extend_from_slice(last_greek);

        return ConsumeResult {
            result,
            consumed: total_consumed,
        };
    }

    use Sound::*;
    if i0 == Sound::J {
        let i1 = input.get(1).unwrap().clone();
        if i1.is_vowel() && i1 != Y && i1 != I {
            return ConsumeResult {
                result: soften_vowel(i1).to_vec(),
                consumed: 2,
            };
            // TODO Case for I Y
        } else {
            return ConsumeResult {
                result: vec![Greek::Acute],
                consumed: 1,
            };
        }
    }

    // check for break
    let i1 = input.get(1).unwrap().clone();
    if i1 == J || i1 == I {
        //|| i1 == Nx || i1 == Zx || i1 == Sx || i1 == Tx  || i1 == Dx ||i1==Rx{
        if i0 == N || i0 == Z || i0 == S || i0 == T || i0 == D || i0 == R {
            let mut r0 = consume_naive(i0).to_vec();
            r0.push(Greek::Break);
            return ConsumeResult {
                result: r0,
                consumed: 1,
            };
        }
    }

    return ConsumeResult {
        result: consume_naive(i0).to_vec(),
        consumed: 1,
    };
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Greek {
    Alpha,
    AlphaAcute,
    Beta,
    Gamma,
    Delta,
    Epsilon,
    EpsilonAcute,
    Zeta,
    Eta,
    EtaAcute,
    Theta,
    Iota,
    IotaAcute,
    Kappa,
    Lambda,
    Mu,
    Nu,
    Xi,
    Omicron,
    OmicronAcute,
    Pi,
    Rho,
    Sigma,
    Tau,
    Upsilon,
    UpsilonAcute,
    Phi,
    Chi,
    Psi,
    Omega,
    OmegaAcute,
    Acute,
    Break,
}

impl Greek {
    fn is_softening(self) -> bool {
        match self {
            Greek::Acute
            | Greek::AlphaAcute
            | Greek::EpsilonAcute
            | Greek::EtaAcute
            | Greek::IotaAcute
            | Greek::OmicronAcute
            | Greek::UpsilonAcute
            | Greek::OmegaAcute => true,
            _ => false,
        }
    }

    fn can_be_softened(self) -> bool {
        use Greek::*;
        match self {
            Rho | Nu | Zeta | Sigma | Delta | Tau | Lambda => true,
            _ => false,
        }
    }
}

fn to_char(greek: Greek) -> char {
    match greek {
        Greek::Alpha => 'α',
        Greek::Beta => 'β',
        Greek::Gamma => 'γ',
        Greek::Delta => 'δ',
        Greek::Epsilon => 'ε',
        Greek::Zeta => 'ζ',
        Greek::Eta => 'η',
        Greek::EtaAcute => 'ή',
        Greek::Theta => 'θ',
        Greek::Iota => 'ι',
        Greek::Kappa => 'κ',
        Greek::Lambda => 'λ',
        Greek::Mu => 'μ',
        Greek::Nu => 'ν',
        Greek::Xi => 'ξ',
        Greek::Omicron => 'ο',
        Greek::Pi => 'π',
        Greek::Rho => 'ρ',
        Greek::Sigma => 'σ',
        Greek::Tau => 'τ',
        Greek::Upsilon => 'υ',
        Greek::Phi => 'φ',
        Greek::Chi => 'χ',
        Greek::Psi => 'ψ',
        Greek::Omega => 'ω',
        Greek::AlphaAcute => 'ά',
        Greek::EpsilonAcute => 'έ',
        Greek::IotaAcute => 'ί',
        Greek::OmicronAcute => 'ό',
        Greek::UpsilonAcute => 'ύ',
        Greek::OmegaAcute => 'ώ',
        Greek::Acute => '\'',
        Greek::Break => '\\',
    }
}

fn char_to_greek(c: char) -> Option<Greek> {
    match c {
        'α' => Some(Greek::Alpha),
        'β' => Some(Greek::Beta),
        'γ' => Some(Greek::Gamma),
        'δ' => Some(Greek::Delta),
        'ε' => Some(Greek::Epsilon),
        'ζ' => Some(Greek::Zeta),
        'η' => Some(Greek::Eta),
        'ή' => Some(Greek::EtaAcute),
        'θ' => Some(Greek::Theta),
        'ι' => Some(Greek::Iota),
        'κ' => Some(Greek::Kappa),
        'λ' => Some(Greek::Lambda),
        'μ' => Some(Greek::Mu),
        'ν' => Some(Greek::Nu),
        'ξ' => Some(Greek::Xi),
        'ο' => Some(Greek::Omicron),
        'π' => Some(Greek::Pi),
        'ρ' => Some(Greek::Rho),
        'σ' => Some(Greek::Sigma),
        'τ' => Some(Greek::Tau),
        'υ' => Some(Greek::Upsilon),
        'φ' => Some(Greek::Phi),
        'χ' => Some(Greek::Chi),
        'ψ' => Some(Greek::Psi),
        'ω' => Some(Greek::Omega),
        'ά' => Some(Greek::AlphaAcute),
        'έ' => Some(Greek::EpsilonAcute),
        'ί' => Some(Greek::IotaAcute),
        'ό' => Some(Greek::OmicronAcute),
        'ύ' => Some(Greek::UpsilonAcute),
        'ώ' => Some(Greek::OmegaAcute),
        '\'' => Some(Greek::Acute),
        '\\' => Some(Greek::Break),
        _ => None,
    }
}

fn naive_greek_to_sound(g: Greek) -> &'static [Sound] {
    match g {
        Greek::Alpha => &[Sound::A],
        Greek::Beta => &[Sound::W],
        Greek::Gamma => &[Sound::G],
        Greek::Delta => &[Sound::D],
        Greek::Epsilon => &[Sound::E],
        Greek::Zeta => &[Sound::Z],
        Greek::Eta => &[Sound::Ex],
        Greek::Theta => &[Sound::Ch],
        Greek::Iota => &[Sound::Y],
        Greek::Kappa => &[Sound::K],
        Greek::Lambda => &[Sound::Lx],
        Greek::Mu => &[Sound::M],
        Greek::Nu => &[Sound::N],
        Greek::Xi => &[Sound::Zh],
        Greek::Omicron => &[Sound::O],
        Greek::Pi => &[Sound::P],
        Greek::Rho => &[Sound::R],
        Greek::Sigma => &[Sound::S],
        Greek::Tau => &[Sound::T],
        Greek::Upsilon => &[Sound::U],
        Greek::Phi => &[Sound::F],
        Greek::Chi => &[Sound::H],
        Greek::Psi => &[Sound::Sh],
        Greek::Omega => &[Sound::Ox],
        Greek::AlphaAcute => &[Sound::J, Sound::A],
        Greek::EpsilonAcute => &[Sound::J, Sound::E],
        Greek::IotaAcute => &[Sound::I],
        Greek::OmicronAcute => &[Sound::J, Sound::O],
        Greek::UpsilonAcute => &[Sound::J, Sound::U],
        Greek::OmegaAcute => &[Sound::J, Sound::Ox],
        Greek::Acute => &[Sound::J],
        Greek::Break => &[],
        Greek::EtaAcute => &[Sound::J, Sound::Ex],
    }
}

struct ParseGrResult {
    result: Vec<Greek>,
    consumed: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseOfResult {
    result: Vec<Sound>,
    consumed: usize,
}

// impl ParseGrResult {
//     fn is_empty(&self) -> bool {
//         self.consumed == 0
//     }
// }

fn greek_vec_to_sound(input_initial: &[Greek]) -> ParseOfResult {
    let mut result = vec![];
    use Greek::*;
    use Sound::*;
    let mut i = 0;
    while i < input_initial.len() {
        let input = &input_initial[i..];
        let c0 = input[0];
        if c0 == Break {
            i += 1;
            continue;
        }

        let mut softened_count = 0;
        let mut was_rho = false;
        while softened_count < input.len() {
            let isc = input[softened_count];
            if !isc.can_be_softened() {
                break;
            }
            if isc != Rho && was_rho {
                softened_count = 0; // no softening for you
                break;
            }
            if isc == Rho {
                was_rho = true;
            }
            if softened_count > 0 && was_rho {
                break;
            }
            softened_count += 1;
        }

        if softened_count > 0
            && input.len() > softened_count
            && input[softened_count].is_softening()
        {
            for j in 0..softened_count {
                let q = match input[j] {
                    Rho => Rx,
                    Nu => Nx,
                    Zeta => Zx,
                    Sigma => Sx,
                    Delta => Dx,
                    Tau => Tx,
                    Lambda => L,
                    _ => panic!("nie zmiękczalny :("),
                };
                result.push(q);
                i += 1;
            }

            if input[softened_count] == Acute {
                // not in result
                i += 1
            } else {
                let c = match input[softened_count] {
                    Greek::AlphaAcute => A,
                    Greek::EpsilonAcute => E,
                    Greek::EtaAcute => Ex,
                    Greek::IotaAcute => {
                        if c0 == Rho {
                            Y
                        } else {
                            I
                        }
                    } // na pewno
                    Greek::OmicronAcute => O,
                    Greek::UpsilonAcute => U,
                    Greek::OmegaAcute => Ox,
                    _ => panic!("no soft :("),
                };
                if c == O && input.get(softened_count + 1) == Some(&Upsilon) {
                    result.push(Ou);
                    i += 2;
                } else {
                    result.push(c);
                    i += 1;
                }
            }

            continue;
        }

        ///// DWUZNAKI
        if input.len() > 1 {
            let c1 = input[1];
            if c0 == Tau && c1 == Sigma {
                i += 2;
                result.push(C);
                continue;
            }
            if c0 == Delta && c1 == Zeta {
                i += 2;
                result.push(Dz);
                continue;
            }
            if c0 == Delta && c1 == Xi {
                i += 2;
                result.push(Dh);
                continue;
            }
            if c0 == Mu && c1 == Pi {
                i += 2;
                result.push(B);
                continue;
            }
            if c0 == Omicron && c1 == Upsilon {
                i += 2;
                result.push(Ou);
                continue;
            }
            if c0 == OmicronAcute && c1 == Upsilon {
                i += 2;
                result.push(J);
                result.push(Ou);
                continue;
            }
        }
        // EODWUZNAKI

        let snd = naive_greek_to_sound(c0);
        result.extend(snd);
        i += 1
    }

    ParseOfResult {
        result,
        consumed: i,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum TextRepr {
    Arbitrary(String),
    Word(Vec<Greek>, CapitalisationMode),
}

#[derive(PartialEq, Eq, Debug)]
pub struct GreekText {
    pub parts: Vec<TextRepr>,
}

pub fn utf8_greek_to_text(input: &str) -> super::official::Text {
    let greek_text = utf8_to_greek(input);
    let prepared = greek_text
        .parts
        .iter()
        .map(|part| match part {
            TextRepr::Arbitrary(arbitrary) => {
                super::official::TextRepr::Arbitrary(arbitrary.clone())
            }
            TextRepr::Word(word, orig) => {
                let parse_result = greek_vec_to_sound(word);
                // TODO ignore count - sanity check
                if parse_result.consumed != word.len() {
                    panic!("Not all parsed")
                }
                super::official::TextRepr::Word(parse_result.result, orig.clone())
            }
        })
        .collect::<Vec<super::official::TextRepr>>();

    super::official::Text { parts: prepared }
}

fn utf8_to_greek(input: &str) -> GreekText {
    let case_preserving_input = input.chars().collect::<Vec<char>>();
    let chars_input: Vec<char> = input.to_lowercase().chars().collect();

    let mut parts = vec![];
    let mut i = 0;

    while i < chars_input.len() {
        let chars = &chars_input[i..];
        if chars.is_empty() {
            break;
        }

        let cr = consume_utf8_word(chars);
        if cr.consumed > 0 {
            parts.push(TextRepr::Word(cr.result, super::CapitalisationMode::detect(&case_preserving_input[i..i + cr.consumed])));
            i += cr.consumed;
            continue;
        }

        let mut j = 0;
        while j < chars.len() && char_to_greek(chars[j]) == None {
            j += 1;
        }

        if j > 0 {
            parts.push(TextRepr::Arbitrary(chars[..j].iter().collect()));
            i += j;
            continue;
        }

        print!("Failed to parse at index {}: '{}'", i, chars[0]);
        break;
    }

    GreekText { parts }
}

fn consume_utf8_word(input: &[char]) -> ParseGrResult {
    let mut i = 0;
    let mut result = Vec::new();
    while i < input.len() {
        match char_to_greek(input[i]) {
            Some(g) => {
                result.push(g);
                i += 1;
            }
            None => break,
        }
    }
    ParseGrResult {
        result,
        consumed: i,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gr_vec_to_sound_vec() {
        use Greek::*;
        use Sound::*;
        let r: ParseOfResult = greek_vec_to_sound(&vec![Delta, Omicron, Mu, Pi, Rho, Omicron]);
        assert_eq!(
            r,
            ParseOfResult {
                result: vec![D, O, B, R, O],
                consumed: 6
            }
        );

        let r = greek_vec_to_sound(&vec![
            Delta, AlphaAcute, Mu, Acute, Rho, IotaAcute, Nu, IotaAcute,
        ]);
        assert_eq!(
            r,
            ParseOfResult {
                result: vec![Dx, A, M, J, Rx, Y, Nx, I],
                consumed: 8
            }
        );

        let r = greek_vec_to_sound(&vec![Mu, Alpha, Tau, Sigma, Kappa, IotaAcute]);
        assert_eq!(
            r,
            ParseOfResult {
                result: vec![M, A, C, K, I],
                consumed: 6
            }
        );

        let q = vec![
            Greek::Tau,
            Greek::EpsilonAcute,
            Greek::Pi,
            Greek::Lambda,
            Greek::UpsilonAcute,
            Greek::Tau,
            Greek::Kappa,
            Greek::Omicron,
        ];
        let r = greek_vec_to_sound(&q);
        assert_eq!(
            r,
            ParseOfResult {
                result: vec![Tx, E, P, L, U, T, K, O],
                consumed: 8
            }
        );

        let q = vec![
            Greek::Rho,
            Greek::Alpha,
            Greek::Delta,
            Greek::Omicron,
            Greek::Sigma,
            Greek::Tau,
            Greek::Acute,
        ];
        let r = greek_vec_to_sound(&q);
        assert_eq!(
            r,
            ParseOfResult {
                result: vec![R, A, D, O, Sx, Tx],
                consumed: 7
            }
        );

        let q = vec![
            Greek::Mu,
            Greek::Pi,
            Greek::Alpha,
            Greek::Rho,
            Greek::Delta,
            Greek::EpsilonAcute,
            Greek::Acute,
        ];
        let r = greek_vec_to_sound(&q);
        assert_eq!(
            r,
            ParseOfResult {
                result: vec![B, A, R, Dx, E, J],
                consumed: 7
            }
        );

        let q = vec![Mu, OmicronAcute, Upsilon, Delta];
        let r = greek_vec_to_sound(&q);
        assert_eq!(
            r,
            ParseOfResult {
                result: vec![M, J, Ou, D],
                consumed: 4
            }
        );
    }

    #[test]
    fn utf8_do_gr() {
        let res = utf8_to_greek("ποζδραβάμ τέπλύτκο! :)");
        assert_eq!(res.parts.len(), 4);
        assert_eq!(
            res.parts[0],
            TextRepr::Word(vec![
                Greek::Pi,
                Greek::Omicron,
                Greek::Zeta,
                Greek::Delta,
                Greek::Rho,
                Greek::Alpha,
                Greek::Beta,
                Greek::AlphaAcute,
                Greek::Mu
            ], CapitalisationMode::Lowercase)
        );
        assert_eq!(res.parts[1], TextRepr::Arbitrary(" ".into()));
        assert_eq!(
            res.parts[2],
            TextRepr::Word(vec![
                Greek::Tau,
                Greek::EpsilonAcute,
                Greek::Pi,
                Greek::Lambda,
                Greek::UpsilonAcute,
                Greek::Tau,
                Greek::Kappa,
                Greek::Omicron
            ], CapitalisationMode::Lowercase)
        );
        assert_eq!(res.parts[3], TextRepr::Arbitrary("! :)".into()));

        let res = utf8_to_greek("ραδοστ'");
        assert_eq!(res.parts.len(), 1);
        assert_eq!(
            res.parts[0],
            TextRepr::Word(vec![
                Greek::Rho,
                Greek::Alpha,
                Greek::Delta,
                Greek::Omicron,
                Greek::Sigma,
                Greek::Tau,
                Greek::Acute,
            ], CapitalisationMode::Lowercase)
        );
    }

    #[test]
    fn test_to_greek() {
        let input = vec![Sound::C];
        let result = consume_greek(&input);
        assert_eq!(result.result, vec![Greek::Tau, Greek::Sigma]);
        assert_eq!(result.consumed, 1);

        let input = vec![Sound::Tx];
        let result = consume_greek(&input);
        assert_eq!(result.result, vec![Greek::Tau, Greek::Acute]);
        assert_eq!(result.consumed, 1);

        let input = vec![Sound::Ox];
        let result = consume_greek(&input);
        assert_eq!(result.result, vec![Greek::Omega]);
        assert_eq!(result.consumed, 1);

        let input = vec![Sound::Sx, Sound::A];
        let result = consume_greek(&input);
        assert_eq!(result.result, vec![Greek::Sigma, Greek::AlphaAcute]);
        assert_eq!(result.consumed, 2);
    }

    #[test]
    fn test_job() {
        use super::Sound::*;
        let input = vec![C, A, Lx, O, Sx, Tx];
        let result = to_greek(&input);
        assert_eq!(result, "τσαλοστ'");

        let input = vec![R, A, D, O, Sx, Tx, I];
        let result = to_greek(&input);
        assert_eq!(result, "ραδοστί");

        assert_eq!(to_greek(&vec![L, I, T, W, O]), "λίτβο");
        assert_eq!(to_greek(&vec![O, J, Ch, Y, Z, N, O]), "ο'θιζνο");
        assert_eq!(to_greek(&vec![M, O, J, A]), "μοά");
        assert_eq!(to_greek(&vec![T, Y]), "τι");
        assert_eq!(to_greek(&vec![J, E, S, T, E, Sx]), "έστεσ'");
        assert_eq!(to_greek(&vec![J, A, K]), "άκ");
        assert_eq!(to_greek(&vec![Z, D, R, O, W, J, E]), "ζδροβέ");

        assert_eq!(to_greek(&vec![I, L, E]), "ίλέ");
        assert_eq!(to_greek(&vec![Tx, Ex]), "τή");
        assert_eq!(to_greek(&vec![T, Rx, E, B, A]), "τρέμπα");
        assert_eq!(to_greek(&vec![C, E, Nx, I, Tx]), "τσενίτ'");
        assert_eq!(to_greek(&vec![T, E, N]), "τεν");
        assert_eq!(to_greek(&vec![T, Y, L, K, O]), "τιλ'κο");
        assert_eq!(to_greek(&vec![Sx, Ex]), "σή");
        assert_eq!(to_greek(&vec![D, O, W, J, E]), "δοβέ");
        assert_eq!(to_greek(&vec![C, O]), "τσο");
        assert_eq!(to_greek(&vec![Tx, Ex]), "τή");
        assert_eq!(to_greek(&vec![S, T, R, A, Tx, I, Lx]), "στρατίλ");
        assert_eq!(to_greek(&vec![M, J, U, D]), "μύδ");
        assert_eq!(to_greek(&vec![M, J, Ou, D]), "μόυδ");
    }

    //  #[test]
    // fn test_isol() {
    //     use super::Sound::*;

    // assert_eq!( to_greek(&vec![J,A,K]), "άκ");
    // }
}
