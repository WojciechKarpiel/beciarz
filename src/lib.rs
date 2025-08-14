#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Sound {
    A,
    B,
    C,
    Ch,
    D,
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
            Sound::I | Sound::Sx | Sound::Zx | Sound::L | Sound::Nx | Sound::Tx| Sound::Rx => true,
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

    fn transferable_softening(self) -> bool {
        // Chyba nie L
        return self.is_softened();
    }
}

mod greek {
    struct ConsumeResult {
        result: Vec<Greek>,
        consumed: usize,
    }

    use super::Sound;

    fn do_the_job(input: &[Sound]) -> Vec<Greek> {
        let mut result = vec![];
        let mut i = 0;

        while i < input.len() {
            let consume_result = consume_greek(&input[i..]);
            result.extend(consume_result.result);
            i += consume_result.consumed;
            println!(
                "Consumed {} sounds, result so far: {:?}",
                consume_result.consumed,
                result
            );
        }

        result
    }

    fn to_greek(input: &Vec<Sound>) -> String {
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
        while softened_count < input.len() && input[softened_count].is_softened() && input[softened_count] != Sound::I {
            softened_count += 1;
        }

        if softened_count > 0 {
            println!("Softened count: {}", softened_count);
            let following_vowel = input.get(softened_count).cloned().filter(|x| x.is_vowel());
            let total_consumed = softened_count + following_vowel.map_or(0, |_| 1);


            let last_greek:Greek = match following_vowel {
                Some(vowel) => soften_vowel( vowel), // todo what if not-softened
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
            if i1 == E || i1 == A || i1 == Ex  || i1==Ox || i1==U {
                return ConsumeResult {
                    result: vec![soften_vowel(i1)],
                    consumed: 2,
                };
                // TODO Case for I Y 
            }else {
                return ConsumeResult {
                    result: vec![Greek::Acute],
                    consumed: 1,
                };
            }
        }


        // print!("FALLBACK TO NAIVE CONSUME: {:?}", i0);
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
        Break,
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

            let input = vec![R, A, D, O, Sx, Tx,I];
            let result = to_greek(&input);
            assert_eq!(result, "ραδοστί");


            assert_eq!( to_greek(&vec![L,I,T,W,O]), "λίτβο");
            assert_eq!( to_greek(& vec![O,J,Ch,Y,Z,N,O]), "ο'θιζνο");
            assert_eq!( to_greek(& vec![M,O,J,A]), "μοά");
            assert_eq!( to_greek(& vec![T,Y]), "τι");
            assert_eq!( to_greek(& vec![J,E,S,T,E,Sx]), "έστεσ'");
            assert_eq!( to_greek(&vec![J,A,K]), "άκ");
            assert_eq!( to_greek(& vec![Z,D,R,O,W,J,E]), "ζδροβέ");

            assert_eq!( to_greek(& vec![I,L,E]), "ίλέ");
            assert_eq!( to_greek(& vec![Tx,Ex]), "τή");
            assert_eq!( to_greek(& vec![T,Rx,E,B,A]), "τρέμπα");
            assert_eq!( to_greek(& vec![C,E,Nx,I,Tx]), "τσενίτ'");
            assert_eq!( to_greek(& vec![T,E,N]), "τεν");
            assert_eq!( to_greek(& vec![T,Y,L,K,O]), "τιλ'κο");
            assert_eq!( to_greek(& vec![Sx,Ex]), "σή");
            assert_eq!( to_greek(& vec![D,O,W,J,E]), "δοβέ");
            assert_eq!( to_greek(& vec![C,O]), "τσο");
            assert_eq!( to_greek(& vec![Tx,Ex]), "τή");
            assert_eq!( to_greek(& vec![S,T,R,A,Tx,I,Lx]), "στρατίλ");
            
        }

        //  #[test]
        // fn test_isol() {
        //     use super::Sound::*;

        // assert_eq!( to_greek(&vec![J,A,K]), "άκ");
        // }
    }
}
