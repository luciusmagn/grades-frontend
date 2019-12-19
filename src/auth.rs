use crate::Msg;
use seed::{prelude::*, *};

pub(crate) fn view() -> Vec<Node<crate::Msg>> {
	vec![
		h2!["Auth"],
		section![
			attrs!{At::Class => "row"},
			section![attrs!{At::Class => "col-1"}],
			section![
				attrs!{At::Class => "col"},
				div![
					attrs![At::Class => "card"],
					header![
						h4!["Přihlásit - Vyučující"]
					],
					form![
						label!["Email"],
						input![
							attrs!{
								At::Type => "email",
								At::Placeholder => "xvomma01@gjk.cz"
							},
							input_ev(Ev::Input, Msg::TeacherLoginEmail),
						],
						label!["Heslo"],
						input![
							attrs!{
								At::Type => "password",
								At::Placeholder => "1234"
							},
							input_ev(Ev::Input, Msg::TeacherLoginPass),
						],
						button![
							style![St::MarginTop => "1em"],
							"Přihlásit",
						],
						raw_ev(Ev::Submit, move |event| {
							event.prevent_default();
							Msg::TeacherLogin
						}),
					]
				]
			],
			section![
				attrs!{At::Class => "col"},
				div![
					attrs![At::Class => "card"],
					header![
						h4!["Přihlásit - Student"]
					],
					form![
						label!["Email"],
						input![
							attrs!{
								At::Type => "email",
								At::Placeholder => "xvomma01@gjk.cz"
							},
							input_ev(Ev::Input, Msg::StudentLoginEmail),
						],
						label!["Heslo"],
						input![
							attrs!{
								At::Type => "pass",
								At::Placeholder => "1234"
							},
							input_ev(Ev::Input, Msg::StudentLoginPass),
						],
						button![
							style![St::MarginTop => "1em"],
							"Přihlásit",
							simple_ev(Ev::Click, Msg::StudentLogin),
						],
						raw_ev(Ev::Submit, move |event| {
							event.prevent_default();
							Msg::StudentLogin
						}),
					]
				],
			],
			section![attrs!{At::Class => "col-1"}],
		],
		section![
			attrs!{At::Class => "row"},
			section![attrs!{At::Class => "col-1"}],
			section![
				attrs!{At::Class => "col"},
				div![
					attrs![At::Class => "card"],
					header![
						h4!["Registrace - Vyučující"]
					],
					form![
						label!["Jméno"],
						input![
							attrs!{
								At::Type => "text",
								At::Placeholder => "Brian Kernighan",
							},
							input_ev(Ev::Input, Msg::TeacherRegisterName),
						],
						label!["Školní email"],
						input![
							attrs!{
								At::Type => "email",
								At::Placeholder => "hozda@gjk.cz",
							},
							input_ev(Ev::Input, Msg::TeacherRegisterEmail),
						],
						label!["Heslo"],
						input![
							attrs!{
								At::Type => "password",
								At::Placeholder => "1234",
							},
							input_ev(Ev::Input, Msg::TeacherRegisterPass),
						],
						label!["Heslo znovu"],
						input![
							attrs!{
								At::Type => "password",
								At::Placeholder => "1234",
							},
							input_ev(Ev::Input, Msg::TeacherRegisterPassAgain),
						],
						button![
							style![St::MarginTop => "1em"],
							"Registrovat",
						],
						raw_ev(Ev::Submit, move |event| {
							event.prevent_default();
							Msg::TeacherRegister
						}),
					]
				]
			],
			section![
				attrs!{At::Class => "col"},
				div![
					attrs![At::Class => "card"],
					header![
						h4!["Registrace - Student"]
					],
					form![
						label!["Jméno"],
						input![
							attrs!{
								At::Type => "text",
								At::Placeholder => "Daniel Ritchie",
							},
							input_ev(Ev::Input, Msg::StudentRegisterName),
						],
						label!["Školní email"],
						input![
							attrs!{
								At::Type => "email",
								At::Placeholder => "xhozl01@gjk.cz",
							},
							input_ev(Ev::Input, Msg::StudentRegisterEmail),
						],
						label!["Heslo"],
						input![
							attrs!{
								At::Type => "password",
								At::Placeholder => "1234",
							},
							input_ev(Ev::Input, Msg::StudentRegisterPass),
						],
						label!["Heslo znovu"],
						input![
							attrs!{
								At::Type => "password",
								At::Placeholder => "1234",
							},
							input_ev(Ev::Input, Msg::StudentRegisterPassAgain),
						],
						button![
							style![St::MarginTop => "1em"],
							"Registrovat",
						],
						raw_ev(Ev::Submit, move |event| {
							event.prevent_default();
							Msg::StudentRegister
						}),
					]
				]
			],
			section![attrs!{At::Class => "col-1"}],
		],
	]
}
