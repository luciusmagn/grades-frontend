use seed::{prelude::*, *};

pub(crate) fn view() -> Vec<Node<crate::Msg>> {
	vec![
		h2!["Auth"],
		section![
			attrs!{At::Class => "row"},
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
							}
						]
					]
				]
			],
			section![
				attrs!{At::Class => "col"},
				div![
					attrs![At::Class => "card"],
					header![
						h4!["Přihlášení - Student"]
					],
				]
			],
		],
		section![
			attrs!{At::Class => "row"},
			section![
				attrs!{At::Class => "col"},
				div![
					attrs![At::Class => "card"],
					header![
						h4!["Registrace - Vyučující"]
					],
				]
			],
			section![
				attrs!{At::Class => "col"},
				div![
					attrs![At::Class => "card"],
					header![
						h4!["Přihlášení - Vyučující"]
					],
				]
			],
		],
	]
}
