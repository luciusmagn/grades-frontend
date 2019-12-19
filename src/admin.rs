use uuid::Uuid;
use seed::{prelude::*, *};

use crate::{Msg, Model};
use crate::grades::{grades, bonus, penalisation};

pub(crate) fn view(model: &Model) -> Vec<Node<crate::Msg>> {
	vec![
		h2!["Auth"],
		section![
			attrs!{At::Class => "row"},
			section![attrs!{At::Class => "col-1"}],
			section![
				attrs!{At::Class => "col-4"},
				div![
					attrs![At::Class => "card"],
					header![
						h4!["Můj popis"]
					],
					form![
						textarea![
							model.teacher.info,
							input_ev(Ev::Input, Msg::MyDescription)
						],
						button![
							style![St::MarginTop => "1em"],
							"Aktualizovat"
						],
						raw_ev(Ev::Submit, move |event| {
							event.prevent_default();
							Msg::MyDescriptionSubmit
						})
					]
				],
				div![
					attrs![At::Class => "card"],
					style![
						St::MarginTop => "1em",
					],
					header![
						h4!["Moje předměty"]
					],
					ul![
						model.subjects
							.iter()
							.filter(|x| x.teacher == model.teacher.id)
							.map(|x| li![x.name])
							.collect::<Vec<_>>()
					]
				]
			],
			section![
				attrs!{At::Class => "col"},
				div![
					attrs!{At::Class => "card"},
					header![
						h4!["Nový předmět"]
					],
					form![
						label!["Název"],
						input![
							attrs!{
								At::Type => "text",
								At::Placeholder => "Středověk kolem nás",
							},
							input_ev(Ev::Input, Msg::NewSubjectName)
						],
						label!["Popis"],
						textarea![
							attrs!{
								At::Placeholder => "Středověk kolem nás",
							},
							input_ev(Ev::Input, Msg::NewSubjectDescription)
						],						
						label!["Určeno pro"],
						input![
							attrs!{
								At::Type => "text",
								At::Placeholder => "R1.A-R3.A",
							},
							input_ev(Ev::Input, Msg::NewSubjectYear)
						],
						label!["Vzorec výsledné známky"],
						textarea![
							attrs!{
								At::Placeholder => "Středověk kolem nás",
							},
							input_ev(Ev::Input, Msg::NewSubjectGradeFormula)
						],
						label!["Typ předmětu"],
						select![
							attrs!{At::Value => "Science"},
							option![attrs!{At::Value => "Science"}, "Přírodovědný"],
							option![attrs!{At::Value => "Humanity"}, "Humanitní"],
							option![attrs!{At::Value => "Other"}, "Ostatní"],
							input_ev(Ev::Input, Msg::NewSubjectKind),
						],
						button![
							style![St::MarginTop => "1em"],
							"Vytvořit",
						],
						raw_ev(Ev::Submit, move |event| {
							event.prevent_default();
							Msg::NewSubjectSubmit
						})
					]
				]
			],
			section![attrs!{At::Class => "col-1"}],
		],
		section![
			attrs!{At::Class => "row"},
			section![attrs!{At::Class => "col-1"}],
			section![
				attrs!{At::Class => "col"},
				div![
					attrs!{At::Class => "card"},
					header![
						h4![
							"Nová známka"
						],
					],
					form![
						label!["Typ známky"],
						select![
							attrs!{At::Value => "Regular"},
							option![attrs!{At::Value => "Regular"}, "Známka"],
							option![attrs!{At::Value => "Bonus"}, "Bonus"],
							option![attrs!{At::Value => "Penalisation"}, "Penalizace"],
							input_ev(Ev::Input, Msg::NewGradeType),
						],
						label!["Datum"],
						input![
							attrs!{At::Type => "date"},
							input_ev(Ev::Input, Msg::NewGradeDate),
						],
						label!["Předmět"],
						select![
							attrs!{At::Value => "..."},
							model.subjects
								.iter()
								.filter(|x| x.teacher == model.teacher.id)
								.map(|x| option![attrs!{At::Value => x.id}, x.name.clone()])
								.collect::<Vec<_>>(),
							input_ev(Ev::Input, |v| Msg::NewGradeSubject(Uuid::parse_str(&v).unwrap()))
						],
						label!["Student"],
						select![
							attrs!{At::Value => "..."},
							model.students
								.iter()
								.filter(|x| x.subjects.contains(&model.new_grade.subject))
								.map(|x| option![attrs!{At::Value => x.id}, x.name.clone()])
								.collect::<Vec<_>>(),
							input_ev(Ev::Input, |v| Msg::NewGradeStudent(Uuid::parse_str(&v).unwrap())),
						],
						label!["Hodnota"],
						input![
							attrs!{
								At::Type => "number",
								At::Placeholder => "Písemka z gravitace",
							},
							input_ev(Ev::Input, Msg::NewGradeNumber),
						],
						label!["Název"],
						input![
							attrs!{
								At::Type => "text",
								At::Placeholder => "Písemka z gravitace",
							},
							input_ev(Ev::Input, Msg::NewGradeName),
						],
						label!["Popis (volitelný)"],
						textarea![
							attrs!{
								At::Placeholder => "detailnější popis..."
							},
							input_ev(Ev::Input, Msg::NewGradeDescription),
						],
						button![
							style![St::MarginTop => "1em"],
							"Zapsat známku"
						],
						raw_ev(Ev::Submit, move |event| {
							event.prevent_default();
							Msg::NewGradeSubmit
						})
					]
				]
			],
			section![
				attrs!{At::Class => "col"},
				div![
					attrs!{At::Class => "card"},
					header![
						h4!["Studenti"]
					],
					form![
						label!["Předmět"],
						select![
							attrs!{At::Value => "..."},
							model.subjects
								.iter()
								.filter(|x| x.teacher == model.teacher.id)
								.map(|x| option![attrs!{At::Value => x.id}, x.name.clone()])
								.collect::<Vec<_>>(),
							input_ev(Ev::Input, |v| Msg::SelectedSubject(Uuid::parse_str(&v).unwrap())),
						]
					],
					ul![
						model.students
							.iter()
							.filter(|x| x.subjects.contains(&model.selected_subject))
							.map(|x| li![x.name.clone()])
							.collect::<Vec<_>>(),
					],
				]
			],
			section![attrs!{At::Class => "col-1"}],
		],
		section![
			attrs!{At::Class => "row"},
			section![attrs!{At::Class => "col-1"}],
			section![
				attrs!{At::Class => "col"},
				div![
					attrs!{At::Class => "card"},
					header![
						h4!["Přehled známek"]
					],
					form![
						div![
							attrs!{At::Class => "row"},
							div![
								attrs!{At::Class => "col"},
								select![
									attrs!{At::Value => model.grade_view.subject},
									model.subjects
										.iter()
										.filter(|x| x.teacher == model.teacher.id)
										.map(|x| option![attrs!{At::Value => x.id}, x.name.clone()])
										.collect::<Vec<_>>(),
									input_ev(Ev::Input, |v| Msg::GradeViewSubject(Uuid::parse_str(&v).unwrap())),
								]
							],
							div![
								attrs!{At::Class => "col"},
								select![
									attrs!{At::Value => "..."},
									model.students
										.iter()
										.filter(|x| x.subjects.contains(&model.grade_view.subject))
										.map(|x| option![attrs!{At::Value => x.id}, x.name.clone()])
										.collect::<Vec<_>>(),
									input_ev(Ev::Input, |v| Msg::GradeViewStudent(Uuid::parse_str(&v).unwrap())),
								]
							],
							div![
								attrs!{At::Class => "col"},
								select![
									attrs!{At::Value => "Regular"},
									option![attrs!{At::Value => "Regular"}, "Známka"],
									option![attrs!{At::Value => "Bonus"}, "Bonus"],
									option![attrs!{At::Value => "Penalisation"}, "Penalizace"],
									input_ev(Ev::Input, Msg::GradeViewKind),
								],
							]
						],
					],
					match model.grade_view.kind.as_str() {
						"Regular" => grades(model.grades
							.iter()
							.filter(|x| x.subject == model.grade_view.subject)
							.filter(|x| x.student == model.grade_view.student)
							.filter(|x| x.is_grade())
							.cloned()
							.collect::<Vec<_>>()),
						"Bonus" => bonus(model.grades
							.iter()
							.filter(|x| x.subject == model.grade_view.subject)
							.filter(|x| x.student == model.grade_view.student)
							.filter(|x| x.is_bonus())
							.cloned()
							.collect::<Vec<_>>()),
						"Penalisation" => penalisation(model.grades
							.iter()
							.filter(|x| x.subject == model.grade_view.subject)
							.filter(|x| x.student == model.grade_view.student)
							.filter(|x| x.is_penalisation())
							.cloned()
							.collect::<Vec<_>>()),
						_ => br![],
					}
				]
			],
			section![attrs!{At::Class => "col-1"}],
		]
	]
}
