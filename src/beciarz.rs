mod greek;
mod official;

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

pub fn greek_to_official(input: &str) -> String {
    let text = greek::utf8_greek_to_text(input);

    text.parts
        .iter()
        .map(|part| match part {
            official::TextRepr::Word(sounds) => official::to_official_utf8(sounds),
            official::TextRepr::Arbitrary(text) => text.clone(), // todo noclone
        })
        .collect::<Vec<_>>()
        .join("")
}

mod test {
    #[test]
    fn official_greek_both_ways(){
        let official = "magia";
        let greek = "μαγά";
        assert_eq!(super::official_to_greek(official), greek);
        assert_eq!(super::greek_to_official(greek), official);
    }

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

        assert_eq!(super::official_to_greek("kiedy"), "κέδι");
        assert_eq!(super::official_to_greek("mięso"), "μήσο");
    }

    #[test]
    fn test_greek_to_official() {
        let input = "λίτβο, ο'θιζνο μοά! τι έστεσ' άκ ζδροβέ! ίλέ τή τρέμπα τσενίτ', τεν τιλ'κο σή δοβέ, κτο τή στρατίλ.";
        let expected = "litwo, ojczyzno moja! ty jesteś jak zdrowie! ile cię trzeba cenić, ten tylko się dowie, kto cię stracił.";
        let text = super::greek_to_official(input);
        assert_eq!(text, expected);

        let input = "ποζδραβάμ τέπλύτκο!";
        let expected = "pozdrawiam cieplutko!";
        let text = super::greek_to_official(input);
        assert_eq!(text, expected);

        assert_eq!(super::greek_to_official("Κέδι"), "kiedy");
        assert_eq!(super::greek_to_official("μήσο"), "mięso");
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
