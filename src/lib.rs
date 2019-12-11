use seed::{prelude::*, *};

#[macro_use]
extern crate serde;
extern crate chrono;
extern crate uuid;

mod auth;
mod index;
mod grades;
mod subjects;
mod teachers;

mod models;

#[derive(Clone)]
enum Routes {
	Index,
	Auth,
	Grades,
	Subjects,
	Teachers,
}

struct Model {
	route: Routes,
	index: index::Model,
}

impl Default for Model {
	fn default() -> Self {
		Self { route: Routes::Index, index: index::Model {}}
	}
}


#[derive(Clone)]
enum Msg {
	Route(Routes),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
	match msg {
		Msg::Route(r) => model.route = r,
	}
}

fn view(model: &Model) -> impl View<Msg> {
	let a_style = style![
		St::PointerEvents => "auto",
		St::Cursor => "pointer",
		St::Transition => 0.4,
	];

	section![
		attrs!{At::Class => "row"},
		aside![
			attrs!{At::Class => "col-2 nav"},
			div![
				// settings
				attrs!{At::Class => "nav-left"},
				style![
					St::FlexDirection => "column",
				],

				// life-hack
				h1![],
				hr![
					style![
						St::Opacity => 0,
					]
				],

				a![
					a_style.clone(),
					simple_ev(Ev::Click, Msg::Route(Routes::Index)),
					"Domů"
				],
				a![
					a_style.clone(),
					simple_ev(Ev::Click, Msg::Route(Routes::Auth)),
					"Registrace/Přihlášení"
				],
				a![
					a_style.clone(),
					simple_ev(Ev::Click, Msg::Route(Routes::Grades)),
					"Známky"
				],
				a![
					a_style.clone(),
					simple_ev(Ev::Click, Msg::Route(Routes::Subjects)),
					"Předměty"
				],
				a![
					a_style,
					simple_ev(Ev::Click, Msg::Route(Routes::Teachers)),
					"Učitelé"
				],
			],
		],
		main![
			attrs!{At::Class => "col"},
			h1![ "Známky" ],
			hr![],
			
			{
				match model.route {
					Routes::Auth => auth::view(),
					Routes::Index => index::view(),
					Routes::Grades => grades::view(),
					Routes::Subjects => subjects::view(model),
					Routes::Teachers => teachers::view(model),
				}
			}
		]
	]
}

#[wasm_bindgen(start)]
pub fn render() {
	App::builder(update, view).build_and_start();
}
