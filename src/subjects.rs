use uuid::Uuid;
use seed::{prelude::*, *};

use crate::{
	Msg,
	Model,
	models::Kind,
};

fn nazev(n: &str) -> Node<Msg> {
	header![
		h4![n]
	]
}

fn popis(u: &str, p: &str) -> Vec<Node<Msg>> {
	vec![
		p![
			b!["Vyučující: "],
			u
		],
		p![p]
	]
}

fn footer(id: Uuid) -> Node<Msg> {
	footer![
		a![
			attrs!{At::Class => "button dark"},
			simple_ev(Ev::Click, Msg::SignUp(id)),
			"Zapsat se"
		]
	]
}

fn subject(id: Uuid, n: &str, u: &str, p: &str) -> Node<Msg> {
	let store = seed::storage::get_storage().unwrap();
	let typ: String = store.get_item("typ").unwrap().unwrap_or_default();

	section![
		attrs!{At::Class => "card"},
		style![
			St::MarginTop => "1em",
		],
		nazev(n),
		popis(u, p),
		match typ.replace('"', "").as_str() {
			"student" => footer(id),
			_ => br![]
		}
	]
}

pub(crate) fn view(model: &Model) -> Vec<Node<Msg>> {
	vec![
		h2!["Předměty"],
		section![
			attrs!{At::Class => "row"},
			section![
				attrs!{At::Class => "col"},
				h3!["Přírodovědné"],
				model.subjects
					.iter()
					.filter(|x| x.kind == Kind::Science)
					.map(|x| subject(x.id, &x.name, &model.teachers.iter().find(|u| x.teacher == u.id).unwrap().name, &x.description))
					.collect::<Vec<_>>()
			],
			section![
				attrs!{At::Class => "col"},
				h3!["Humanitní"],
				model.subjects
					.iter()
					.filter(|x| x.kind == Kind::Humanity)
					.map(|x| subject(x.id, &x.name, &model.teachers.iter().find(|u| x.teacher == u.id).unwrap().name, &x.description))
					.collect::<Vec<_>>()
			],
			section![
				attrs!{At::Class => "col"},
				h3!["Ostatní"],
				model.subjects
					.iter()
					.filter(|x| x.kind == Kind::Other)
					.map(|x| subject(x.id, &x.name, &model.teachers.iter().find(|u| x.teacher == u.id).unwrap().name, &x.description))
					.collect::<Vec<_>>()
			],
		]
	]
}
