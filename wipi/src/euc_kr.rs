mod data;

use alloc::{borrow::Cow, vec::Vec};

use self::data::{KS_X_1001_HANGUL, SYMBOLS};

/// Encodes text for the handset, which draws EUC-KR (with the UHC extension).
/// Hanja and other unmapped characters become `?` to keep the tables small.
pub fn encode(text: &str) -> Cow<'_, [u8]> {
    if text.is_ascii() {
        return Cow::Borrowed(text.as_bytes());
    }

    let mut encoded = Vec::with_capacity(text.len());
    for c in text.chars() {
        let code = c as u32;
        let wide = match code {
            0..0x80 => {
                encoded.push(code as u8);
                continue;
            }
            0xAC00..=0xD7A3 => Some(encode_hangul(code as u16)),
            _ => u16::try_from(code).ok().and_then(|code| {
                let index = SYMBOLS.binary_search_by_key(&code, |&(c, _)| c).ok()?;
                Some(SYMBOLS[index].1)
            }),
        };
        match wide {
            Some(wide) => encoded.extend_from_slice(&wide.to_be_bytes()),
            None => encoded.push(b'?'),
        }
    }

    Cow::Owned(encoded)
}

fn encode_hangul(code: u16) -> u16 {
    let (lead, trail) = match KS_X_1001_HANGUL.binary_search(&code) {
        // KS X 1001 lays its syllables out in code point order, 94 per row.
        Ok(index) => (0xB0 + index / 94, 0xA1 + index % 94),
        // UHC packs the remaining syllables, also in code point order, into
        // the cells KS X 1001 leaves unused: 178 per lead byte up to 0xA0
        // and 84 per lead byte after it.
        Err(preceding) => {
            let rank = (code - 0xAC00) as usize - preceding;
            let (lead, cell) = if rank < 32 * 178 {
                (0x81 + rank / 178, rank % 178)
            } else {
                (0xA1 + (rank - 32 * 178) / 84, (rank - 32 * 178) % 84)
            };
            let trail = match cell {
                0..26 => 0x41 + cell,
                26..52 => 0x61 + cell - 26,
                _ => 0x81 + cell - 52,
            };
            (lead, trail)
        }
    };

    (lead << 8 | trail) as u16
}

#[cfg(test)]
mod tests {
    use super::encode;

    #[test]
    fn matches_encoding_rs_except_hanja() {
        for c in (0x80..=0xFFFF).filter_map(char::from_u32) {
            let mut buf = [0; 4];
            let text = c.encode_utf8(&mut buf);
            let (expected, _, unmappable) = encoding_rs::EUC_KR.encode(text);
            let hanja = matches!(c as u32, 0x4E00..=0x9FFF | 0xF900..=0xFAFF);
            if unmappable || hanja {
                assert_eq!(&*encode(text), b"?", "{c:?}");
            } else {
                assert_eq!(encode(text), expected, "{c:?}");
            }
        }

        assert_eq!(&*encode("A가\u{1F600}"), [b'A', 0xB0, 0xA1, b'?']);
    }
}
