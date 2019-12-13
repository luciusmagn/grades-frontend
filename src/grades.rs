use seed::{prelude::*, *};
use crate::models::GradeVal;

pub(crate) fn name(n: &str) -> Node<crate::Msg> {
	header![
		h3![n]
	]
}

pub(crate) fn grades(g: Vec<crate::models::Grade>) -> Node<crate::Msg> {
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
							_ => unreachable!(),
						}],
						td![x.name]
					]
				})
		]
	]
}

pub(crate) fn bonus(g: Vec<crate::models::Grade>) -> Node<crate::Msg> {
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
							_ => unreachable!(),
						}],
					]
				})
		]
	]
}

pub(crate) fn penalisation(g: Vec<crate::models::Grade>) -> Node<crate::Msg> {
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
							GradeVal::Penalisation(u) => u.to_string(),
							_ => unreachable!(),
						}],
					]
				})
		]
	]
}

pub(crate) fn subject(n: &str, formule: &str, g: Vec<crate::models::Grade>, b: Vec<crate::models::Grade>, p: Vec<crate::models::Grade>) -> Node<crate::Msg> {
	section![
		attrs!{At::Class => "card"},
		name(n),
		ul![
			li!["Finální známka: 2"],
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

pub(crate) fn view(model: &crate::Model) -> Vec<Node<crate::Msg>> {
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
		.filter(|x| model
			.student
			.subjects
			.contains(&x.subject))
		.filter(|x| model
			.student
			.id == x.student)
		.cloned()
		.collect::<Vec<crate::models::Grade>>();

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
					th!["Hodnocení"],
					td!["1.3"],
					td!["1.9"],
					td!["4"],
					td!["88"],
				],
				tr![
					th!["Bonus"],
					td!["13"],
					td!["16"],
					td!["26"],
					td!["66"],
				],
				tr![
					th!["Penalizace"],
					td!["1.3"],
					td!["1.9"],
					td!["4"],
					td!["88"],
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
				&x.name,
				&x.grade_formula,
				grades
					.iter()
					.filter(|y| y.is_grade())
					.cloned()
					.collect::<Vec<crate::models::Grade>>(),
				grades
					.iter()
					.filter(|y| y.is_bonus())
					.cloned()
					.collect::<Vec<crate::models::Grade>>(),
				grades
					.iter()
					.filter(|y| y.is_penalisation())
					.cloned()
					.collect::<Vec<crate::models::Grade>>()
			))
			.collect::<Vec<Node<crate::Msg>>>()
		]
	]
}
