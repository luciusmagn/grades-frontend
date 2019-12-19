use uuid::Uuid;
use serde::{Serialize, Deserialize};
use seed::{browser::service::fetch, prelude::*, *};

extern crate serde;
extern crate chrono;
extern crate uuid;
extern crate rsc;

mod auth;
mod rest;
mod index;
mod grades;
mod console;
mod subjects;
mod teachers;

mod models;

use rest::{
	grades_info,
	subjects_info,
	teachers_info,
	fetch_student,
	sign_up_for_subject,
	register_student,
};

#[derive(Clone, Debug)]
enum Routes {
	Index,
	Auth,
	Grades,
	Subjects,
	Teachers,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct StudentLoginForm {
	email: String,
	pass: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct TeacherLoginForm {
	email: String,
	pass: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct StudentRegisterForm {
	name: String,
	email: String,
	pass: String,
	pass_again: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct TeacherRegisterForm {
	name: String,
	email: String,
	pass: String,
	pass_again: String,
}

#[derive(Debug)]
struct Model {
	// general
	route: Routes,

	// fetched data
	grades: Vec<models::Grade>,
	subjects: Vec<models::Subject>,
	teachers: Vec<models::Teacher>,
	student: models::Student,

	//
	student_login: StudentLoginForm,
	teacher_login: TeacherLoginForm,
	student_register: StudentRegisterForm,
	teacher_register: TeacherRegisterForm,
}

impl Default for Model {
	fn default() -> Self {
		Self {
			route: Routes::Index,
			grades: vec![],
			subjects: vec![],
			teachers: vec![],
			student: models::Student::default(),
			student_login: StudentLoginForm::default(),
			teacher_login: TeacherLoginForm::default(),
			student_register: StudentRegisterForm::default(),
			teacher_register: TeacherRegisterForm::default(),
		}
	}
}


#[derive(Clone)]
enum Msg {
	Route(Routes),
	SignUp(Uuid),
	FetchStudent,

	Student(fetch::ResponseDataResult<models::Student>),
	Grades(fetch::ResponseDataResult<Vec<models::Grade>>),
	Teachers(fetch::ResponseDataResult<Vec<models::Teacher>>),
	Subjects(fetch::ResponseDataResult<Vec<models::Subject>>),

	StudentRegisterName(String),
	StudentRegisterEmail(String),
	StudentRegisterPass(String),
	StudentRegisterPassAgain(String),
	StudentRegister,

	TeacherRegisterName(String),
	TeacherRegisterEmail(String),
	TeacherRegisterPass(String),
	TeacherRegisterPassAgain(String),
	TeacherRegister,

	StudentLoginEmail(String),
	StudentLoginPass(String),
	StudentLogin,

	TeacherLoginEmail(String),
	TeacherLoginPass(String),
	TeacherLogin,
}

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
	orders.perform_cmd(grades_info());
	orders.perform_cmd(subjects_info());
	orders.perform_cmd(teachers_info());
	AfterMount::default()
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Route(r) 			=> model.route = r,
		Msg::SignUp(id) 		=> {orders.perform_cmd(sign_up_for_subject(id));},
		Msg::FetchStudent       => {orders.perform_cmd(fetch_student());},

		Msg::Grades(Ok(grades)) => model.grades = grades,
		Msg::Grades(Err(f))     => console_log!("grades failed: {:?}", f),

		Msg::Subjects(Ok(subjects)) => model.subjects = subjects,
		Msg::Subjects(Err(f))     => console_log!("subjects failed: {:?}", f),

		Msg::Teachers(Ok(teachers)) => model.teachers = teachers,
		Msg::Teachers(Err(f))     => console_log!("teachers failed: {:?}", f),

		Msg::Student(Ok(student)) => model.student = student,
		Msg::Student(Err(f))     => console_log!("student failed: {:?}", f),

		Msg::StudentRegisterName(val) 	   => model.student_register.name = val,
		Msg::StudentRegisterEmail(val) 	   => model.student_register.email = val,
		Msg::StudentRegisterPass(val) 	   => model.student_register.pass = val,
		Msg::StudentRegisterPassAgain(val) => model.student_register.pass_again = val,
		Msg::StudentRegister => {orders.perform_cmd(register_student(model.student_register.clone()));},

		Msg::TeacherRegisterName(val) 	   => model.teacher_register.name = val,
		Msg::TeacherRegisterEmail(val) 	   => model.teacher_register.email = val,
		Msg::TeacherRegisterPass(val) 	   => model.teacher_register.pass = val,
		Msg::TeacherRegisterPassAgain(val) => model.teacher_register.pass_again = val,
		Msg::TeacherRegister => console_log!("{:?}", model),

		Msg::StudentLoginEmail(val) => model.student_login.email = val,
		Msg::StudentLoginPass(val)  => model.student_login.pass = val,
		Msg::StudentLogin => console_log!("{:?}", model),

		Msg::TeacherLoginEmail(val) => model.teacher_login.email = val,	
		Msg::TeacherLoginPass(val)  => model.teacher_login.pass = val,
		Msg::TeacherLogin => console_log!("{:?}", model),
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
					Routes::Grades => grades::view(model),
					Routes::Subjects => subjects::view(model),
					Routes::Teachers => teachers::view(model),
				}
			}
		],
	]
}

#[wasm_bindgen(start)]
pub fn render() {
	App::builder(update, view)
		.after_mount(after_mount)
		.build_and_start();
}
