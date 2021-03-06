#![allow(missing_docs)]
use uuid::Uuid;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GradeVal {
	Regular(f32),
	Bonus(u32),
	Penalisation(u32),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Grade {
	pub id: Uuid,
	pub name: String,
	pub val: GradeVal,
	pub description: Option<String>,
	pub date: DateTime<Utc>,
	pub subject: Uuid,
	pub student: Uuid,
}

impl Grade {
	pub fn is_grade(&self) -> bool {
		if let GradeVal::Regular(_) = self.val {
			true
		} else { false }
	}

	pub fn is_bonus(&self) -> bool {
		if let GradeVal::Bonus(_) = self.val {
			true
		} else { false }
	}

	pub fn is_penalisation(&self) -> bool {
		if let GradeVal::Penalisation(_) = self.val {
			true
		} else { false }
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Kind {
	Other,
	Science,
	Humanity,
}

impl Default for Kind {
	fn default() -> Kind {
		Kind::Science
	}
}

impl Kind {
	pub(crate) fn new(s: &str) -> Kind {
		match s {
			"Science" => Kind::Science,
			"Humanity" => Kind::Humanity,
			"Other" => Kind::Other,
			_ => unreachable!("kind"),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subject {
	pub id: Uuid,
	pub name: String,
	pub description: String,
	pub year: String,
	pub grade_formula: String,
	pub kind: Kind,
	pub teacher: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Teacher {
	pub id: Uuid,
	pub name: String,
	pub email: String,
	pub info: String,
	pub subjects: Vec<Uuid>,
}

impl Default for Teacher {
	fn default() -> Self {
		Self {
			id: Uuid::new_v4(),
			name: String::new(),
			email: String::new(),
			info: String::new(),
			subjects: vec![],
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Student {
	pub id: Uuid,
	pub name: String,
	pub email: String,
	pub subjects: Vec<Uuid>,
}

impl Default for Student {
	fn default() -> Self {
		Self {
			id: Uuid::new_v4(),
			name: String::new(),
			email: String::new(),
			subjects: vec![],
		}
	}
}
