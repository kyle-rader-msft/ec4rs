use std::io;
use crate::ReadError;
use crate::Section;
use crate::linereader::LineReader;

/// A parser for the text of an EditorConfig file.
///
/// This struct wraps any [std::io::BufRead]
/// and parses the prelude and zero or more sections from it.
pub struct EcParser<R: io::BufRead> {
	/// Incidates if a `root = true` line was found in the prelude.
	pub is_root: bool,
	eof: bool,
	reader: LineReader<R>
}

impl<R: io::Read> EcParser<io::BufReader<R>> {
	/// See [EcParser::new].
	pub fn new_buffered(source: R) -> Result<EcParser<io::BufReader<R>>, ReadError> {
		Self::new(io::BufReader::new(source))
	}
}

impl<R: io::BufRead> EcParser<R> {
	/// Constructs a new [EcParser] and reads the prelude from the provided source.
	///
	/// Returns `Ok` if the prelude was read successfully,
	/// otherwise returns `Err` with the error that occurred during reading.
	pub fn new(buf_source: R) -> Result<EcParser<R>, ReadError> {
		let mut reader = LineReader::new(buf_source);
		let mut is_root = false;
		let eof = loop {
			use crate::linereader::Line;
			match reader.next_line() {
				Err(ReadError::Eof)  => break true,
				Err(e)               => return Err(e),
				Ok(Line::Nothing)    => (),
				Ok(Line::Section(_)) => break false,
				Ok(Line::Pair(k, v)) => {
					if "root".eq_ignore_ascii_case(k) {
						if let Ok(b) = v.parse::<bool>() {
							is_root = b;
						}
					}
					// Quietly ignore unknown properties.
				}
			}
		};
		Ok(EcParser {is_root, reader, eof})
	}

	/// Returns `true` if there may be another section to read.
	pub fn has_more(&self) -> bool {
		self.eof
	}

	/// Reads a [Section] from the internal source.
	pub fn read_section(&mut self) -> Result<Section, ReadError> {
		if !self.eof {
			use crate::linereader::Line;
			if let Ok(Line::Section(header)) = self.reader.reparse() {
				let mut section = Section::new(header);
				loop {
					match self.reader.next_line() {
						Err(e) => {
							self.eof = true;
							if let ReadError::Eof = e {
								break Ok(section)
							} else {
								break Err(e)
							}
						}
						Ok(Line::Section(_)) => break Ok(section),
						Ok(Line::Nothing)    => (),
						Ok(Line::Pair(k,v))  => {
							section.insert(k,v);
						}
					}
				}
			} else {
				Err(ReadError::InvalidLine)
			}
		} else {
			Err(ReadError::Eof)
		}
	}
}

impl<R: io::BufRead> Iterator for EcParser<R> {
	type Item = Result<Section, ReadError>;
	fn next(&mut self) -> Option<Self::Item> {
		match self.read_section() {
			Ok(r)               => Some(Ok(r)),
			Err(ReadError::Eof) => None,
			Err(e)              => Some(Err(e))
		}
	}
}

impl<R: io::BufRead> std::iter::FusedIterator for EcParser<R> {}


impl<R: io::BufRead> crate::PropertiesSource for &mut EcParser<R> {
	fn apply_to(self, props: &mut crate::Properties, path: impl AsRef<std::path::Path>) {
		let path = path.as_ref();
		for section in self.flatten() {
				section.apply_to(props, path)
		}
	}
}