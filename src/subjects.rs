use seed::{prelude::*, *};

fn nazev(n: &str) -> Node<crate::Msg> {
	header![
		h4![n]
	]
}

fn popis(u: &str, p: &str) -> Vec<Node<crate::Msg>> {
	vec![
		p![
			b!["Vyučující: "],
			u
		],
		p![p]
	]
}

fn footer() -> Node<crate::Msg> {
	footer![
		a![
			attrs!{At::Class => "button dark"},
			"Zapsat se"
		]
	]
}

fn subject(n: &str, u: &str, p: &str) -> Node<crate::Msg> {
	section![
		attrs!{At::Class => "card"},
		style![
			St::MarginTop => "1em",
		],
		nazev(n),
		popis(u, p),
		footer(),
	]
}

pub(crate) fn view(model: &crate::Model) -> Vec<Node<crate::Msg>> {
	vec![
		h2!["Předměty"],
		section![
			attrs!{At::Class => "row"},
			section![
				attrs!{At::Class => "col"},
				h3!["Přírodovědné"],
			],
			section![
				attrs!{At::Class => "col"},
				h3!["Humanitní"],
			],
			section![
				attrs!{At::Class => "col"},
				h3!["Ostatní"],
			],
		]
	]
}
