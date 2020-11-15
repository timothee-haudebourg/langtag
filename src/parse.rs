use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
	EmptyPrivateUse,
	InvalidLanguage,
	MalformedLangTag
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::EmptyPrivateUse => write!(f, "empty private use tag"),
			Error::InvalidLanguage => write!(f, "invalid primary language"),
			Error::MalformedLangTag => write!(f, "malformed lang tag")
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ParsedLangTag {
	pub language_end: usize,
	pub script_end: usize,
	pub region_end: usize,
	pub variant_end: usize,
	pub extension_end: usize,
	pub privateuse_end: usize
}

impl ParsedLangTag {
	pub fn len(&self) -> usize {
		self.privateuse_end
	}
}

/// Parse a <langtag> production.
pub fn langtag(data: &[u8], i: usize) -> Result<ParsedLangTag, Error> {
	let language_end = language(data, i);

	if language_end == i {
		return Err(Error::InvalidLanguage)
	}

	let mut script_end = language_end;
	if separator(data, script_end) {
		let j = script(data, script_end+1);
		if j > script_end+1 {
			script_end = j
		}
	}

	let mut region_end = script_end;
	if separator(data, region_end) {
		let j = region(data, region_end+1);
		if j > region_end+1 {
			region_end = j
		}
	}

	let mut variant_end = region_end;
	if separator(data, variant_end) {
		let j = variants(data, variant_end+1);
		if j > variant_end+1 {
			variant_end = j
		}
	}

	let mut extension_end = variant_end;
	if separator(data, extension_end) {
		let j = extensions(data, extension_end+1);
		if j > extension_end+1 {
			extension_end = j
		}
	}

	let mut privateuse_end = extension_end;
	if separator(data, privateuse_end) {
		let j = privateuse(data, privateuse_end+1);
		if j > privateuse_end+1 {
			privateuse_end = j
		}
	}

	Ok(ParsedLangTag {
		language_end,
		script_end,
		region_end,
		variant_end,
		extension_end,
		privateuse_end
	})
}

/// Parse a <language> production.
pub fn language(data: &[u8], i: usize) -> usize {
	let primary_end = primary_language(data, i);

	if primary_end < i+4 {
		// sometimes followed by extended language subtags
		if separator(data, primary_end) {
			let j = extlang(data, primary_end+1);
			if j > primary_end+1 {
				return j
			}
		}
	}

	primary_end
}

pub fn primary_language(data: &[u8], mut i: usize) -> usize {
	let s = i;

	// shortest ISO 639 code
	if alpha(data, i) && alpha(data, i+1) {
		let mut j = i+2;

		if alpha(data, j) {
			j += 1
		}

		if wordsep(data, j) {
			i = j
		} else {
			// or reserved for future use, or registered language subtag.
			while j < s+8 && alpha(data, j) {
				j += 1
			}

			if wordsep(data, j) {
				i = j
			}
		}
	}

	i
}

pub fn extlang(data: &[u8], mut i: usize) -> usize {
	// selected ISO 639 codes
	let j = extlang_tag(data, i);
	if j > i {
		i = j;

		if separator(data, i) {
			let j = extlang_tag(data, i+1);
			if j > i+1 {
				i = j;

				if separator(data, i) {
					let j = extlang_tag(data, i+1);
					if j > i+1 {
						i = j;
					}
				}
			}
		}
	}

	i
}

pub fn extlang_tag(data: &[u8], mut i: usize) -> usize {
	if alpha(data, i) && alpha(data, i+1) && alpha(data, i+2) && wordsep(data, i+3) {
		i += 3;
	}

	i
}

pub fn script(data: &[u8], i: usize) -> usize {
	if alpha(data, i) && alpha(data, i+1) && alpha(data, i+2) && alpha(data, i+3) && wordsep(data, i+4) {
		i + 4
	} else {
		i
	}
}

pub fn region(data: &[u8], mut i: usize) -> usize {
	if alpha(data, i) && alpha(data, i+1) && wordsep(data, i+2) {
		i += 2
	} if digit(data, i) && digit(data, i+1) && digit(data, i+2) && wordsep(data, i+3) {
		i += 3
	}

	i
}

