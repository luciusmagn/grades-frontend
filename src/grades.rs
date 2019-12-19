use seed::{prelude::*, *};
use crate::{
	Msg,
	Model,
	models::{
		GradeVal,
		Grade,
	},
	console_log,
};

pub(crate) fn name(n: &str) -> Node<Msg> {
	header![
		h3![n]
	]
}

pub(crate) fn grades(g: Vec<Grade>) -> Node<Msg> {
	table![
		attrs!{At::Class => "striped"},
		caption![
			h4!["Známky"]
		],
		thead![
			th!["Datum"],
			th!["Hodnocení"],
			th!["Popis"],
		],
		tbody![
			g.iter()
				.map(|x| {
					tr![
						td![x.date.format("%d.%m.%Y").to_string()],
						td![match x.val {
							GradeVal::Regular(f) => f.to_string(),
							_ => unreachable!("regular"),
						}],
						td![x.name]
					]
				})
		]
	]
}

pub(crate) fn bonus(g: Vec<Grade>) -> Node<Msg> {
	table![
		attrs!{At::Class => "striped"},
		caption![
			h4!["Bonus"]
		],
		thead![
			th!["Datum"],
			th!["Hodnota"],
		],
		tbody![
			g.iter()
				.map(|x| {
					tr![
						td![x.date.format("%d.%m.%Y").to_string()],
						td![match x.val {
							GradeVal::Bonus(u) => u.to_string(),
							_ => unreachable!("bonus"),
						}],
					]
				})
		]
	]
}

pub(crate) fn penalisation(g: Vec<Grade>) -> Node<Msg> {
	table![
		attrs!{At::Class => "striped"},
		caption![
			h4!["Penalizace"]
		],
		thead![
			th!["Datum"],
			th!["Hodnota"],
		],
		tbody![
			g.iter()
				.map(|x| {
					tr![
						td![x.date.format("%d.%m.%Y").to_string()],
						td![match x.val {
							GradeVal::Penalisation(u) => u.to_string(),
							_ => unreachable!("penalisation"),
						}],
					]
				})
		]
	]
}

pub(crate) fn subject(s: &crate::models::Subject, n: &str, formule: &str, g: Vec<Grade>, b: Vec<Grade>, p: Vec<Grade>) -> Node<Msg> {
	console_log!("{:?} {:?} {:?}", g, b, p);
	section![
		attrs!{At::Class => "card"},
		name(n),
		ul![
			li![format!(
				"Finální známka: {}",
					rsc::eval(
						&s.grade_formula
							.replace("$sum_regular$", &g.iter()
								.filter_map(|y| match y.val.clone() {
									GradeVal::Regular(v) => Some(v),
									_ => None,
								})
								.fold(0.0, |acc, n| acc + n)
								.to_string()
							)
							.replace("$sum_bonus$", &b.iter()
								.filter_map(|y| match y.val.clone() {
									GradeVal::Bonus(v) => Some(v),
									_ => None,
								})
								.fold(0, |acc, n| acc + n)
								.to_string()
							)
							.replace("$sum_penal$", &p.iter()
								.filter_map(|y| match y.val.clone() {
									GradeVal::Penalisation(v) => Some(v),
									_ => None,
								})
								.fold(0, |acc, n| acc + n)
								.to_string()
							)
							.replace("$count_regular$", &g.iter()
								.count()
								.to_string()
							)
							.replace("$count_bonus$", &b.iter()
								.count()
								.to_string()
							)
							.replace("$count_penal$", &p.iter()
								.count()
								.to_string()
							)
						).unwrap_or(-1.0)
				)],
			li![format!("Vzorec: {}", formule)],
		],
		section![
			attrs!{At::Class => "row"},
			section![
				attrs!{At::Class => "col"},
				grades(g),
			],
			section![
				attrs!{At::Class => "col"},
				bonus(b),
			],
			section![
				attrs!{At::Class => "col"},
				penalisation(p),
			],
		]
	]
}

