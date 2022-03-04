use super::Matcher;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Glob(pub(super) Vec<Matcher>);

impl Glob {
	pub fn append_char(&mut self, c: char) {
		if c == '/' {
			self.append(Matcher::Sep)
		} else if let Some(Matcher::Suffix(suffix)) = &mut self.0.last_mut() {
			suffix.push(c);
		} else {
			self.append(Matcher::Suffix(c.to_string()))
		}
	}

	pub fn append(&mut self, matcher: Matcher) {
		match &matcher {
			Matcher::Sep => {
				if let Some(Matcher::Sep) = &self.0.last() {
					return
				}
			},
			Matcher::AnySeq(true) => {
				if let Some(Matcher::AnySeq(true)) = &self.0.last() {
					return
				}
			}
			_ => ()
		}
		self.0.push(matcher);
	}
}