pub fn variant(data: &[u8], mut i: usize) -> usize {
	if digit(data, i) && alphanum(data, i+1) && alphanum(data, i+2) && alphanum(data, i+3) && wordsep(data, i+4) {
		i += 4
	} else if alphanum(data, i) && alphanum(data, i+1) && alphanum(data, i+2) && alphanum(data, i+3) && alphanum(data, i+4) {
		let mut j = i+5;

		if alphanum(data, j) {
			j += 1;
			if alphanum(data, j) {
				j += 1;
				if alphanum(data, j) {
					j += 1;
				}
			}
		}

		if wordsep(data, j) {
			i = j
		}
	}

	i
}

pub fn variants(data: &[u8], mut i: usize) -> usize {
	let j = variant(data, i);
	if j > i {
		i = j;
	}

	while separator(data, i) {
		let j = variant(data, i+1);
		if j > i+1 {
			i = j
		} else {
			break
		}
	}

	i
}

pub fn extension(data: &[u8], mut i: usize) -> usize {
	if singleton(data, i) && separator(data, i+1) {
		let j = extension_subtag(data, i+2);

		if j > i+2 {
			i = j;

			while separator(data, i) {
				let j = extension_subtag(data, i+1);
				if j > i+1 {
					i = j
				} else {
					break
				}
			}
		}
	}

	i
}

pub fn extension_subtag(data: &[u8], mut i: usize) -> usize {
	if alphanum(data, i) {
		if alphanum(data, i+1) {
			let mut j = i+2;
			if alphanum(data, j) {
				j += 1;
				if alphanum(data, j) {
					j += 1;
					if alphanum(data, j) {
						j += 1;
						if alphanum(data, j) {
							j += 1;
							if alphanum(data, j) {
								j += 1;
								if alphanum(data, j) {
									j += 1;
								}
							}
						}
					}
				}
			}

			if wordsep(data, j) {
				i = j;
			}
		}
	}

	i
}

pub fn extensions(data: &[u8], mut i: usize) -> usize {
	let j = extension(data, i);
	if j > i {
		i = j;
	}

	while separator(data, i) {
		let j = extension(data, i+1);
		if j > i+1 {
			i = j
		} else {
			break
		}
	}

	i
}

pub fn privateuse(data: &[u8], mut i: usize) -> usize {
	if privateuse_singleton(data, i) && separator(data, i+1) {
		let j = privateuse_subtag(data, i+2);

		if j > i+2 {
			i = j;

			while separator(data, i) {
				let j = privateuse_subtag(data, i+1);
				if j > i+1 {
					i = j
				} else {
					break
				}
			}
		}
	}

	i
}

pub fn privateuse_subtag(data: &[u8], mut i: usize) -> usize {
	if alphanum(data, i) {
		let mut j = i+1;
		if alphanum(data, j) {
			j += 1;
			if alphanum(data, j) {
				j += 1;
				if alphanum(data, j) {
					j += 1;
					if alphanum(data, j) {
						j += 1;
						if alphanum(data, j) {
							j += 1;
							if alphanum(data, j) {
								j += 1;
								if alphanum(data, j) {
									j += 1;
								}
							}
						}
					}
				}
			}
		}

		if wordsep(data, j) {
			i = j;
		}
	}

	i
}

fn wordsep(data: &[u8], i: usize) -> bool {
	data.len() <= i || data[i] == b'-'
}

fn separator(data: &[u8], i: usize) -> bool {
	data.len() > i && data[i] == b'-'
}

fn is_digit(c: u8) -> bool {
	c >= b'0' && c <= b'9'
}

fn digit(data: &[u8], i: usize) -> bool {
	if data.len() > i {
		let c = data[i];
		is_digit(c)
	} else {
		false
	}
}

fn is_alpha(c: u8) -> bool {
	(c >= b'A' && c <= b'Z') || (c >= b'a' && c <= b'z')
}

fn alpha(data: &[u8], i: usize) -> bool {
	if data.len() > i {
		let c = data[i];
		is_alpha(c)
	} else {
		false
	}
}

fn alphanum(data: &[u8], i: usize) -> bool {
	if data.len() > i {
		let c = data[i];
		is_digit(c) || is_alpha(c)
	} else {
		false
	}
}

fn is_singleton(c: u8) -> bool {
	is_digit(c) || (c != b'x' && c != b'X' && is_alpha(c))
}

fn singleton(data: &[u8], i: usize) -> bool {
	if data.len() > i {
		let c = data[i];
		is_singleton(c)
	} else {
		false
	}
}

fn privateuse_singleton(data: &[u8], i: usize) -> bool {
	if data.len() > i {
		let c = data[i];
		c == b'x' || c == b'X'
	} else {
		false
	}
}