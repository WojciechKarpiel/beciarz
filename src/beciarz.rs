pub fn official_to_greek(input: &str) -> String {
    let text = official::parse(input);
    text.parts
        .iter()
        .map(|part| match part {
            official::TextRepr::Word(sounds) => greek::to_greek(sounds),
            official::TextRepr::Arbitrary(text) => text.clone(), // todo noclone
        })
        .collect::<Vec<_>>()
        .join("")
}

mod test {

    #[test]
    fn test_official_to_greek() {
        let input = "Litwo, ojczyzno moja! Ty jesteś jak zdrowie! Ile cię trzeba cenić, ten tylko się dowie, kto cię stracił.";
        let text = super::official_to_greek(input);
        assert_eq!(
            text,
            "λίτβο, ο'θιζνο μοά! τι έστεσ' άκ ζδροβέ! ίλέ τή τρέμπα τσενίτ', τεν τιλ'κο σή δοβέ, κτο τή στρατίλ."
        );

        assert_eq!("μέ'", super::official_to_greek("miej"));
        assert_eq!("έ'", super::official_to_greek("jej"));
        assert_eq!("δ\\άγνοστικα", super::official_to_greek("diagnostyka"));
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Sound {
    A,
    B,
    C,
    Ch,
    D,
    Dx,
    Dz,
    Dh,
    E,
    Ex,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    Lx,
    M,
    N,
    Nx,
    O,
    Ox,
    P,
    R,
    Rx, // nie prawda
    S,
    Sx,
    Sh,
    T,
    Tx,
    U,
    W,
    Y,
    Z,
    Zx,
    Zh,
}

impl Sound {
    fn is_softened(self) -> bool {
        match self {
            Sound::I
            | Sound::Sx
            | Sound::Zx
            | Sound::L
            | Sound::Nx
            | Sound::Tx
            | Sound::Rx
            | Sound::Dx => true,
            _ => false,
        }
    }

    fn is_vowel(self) -> bool {
        match self {
            Sound::A
            | Sound::E
            | Sound::Ex
            | Sound::I
            | Sound::Y
            | Sound::O
            | Sound::Ox
            | Sound::U => true,
            _ => false,
        }
    }

    // fn transferable_softening(self) -> bool {
    //     // Chyba nie L
    //     return self.is_softened();
    // }
}

mod greek {
    struct ConsumeResult {
        result: Vec<Greek>,
        consumed: usize,
    }

    use super::Sound;

    pub fn do_the_job(input: &[Sound]) -> Vec<Greek> {
        let mut result = vec![];
        let mut i = 0;

        while i < input.len() {
            let consume_result = consume_greek(&input[i..]);
            result.extend(consume_result.result);
            i += consume_result.consumed;
            // println!(
            //     "Consumed {} sounds, result so far: {:?}",
            //     consume_result.consumed, result
            // );
        }

        result
    }

    pub fn to_greek(input: &Vec<Sound>) -> String {
        let greek_chars = do_the_job(input);
        greek_chars.iter().map(|&g| to_char(g)).collect()
    }

    fn consume_naive(input: Sound) -> Vec<Greek> {
        use Greek::*;
        match input {
            Sound::A => vec![Alpha],
            Sound::B => vec![Mu, Pi],
            Sound::C => vec![Tau, Sigma],
            Sound::Ch => vec![Theta],
            Sound::D => vec![Delta],
            Sound::Dx => vec![Delta, Acute],
            Sound::E => vec![Epsilon],
            Sound::Ex => vec![Eta],
            Sound::F => vec![Phi],
            Sound::G => vec![Gamma],
            Sound::H => vec![Chi],
            Sound::I => vec![IotaAcute],
            Sound::J => vec![Acute],
            Sound::K => vec![Kappa],
            Sound::L => vec![Lambda, Acute],
            Sound::Lx => vec![Lambda],
            Sound::M => vec![Mu],
            Sound::N => vec![Nu],
            Sound::Nx => vec![Nu, Acute],
            Sound::O => vec![Omicron],
            Sound::Ox => vec![Omega],
            Sound::P => vec![Pi],
            Sound::R => vec![Rho],
            Sound::S => vec![Sigma],
            Sound::Sx => vec![Sigma, Acute],
            Sound::Sh => vec![Psi],
            Sound::T => vec![Tau],
            Sound::Tx => vec![Tau, Acute],
            Sound::U => vec![Upsilon],
            Sound::W => vec![Beta],
            Sound::Y => vec![Iota],
            Sound::Z => vec![Zeta],
            Sound::Zx => vec![Zeta, Acute],
            Sound::Zh => vec![Xi],
            Sound::Rx => vec![Rho, Acute],
            Sound::Dz => vec![Delta, Zeta],
            Sound::Dh => vec![Delta, Xi],
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

    fn soften_vowel(sound: Sound) -> Greek {
        use Greek::*;
        use Sound::*;
        match sound {
            A => AlphaAcute,
            E => EpsilonAcute,
            Ex => EtaAcute,
            I => IotaAcute,
            O => OmicronAcute,
            Ox => OmegaAcute,
            U => UpsilonAcute,
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
                result: consume_naive(i0),
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
            // println!("Softened count: {}", softened_count);
            let following_vowel = input.get(softened_count).cloned().filter(|x| x.is_vowel());
            let total_consumed = softened_count + following_vowel.map_or(0, |_| 1);

            let last_greek: Greek = match following_vowel {
                Some(vowel) => soften_vowel(vowel), // todo what if not-softened
                None => Greek::Acute,
            };

            let mut result = input[..softened_count]
                .iter()
                .map(|&s| softened_sound_to_base_greek(s))
                .collect::<Vec<Greek>>();

            result.push(last_greek);

            return ConsumeResult {
                result,
                consumed: total_consumed,
            };
        }

        use Sound::*;
        if i0 == Sound::J {
            let i1 = input.get(1).unwrap().clone();
            if i1 == E || i1 == A || i1 == Ex || i1 == Ox || i1 == U {
                return ConsumeResult {
                    result: vec![soften_vowel(i1)],
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

        // print!("FALLBACK TO NAIVE CONSUME: {:?}", i0);
        // check for break
        let i1 = input.get(1).unwrap().clone();
        if i1 == J || i1 == I { //|| i1 == Nx || i1 == Zx || i1 == Sx || i1 == Tx  || i1 == Dx ||i1==Rx{
            if i0 == N || i0 ==Z || i0 == S || i0 == T || i0==D || i0 ==R{
                // println!("BREAKING: {:?}", i0);
                let mut r0=consume_naive(i0);
                r0.push(Greek::Break);
                return ConsumeResult {
                    result: r0,
                    consumed: 1,
                };
            }
        }

        return ConsumeResult {
            result: consume_naive(i0),
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
        Break, // TODO wsparcie później
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

    #[cfg(test)]
    mod tests {
        use super::*;

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
        }

        //  #[test]
        // fn test_isol() {
        //     use super::Sound::*;

        // assert_eq!( to_greek(&vec![J,A,K]), "άκ");
        // }
    }
}

mod official {
    use super::Sound;
    use super::Sound::*;

    pub fn parse(input_: &str) -> Text {
        let input = input_.to_lowercase(); // TODO handle uppercase

        let mut parts = vec![];
        let mut i = 0;
        let charsi = input.chars().collect::<Vec<char>>();

        while i < charsi.len() {
            let chars = &charsi[i..];
            if chars.is_empty() {
                break;
            }
            let mut j = 0;
            while j < chars.len() && single_naive(chars[j]) == None {
                // println!("Skipping char: {}", charsi[i]);
                j += 1;
            }

            if j > 0 {
                // println!("Adding arbitrary chars: {}", &chars[..j]);
                parts.push(TextRepr::Arbitrary(chars[..j].iter().collect()));
                i += j;
                continue;
            }

            let cr = parse_word(chars);
            if cr.consumed > 0 {
                parts.push(TextRepr::Word(cr.result));
                i += cr.consumed;
                continue;
            }

            print!("Failed to parse at index {}: '{}'", i, chars[0]);
            break;
        }

        Text { parts }
    }

    #[derive(PartialEq, Eq, Debug)]
    pub enum TextRepr {
        Arbitrary(String),
        Word(Vec<Sound>),
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct Text {
        pub parts: Vec<TextRepr>,
    }

    struct ConsumeResult {
        result: Vec<Sound>,
        consumed: usize,
    }
    fn parse_word(input: &[char]) -> ConsumeResult {
        let charsi = input;
        let mut i = 0;
        let mut result = vec![];
        while i < charsi.len() {
            // println!("MAMY I: {} ", i);
            let chars = &charsi[i..];
            let cr = try_dzx(chars);
            if cr.consumed > 0 {
                result.extend(cr.result);
                i += cr.consumed;
                continue;
            }

            if chars.len() >= 2 && chars[0] == 'd' && chars[1] == 'ż' {
                result.push(Dh);
                i += 2;
                continue;
            }

            let cr = try_ci_si_zi(chars);
            if cr.consumed > 0 {
                result.extend(cr.result);
                i += cr.consumed;
                continue;
            }

            let cr = try_i_samogl(chars);
            if cr.consumed > 0 {
                result.extend(cr.result);
                i += cr.consumed;
                continue;
            }

            if let Some((dwuznak_sound, consumed)) = try_dwuznak(chars) {
                result.push(dwuznak_sound);
                i += consumed;
                continue;
            }

            if let Some(single_sound) = single_naive(charsi[i]) {
                result.push(single_sound);
                i += 1;
                continue;
            }
            // nothing found to parse
            break;
        }

        ConsumeResult {
            result: result,
            consumed: i,
        }
    }

    // UWAGA TO NIE ZADZIAŁA ZAWSZE
    fn try_i_samogl(input: &[char]) -> ConsumeResult {
        if input.len() < 2 || input[0] != 'i' {
            return ConsumeResult {
                result: vec![],
                consumed: 0,
            };
        } else {
            let i1 = input[1];
            match i1 {
                'a' => ConsumeResult {
                    result: vec![J, A],
                    consumed: 2,
                },
                'ą' => ConsumeResult {
                    result: vec![J, Ox],
                    consumed: 2,
                },
                'e' => ConsumeResult {
                    result: vec![J, E],
                    consumed: 2,
                },
                'ę' => ConsumeResult {
                    result: vec![J, Ex],
                    consumed: 2,
                },
                'o' => ConsumeResult {
                    result: vec![J, O],
                    consumed: 2,
                },
                'ó' => ConsumeResult {
                    result: vec![J, U],
                    consumed: 2,
                },
                'u' => ConsumeResult {
                    result: vec![J, U],
                    consumed: 2,
                },
                _ => ConsumeResult {
                    result: vec![],
                    consumed: 0,
                },
            }
        }
    }

    fn try_ci_si_zi(input: &[char]) -> ConsumeResult {
        if input.len() < 2 {
            return ConsumeResult {
                result: vec![],
                consumed: 0,
            };
        }

        let i0 = input[0];

        let okk = match i0 {
            'c' => Some(Tx),
            's' => Some(Sx),
            'z' => Some(Zx),
            'n' => Some(Nx),
            _ => None,
        };

        match okk {
            Some(init_sound) => {
                if input.len() == 2 {
                    if input[1] == 'i' {
                        ConsumeResult {
                            result: vec![init_sound, I],
                            consumed: 2,
                        }
                    } else {
                        ConsumeResult {
                            result: vec![],
                            consumed: 0,
                        }
                    }
                } else {
                    if input[1] == 'i' {
                        let i2 = input[2];

                        match i2 {
                            'a' => ConsumeResult {
                                result: vec![init_sound, Sound::A],
                                consumed: 3,
                            },
                            'ą' => ConsumeResult {
                                result: vec![init_sound, Sound::Ox],
                                consumed: 3,
                            },
                            'e' => ConsumeResult {
                                result: vec![init_sound, Sound::E],
                                consumed: 3,
                            },
                            'ę' => ConsumeResult {
                                result: vec![init_sound, Sound::Ex],
                                consumed: 3,
                            },
                            'o' => ConsumeResult {
                                result: vec![init_sound, Sound::O],
                                consumed: 3,
                            },
                            'ó' => ConsumeResult {
                                result: vec![init_sound, Sound::U],
                                consumed: 3,
                            },
                            'u' => ConsumeResult {
                                result: vec![init_sound, Sound::U],
                                consumed: 3,
                            },
                            _ => ConsumeResult {
                                result: vec![init_sound, Sound::I],
                                consumed: 2,
                            },
                        }
                    } else {
                        ConsumeResult {
                            result: vec![],
                            consumed: 0,
                        }
                    }
                }
            }
            None => ConsumeResult {
                result: vec![],
                consumed: 0,
            },
        }
    }

    fn try_dzx(input: &[char]) -> ConsumeResult {
        if input.len() < 3 {
            return ConsumeResult {
                result: vec![],
                consumed: 0,
            };
        }

        let i0 = input[0];
        if i0 != 'd' {
            return ConsumeResult {
                result: vec![],
                consumed: 0,
            };
        }
        let i1 = input[1];

        match (i0, i1) {
            ('d', 'z') => {
                if input.len() == 3 && input[2] == 'i' {
                    ConsumeResult {
                        result: vec![Sound::Dx, Sound::I],
                        consumed: 3,
                    }
                } else if input.len() > 3 {
                    let i2 = input[2];
                    let i3 = input[3];
                    if i2 != 'i' {
                        ConsumeResult {
                            result: vec![],
                            consumed: 0,
                        }
                    } else {
                        match i3 {
                            'a' => ConsumeResult {
                                result: vec![Sound::Dx, Sound::A],
                                consumed: 4,
                            },
                            'ą' => ConsumeResult {
                                result: vec![Sound::Dx, Sound::Ox],
                                consumed: 4,
                            },
                            'e' => ConsumeResult {
                                result: vec![Sound::Dx, Sound::E],
                                consumed: 4,
                            },
                            'ę' => ConsumeResult {
                                result: vec![Sound::Dx, Sound::Ex],
                                consumed: 4,
                            },
                            'o' => ConsumeResult {
                                result: vec![Sound::Dx, Sound::O],
                                consumed: 4,
                            },
                            'ó' => ConsumeResult {
                                result: vec![Sound::Dx, Sound::U],
                                consumed: 4,
                            },
                            'u' => ConsumeResult {
                                result: vec![Sound::Dx, Sound::U],
                                consumed: 4,
                            },
                            _ => ConsumeResult {
                                result: vec![Sound::Dx, Sound::I],
                                consumed: 3,
                            },
                        }
                    }
                } else {
                    ConsumeResult {
                        result: vec![],
                        consumed: 2,
                    }
                }
            }

            _ => ConsumeResult {
                result: vec![],
                consumed: 0,
            },
        }
    }

    fn try_dwuznak(input: &[char]) -> Option<(Sound, usize)> {
        if input.len() < 2 {
            return None;
        }

        let i0 = input[0];
        let i1 = input[1];

        match (i0, i1) {
            ('c', 'z') => Some((Ch, 2)),
            ('s', 'z') => Some((Sh, 2)),
            ('d', 'z') => Some((Dz, 2)),
            ('d', 'ź') => Some((Dx, 2)),
            ('r', 'z') => Some((Rx, 2)),
            ('l', 'x') => Some((Lx, 2)),
            _ => None,
        }
    }

    fn single_naive(c: char) -> Option<Sound> {
        match c {
            'a' => Some(A),
            'ą' => Some(Ox),
            'b' => Some(B),
            'c' => Some(C),
            'ć' => Some(Tx),
            'd' => Some(D),
            'e' => Some(E),
            'ę' => Some(Ex),
            'f' => Some(F),
            'g' => Some(G),
            'h' => Some(H),
            'i' => Some(I),
            'j' => Some(J),
            'k' => Some(K),
            'l' => Some(L),
            'ł' => Some(Lx),
            'm' => Some(M),
            'n' => Some(N),
            'o' => Some(O),
            'ó' => Some(U), // TODO wsparcie dla ou
            'p' => Some(P),
            'r' => Some(R),
            's' => Some(S),
            'ś' => Some(Sx),
            't' => Some(T),
            'u' => Some(U),
            'w' => Some(W),
            'y' => Some(Y),
            'z' => Some(Z),
            'ż' => Some(Zh),
            'ź' => Some(Zx),
            _ => None,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn wejście() {
            let input = "test";
            let result = parse_word(input.chars().collect::<Vec<_>>().as_slice());
            assert_eq!(result.result, vec![Sound::T, Sound::E, Sound::S, Sound::T]);
            assert_eq!(result.consumed, 4);

            let input = "działo";
            let result = parse_word(input.chars().collect::<Vec<_>>().as_slice());
            assert_eq!(
                result.result,
                vec![Sound::Dx, Sound::A, Sound::Lx, Sound::O]
            );
            assert_eq!(result.consumed, 6);

            let input = "działało";
            let result = parse_word(input.chars().collect::<Vec<_>>().as_slice());
            assert_eq!(
                result.result,
                vec![
                    Sound::Dx,
                    Sound::A,
                    Sound::Lx,
                    Sound::A,
                    Sound::Lx,
                    Sound::O
                ]
            );
            assert_eq!(result.consumed, 8);

            use super::Sound::*;
            let input = "ciaksiakizilni";
            let result = parse_word(input.chars().collect::<Vec<_>>().as_slice());
            assert_eq!(result.result, vec![Tx, A, K, Sx, A, K, I, Zx, I, L, Nx, I]);
            assert_eq!(result.consumed, 14);

            let input = "ojczyzno";
            let result = parse_word(input.chars().collect::<Vec<_>>().as_slice());
            assert_eq!(result.result, vec![O, J, Ch, Y, Z, N, O]);
            assert_eq!(result.consumed, 8);
        }

        #[test]
        fn caly() {
            let input = "ala ma \nkota!";
            let result = parse(input);
            assert_eq!(result.parts.len(), 6);
            assert_eq!(
                result.parts[0],
                TextRepr::Word(vec![Sound::A, Sound::L, Sound::A])
            );
            assert_eq!(result.parts[1], TextRepr::Arbitrary(" ".to_string()));
            assert_eq!(result.parts[2], TextRepr::Word(vec![Sound::M, Sound::A]));
            assert_eq!(result.parts[3], TextRepr::Arbitrary(" \n".to_string()));
            assert_eq!(
                result.parts[4],
                TextRepr::Word(vec![Sound::K, Sound::O, Sound::T, Sound::A])
            );
            assert_eq!(result.parts[5], TextRepr::Arbitrary("!".to_string()));
        }
    }
}
