use super::Sound;
use super::Sound::*;

fn naive_to_string(s: Sound) -> &'static str {
    use Sound::*;
    let qq = match s {
        A => "a",
        B => "b",
        C => "c",
        D => "d",
        E => "e",
        F => "f",
        G => "g",
        H => "ch", // bo częstsze
        I => "i",
        J => "j",
        K => "k",
        L => "l",
        Lx => "ł",
        M => "m",
        N => "n",
        Nx => "ń",
        O => "o",
        Ou => "ó",
        Ox => "ą",
        P => "p",
        R => "r",
        Rx => "rz",
        S => "s",
        Sx => "ś",
        Sh => "sz",
        T => "t",
        Tx => "ć",
        U => "u",
        W => "w",
        Y => "y",
        Z => "z",
        Zx => "ź",
        Zh => "ż",
        Ch => "cz",
        Dz => "dz",
        Dx => "dź",
        Dh => "dż",
        Ex => "ę",
    };

    qq
}

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
            j += 1;
        }

        if j > 0 {
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
                result: vec![J, Ou],
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
                            result: vec![init_sound, Sound::Ou],
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
                            result: vec![Sound::Dx, Sound::Ou],
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
                    consumed: 0,
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
        ('c', 'h') => Some((H, 2)),
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
        'ń' => Some(Nx),
        'o' => Some(O),
        'ó' => Some(Ou),
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

pub fn to_official_utf8(input_initial: &[Sound]) -> String {
    let mut res: String = String::new();
    let mut i = 0;

    while i < input_initial.len() {
        let input = &input_initial[i..];
        let c0 = input[0];

        if c0 == Rx {
            res.push_str("rz");
            i += 1;
            continue;
        }

        if input.len() > 1 {
            // ći -> ci; śe -> się
            let i1 = input[1];
            if (c0.is_softened())
                && c0 != I
                && c0 != L
                && (i1 == A || i1 == Ox || i1 == E || i1 == Ex || i1 == O || i1 == Ou || i1 == U || i1 == I)
            {
                // if c0 != Rx {
                match c0 {
                    Sound::Sx => res.push('s'),
                    Sound::Zx => res.push('z'),
                    Sound::Nx => res.push('n'),
                    Sound::Tx => res.push('c'),
                    Sound::Dx => {
                        res.push('d');
                        res.push('z');
                    }
                    other => panic!("ni wim co to: {:?}", other),
                }
                i += 1;
                if i1 != I {
                    res.push('i');
                }
                continue;
            }

            // wje ->wie
            if (c0 == W || c0 == K || c0 == M || c0 == G) && i1 == J && input.len() > 2 {
                let i2 = input[2];
                if i2 == A || i2 == Ox || i2 == E || i2 == Ex || i2 == O  || i2 == Ou|| i2 == U {
                    if c0 == W {
                        res.push('w');
                    } else if c0 == K {
                        res.push('k');
                    } else if c0 == M {
                        res.push('m');
                    } else if c0 == G {
                        res.push('g');
                    } else {
                        panic!("Unexpected sound: {:?}", c0);
                    }
                    i += 1; // consume W/K
                    res.push('i');
                    i += 1; //consuje J
                    continue; // handle the rest separately
                }
            }
        }

        res.push_str(naive_to_string(c0));
        i += 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snd_to_string() {
        use Sound::*;
        let input = vec![K, O, P, Y, T, K, O];
        assert_eq!(to_official_utf8(&input), "kopytko");
        let input = vec![Ch, A, H, A];
        assert_eq!(to_official_utf8(&input), "czacha"); // TODO jak ogarnąć "CH?"
        let input = vec![L, I, T, W, O];
        assert_eq!(to_official_utf8(&input), "litwo");
        let input = vec![Tx, E, P, L, U, T, K, O];
        assert_eq!(to_official_utf8(&input), "cieplutko");
        let input = vec![Z, D, R, O, W, J, E];
        assert_eq!(to_official_utf8(&input), "zdrowie");
        let input = vec![M, J, Ou, D];
        assert_eq!(to_official_utf8(&input), "miód");
    }

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

        let input = "chmura";
        let result = parse_word(input.chars().collect::<Vec<_>>().as_slice());
        assert_eq!(result.result, vec![H, M, U, R, A]);
        assert_eq!(result.consumed, 6);

        let input = "dzw";
        let result = parse_word(input.chars().collect::<Vec<_>>().as_slice());
        assert_eq!(result.result, vec![Dz, W]);
        assert_eq!(result.consumed, 3);

        let input = "dzwo";
        let result = parse_word(input.chars().collect::<Vec<_>>().as_slice());
        assert_eq!(result.result, vec![Dz, W, O]);
        assert_eq!(result.consumed, 4);
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
