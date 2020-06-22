#[derive(Debug, PartialEq)]
// We don't need to store the original DNA strand here. According
// to the spec, this structure has the only purpose: RNA transcription.
pub struct DNA(RNA);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        // We perform the actual RNA transcription in constructor
        // to make method `into_rna` as efficient as possible.
        // We validate and transcript at the same pass.
        let rna = dna
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                'G' => Ok('C'),
                'C' => Ok('G'),
                'T' => Ok('A'),
                'A' => Ok('U'),
                _ => Err(i),
            })
            .collect::<Result<String, usize>>()?;

        Ok(DNA(RNA(rna)))
    }

    pub fn into_rna(self) -> RNA {
        self.0
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        if let Some(i) = rna
            .chars()
            .position(|c| c != 'A' && c != 'C' && c != 'G' && c != 'U')
        {
            Err(i)
        } else {
            Ok(RNA(rna.to_string()))
        }
    }
}
