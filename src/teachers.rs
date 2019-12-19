use crate::{Model, Msg};
use seed::{prelude::*, *};


fn name(n: &str, em: &str) -> Node<Msg> {
	header![
		h4![n],
		a![
			attrs!{At::Href => format!("mailto:{}", em)},
			em
		],
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

fn teacher(n: &str, em: &str, i: &str, p: Vec<String>) -> Node<Msg> {
	section![
		attrs!{At::Class => "card"},
		style![
			St::MarginTop => "1em",
		],
		name(n, em),
		info(i),
		h5!["Předměty"],
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
					.filter(|(i, _)| i % 3 == 0)
					.map(|(_, x)| x)
					.map(|x| teacher(
						&x.name,
						&x.email,
						&x.info,
						model.subjects.iter()
							.filter(|p| p.teacher == x.id)
							.map(|p| p.name.clone())
							.collect::<Vec<_>>()
					))
			],
			section![
				attrs!{At::Class => "col"},
				model.teachers
					.iter()
					.enumerate()
					.filter(|(i, _)| i % 3 == 1)
					.map(|(_, x)| x)
					.map(|x| teacher(
						&x.name,
						&x.email,
						&x.info,
						model.subjects.iter()
							.filter(|p| p.teacher == x.id)
							.map(|p| p.name.clone())
							.collect::<Vec<_>>()
					))
			],
			section![
				attrs!{At::Class => "col"},
				model.teachers
					.iter()
					.enumerate()
					.filter(|(i, _)| i % 3 == 2)
					.map(|(_, x)| x)
					.map(|x| teacher(
						&x.name,
						&x.email,
						&x.info,
						model.subjects.iter()
							.filter(|p| p.teacher == x.id)
							.map(|p| p.name.clone())
							.collect::<Vec<_>>()
					))
			],
		]
	]
}
