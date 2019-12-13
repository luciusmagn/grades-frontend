use seed::{prelude::*, *};


fn name(n: &str) -> Node<crate::Msg> {
	header![
		h4![n]
	]
}

fn info(i: &str) -> Node<crate::Msg> {
	p![i]
}

fn subjects(s: Vec<&str>) -> Node<crate::Msg> {
	ul![
		s.iter()
			.map(|e| li![e])
	]
}

fn teacher(n: &str, i: &str, p: Vec<&str>) -> Node<crate::Msg> {
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

pub(crate) fn view(model: &crate::Model) -> Vec<Node<crate::Msg>> {
	vec![
		h2!["Vyučující"],
		section![
			attrs!{At::Class => "row"},
			section![
				attrs!{At::Class => "col"},
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
