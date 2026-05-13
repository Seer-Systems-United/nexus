use crate::poll::response::unit::Unit;

pub(super) fn split_response_line(line: &str) -> (Option<&str>, Vec<(u16, Unit)>) {
    let mut first_cell_start = None;
    let mut values = Vec::with_capacity(16);
    let mut offset = 0;

    for token in line.split_whitespace() {
        let start = offset
            + line[offset..]
                .find(token)
                .expect("split_whitespace token exists in line");
        offset = start + token.len();

        if let Some(cell) = parse_cell(token) {
            first_cell_start.get_or_insert(start);
            values.push(cell);
        }
    }

    let answer = first_cell_start
        .map(|start| line[..start].trim())
        .filter(|answer| !answer.is_empty())
        .or_else(|| {
            if values.is_empty() {
                Some(line.trim()).filter(|answer| !answer.is_empty())
            } else {
                None
            }
        });

    (answer, values)
}

fn parse_cell(token: &str) -> Option<(u16, Unit)> {
    if let Some(value) = token.strip_suffix('%') {
        if value.is_empty() {
            return None;
        }

        return Some((parse_u16(value)?, Unit::Percent));
    }

    None
}

fn parse_u16(value: &str) -> Option<u16> {
    let mut parsed = 0u16;

    for byte in value.bytes() {
        if !byte.is_ascii_digit() {
            return None;
        }

        parsed = parsed
            .checked_mul(10)?
            .checked_add(u16::from(byte - b'0'))?;
    }

    Some(parsed)
}
