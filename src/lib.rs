pub struct LangTag<T> {
	p: ParsingData,
	data: T
}

impl<T: AsRef<[u8]>> LangTag<T> {
	pub fn new(data: T) -> Result<LangTag<T>, ParseError> {
		let p = ParsingData::new(&data)?;
		Ok(LangTag {
			p,
			data
		})
	}
}

pub enum ParseError {
	// ...
}

pub struct ParsingData {
	// ...
}

impl ParsingData {
	pub fn new<T: AsRef<[u8]>>(data: &T) -> Result<ParsingData, ParseError> {
		Ok(ParsingData {
			// ...
		})
	}
}