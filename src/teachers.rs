use crate::{Model, Msg};
use seed::{prelude::*, *};


fn name(n: &str) -> Node<Msg> {
	header![
		h4![n]
	]
}

fn info(i: &str) -> Node<Msg> {
	p![i]
}

fn subjects(s: Vec<String>) -> Node<Msg> {
	ul![
		s.iter()
			.map(|e| li![e])
	]
}

fn teacher(n: &str, i: &str, p: Vec<String>) -> Node<Msg> {
	section![
		attrs!{At::Class => "card"},
		style![
			St::MarginTop => "1em",
		],
		name(n),
		info(i),
		subjects(p),
	]
}

pub(crate) fn view(model: &Model) -> Vec<Node<Msg>> {
	vec![
		h2!["Vyučující"],
		section![
			attrs!{At::Class => "row"},
			section![
				attrs!{At::Class => "col"},
				model.teachers
					.iter()
					.enumerate()
					.filter(|(i, _)| i % 3 == 1)
					.map(|(_, x)| x)
					.map(|x| teacher(
						&x.name,
						&x.info,
						model.subjects.iter()
							.filter(|p| x.subjects.contains(&p.id))
							.map(|p| p.name.clone())
							.collect::<Vec<_>>()
					))
			],
			section![
				attrs!{At::Class => "col"},
			],
			section![
				attrs!{At::Class => "col"},
			],
		]
	]
}