pub(crate) fn view(model: &Model) -> Vec<Node<Msg>> {
	let mut subjects = model
		.subjects
		.iter()
		.filter(|x| model
			.student
			.subjects
			.contains(&x.id))
		.cloned()
		.collect::<Vec<crate::models::Subject>>();
	
	subjects.sort_by(|a, b| a.name.cmp(&b.name));
	
	let grades = model
		.grades
		.iter()
		.inspect(|x| console_log!("first {:?}", x))
		.filter(|x| model
			.student
			.id == x.student)
		.inspect(|x| console_log!("second {:?}", x))
		.cloned()
		.collect::<Vec<Grade>>();

	vec![
		p![
			"Tato stránka obsahuje seznam všech známek a konečného hodnocení"
		],
		table![
			attrs!{At::Class => "striped"},
			caption!["Celkové hodnocení"],
			thead![
				vec![th![String::new()]]
					.iter()
					.chain(subjects.iter().map(|x| th![x.name.clone()]).collect::<Vec<_>>().iter())
					.cloned()
					.collect::<Vec<_>>(),
			],
			tbody![
				tr![	
					vec![th!["Hodnocení".to_string()]]
						.iter()
						.chain(subjects.iter().map(|x| td![
								rsc::eval(
									&x.grade_formula
										.replace("$sum_regular$", &grades.iter()
											.filter_map(|y| match (y.val.clone(), y.subject == x.id) {
												(GradeVal::Regular(v), true) => Some(v),
												_ => None,
											})
											.fold(0.0, |acc, n| acc + n)
											.to_string()
										)
										.replace("$sum_bonus$", &grades.iter()
											.filter_map(|y| match (y.val.clone(), y.subject == x.id) {
												(GradeVal::Bonus(v), true) => Some(v),
												_ => None,
											})
											.fold(0, |acc, n| acc + n)
											.to_string()
										)
										.replace("$sum_penal$", &grades.iter()
											.filter_map(|y| match (y.val.clone(), y.subject == x.id) {
												(GradeVal::Penalisation(v), true) => Some(v),
												_ => None,
											})
											.fold(0, |acc, n| acc + n)
											.to_string()
										)
										.replace("$count_regular$", &grades.iter()
											.filter_map(|y| match (y.val.clone(), y.subject == x.id) {
												(GradeVal::Regular(v), true) => Some(v),
												_ => None,
											})
											.count()
											.to_string()
										)
										.replace("$count_bonus$", &grades.iter()
											.filter_map(|y| match (y.val.clone(), y.subject == x.id) {
												(GradeVal::Bonus(v), true) => Some(v),
												_ => None,
											})
											.count()
											.to_string()
										)
										.replace("$count_penal$", &grades.iter()
											.filter_map(|y| match (y.val.clone(), y.subject == x.id) {
												(GradeVal::Penalisation(v), true) => Some(v),
												_ => None,
											})
											.count()
											.to_string()
										)
									).unwrap_or(-1.0).to_string()
							]).collect::<Vec<_>>().iter())
						.cloned()
						.collect::<Vec<_>>(),
				],
				tr![
					vec![th!["Bonus".to_string()]]
						.iter()
						.chain(
							subjects
								.iter()
								.map(|x| td![grades.iter()
									.filter_map(|y| match (y.val.clone(), y.subject == x.id) {
										(GradeVal::Bonus(v), true) => Some(v),
										_ => None,
									}).fold(0, |acc, n| acc + n).to_string()]
								)
								.collect::<Vec<_>>().iter()
						)
						.cloned()
						.collect::<Vec<_>>(),
				],
				tr![
					vec![th!["Penalizace".to_string()]]
						.iter()
						.chain(
							subjects
								.iter()
								.map(|x| td![grades.iter()
									.filter_map(|y| match (y.val.clone(), y.subject == x.id) {
										(GradeVal::Penalisation(v), true) => Some(v),
										_ => None,
									}).fold(0, |acc, n| acc + n).to_string()]
								)
								.collect::<Vec<_>>().iter()
						)
						.cloned()
						.collect::<Vec<_>>(),
				],
			],
		],
		hr![],
		br![],
		br![],
		
		div![
		subjects
			.iter()
			.map(|x| subject(
				&x,
				&x.name,
				&x.grade_formula,
				grades
					.iter()
					.filter(|y| y.is_grade())
					.cloned()
					.collect::<Vec<Grade>>(),
				grades
					.iter()
					.filter(|y| y.is_bonus())
					.cloned()
					.collect::<Vec<Grade>>(),
				grades
					.iter()
					.filter(|y| y.is_penalisation())
					.cloned()
					.collect::<Vec<Grade>>()
			))
			.collect::<Vec<Node<Msg>>>()
		]
	]
}
