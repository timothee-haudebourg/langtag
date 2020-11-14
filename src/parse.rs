pub enum Error {
	EmptyPrivateUse,
	InvalidLanguage
}

pub struct ParsedLangTag {
	pub language_end: usize,
	pub script_end: usize,
	pub region_end: usize,
	pub variant_end: usize,
	pub extension_end: usize,
	pub privateuse_end: usize
}

/// Parse a <langtag> production.
pub fn langtag(data: &[u8], i: usize) -> Result<ParsedLangTag, Error> {
	let language_end = language(data, i);

	if language_end == i {
		return Err(Error::InvalidLanguage)
	}

	let script_end = if separator(data, language_end) {
		script(data, language_end+1)
	} else {
		language_end
	};

	let region_end = if separator(data, script_end) {
		region(data, script_end+1)
	} else {
		script_end
	};

	let variant_end = if separator(data, region_end) {
		variant(data, region_end+1)
	} else {
		region_end
	};

	let extension_end = if separator(data, variant_end) {
		extension(data, variant_end+1)
	} else {
		variant_end
	};

	let privateuse_end = if separator(data, extension_end) {
		privateuse(data, extension_end+1)
	} else {
		extension_end
	};

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
pub fn language(data: &[u8], mut i: usize) -> usize {
	let s = i;

	// shortest ISO 639 code
	if alpha(data, i) && alpha(data, i+1) {
		i += 2;

		if alpha(data, i) {
			i += 1;
		}

		// sometimes followed by extended language subtags
		if separator(data, i) {
			i = extlang(data, i+1)
		} else {
			// or reserved for future use, or registered language subtag.
			while i < s+8 && alpha(data, i) {
				i += 1
			}
		}
	}

	i
}

fn extlang(data: &[u8], mut i: usize) -> usize {
	// selected ISO 639 codes
	if alpha(data, i) && alpha(data, i+1) && alpha(data, i+2) {
		i += 3;

		// permanently reserved
		if separator(data, i) {
			if alpha(data, i+1) && alpha(data, i+2) && alpha(data, i+3) {
				i += 4;
				if separator(data, i) {
					if alpha(data, i+1) && alpha(data, i+2) && alpha(data, i+3) {
						i += 4;
					}
				}
			}
		}
	}

	i
}

pub fn script(data: &[u8], i: usize) -> usize {
	if alpha(data, i) && alpha(data, i+1) && alpha(data, i+2) && alpha(data, i+3) {
		i + 4
	} else {
		i
	}
}

pub fn region(data: &[u8], mut i: usize) -> usize {
	if alpha(data, i) && alpha(data, i+1) {
		i += 2
	} if digit(data, i) && digit(data, i+1) && digit(data, i+2) {
		i += 3
	}

	i
}

pub fn variant(data: &[u8], mut i: usize) -> usize {
	if digit(data, i) && alphanum(data, i+1) && alphanum(data, i+2) && alphanum(data, i+3) {
		i += 4
	} if alphanum(data, i) && alphanum(data, i+1) && alpha(data, i+2) && alphanum(data, i+3) && alphanum(data, i+4) {
		i += 5;

		if alphanum(data, i) {
			i += 1;
		}

		if alphanum(data, i) {
			i += 1;
		}

		if alphanum(data, i) {
			i += 1;
		}
	}

	i
}

pub fn extension(data: &[u8], mut i: usize) -> usize {
	if singleton(data, i) && separator(data, i+1) && alphanum(data, i+2) && alphanum(data, i+3) {
		i += 4;

		if alphanum(data, i) {
			i += 1;
			if alphanum(data, i) {
				i += 1;
				if alphanum(data, i) {
					i += 1;
					if alphanum(data, i) {
						i += 1;
						if alphanum(data, i) {
							i += 1;
							if alphanum(data, i) {
								i += 1;
							}
						}
					}
				}
			}
		}

		while separator(data, i) && alphanum(data, i+1) && alphanum(data, i+2) {
			i += 3;

			if alphanum(data, i) {
				i += 1;
				if alphanum(data, i) {
					i += 1;
					if alphanum(data, i) {
						i += 1;
						if alphanum(data, i) {
							i += 1;
							if alphanum(data, i) {
								i += 1;
								if alphanum(data, i) {
									i += 1;
								}
							}
						}
					}
				}
			}
		}
	}

	i
}

pub fn privateuse(data: &[u8], mut i: usize) -> usize {
	if privateuse_singleton(data, i) && separator(data, i+1) && alphanum(data, i+2) && alphanum(data, i+3) {
		i += 4;

		if alphanum(data, i) {
			i += 1;
			if alphanum(data, i) {
				i += 1;
				if alphanum(data, i) {
					i += 1;
					if alphanum(data, i) {
						i += 1;
						if alphanum(data, i) {
							i += 1;
							if alphanum(data, i) {
								i += 1;
							}
						}
					}
				}
			}
		}

		while separator(data, i) && alphanum(data, i+1) && alphanum(data, i+2) {
			i += 3;

			if alphanum(data, i) {
				i += 1;
				if alphanum(data, i) {
					i += 1;
					if alphanum(data, i) {
						i += 1;
						if alphanum(data, i) {
							i += 1;
							if alphanum(data, i) {
								i += 1;
								if alphanum(data, i) {
									i += 1;
								}
							}
						}
					}
				}
			}
		}
	}

	i
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