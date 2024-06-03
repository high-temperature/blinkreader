pub fn split_into_chunks(text: String, chunk_size: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut start = 0;
    let text_len = text.len();

    while start < text_len {
        let mut end = start;
        let mut char_count = 0;

        for (idx, c) in text[start..].char_indices() {
            if c == '\n' {
                end = start + idx + 1;
                break;
            }
            if char_count >= chunk_size {
                end = start + idx;
                break;
            }
            char_count += 1;
            end = start + idx + 1;
        }

        if end > text_len {
            end = text_len;
        }

        let chunk = text[start..end].to_string();
        chunks.push(chunk);
        start = end;
    }

    chunks
}