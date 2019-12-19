use uuid::Uuid;
use serde::{Serialize, Deserialize};
use seed::{browser::service::fetch, prelude::*, *};

extern crate serde;
extern crate chrono;
extern crate uuid;
extern crate rsc;

mod auth;
mod rest;
mod admin;
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
	students_info,
	fetch_student,
	fetch_teacher,
	sign_up_for_subject,
	register_student,
	register_teacher,
	login_student,
	login_teacher,
	my_description,
	new_grade,
	new_subject,
};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Clone, Debug)]
enum Routes {
	Index,
	Auth,
	Grades,
	Subjects,
	Teachers,
	Admin,
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct NewSubjectForm {
	name: String,
	description: String,
	grade_formula: String,
	year: String,
	kind: models::Kind,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct NewGradeForm {
	typ: String,
	name: String,
	date: String,
	subject: Uuid,
	number: String,
	student: Uuid,
	description: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct GradeView {
	subject: Uuid,
	student: Uuid,
	kind: String,
}

#[derive(Debug)]
struct Model {
	// general
	route: Routes,

	// fetched data
	grades: Vec<models::Grade>,
	subjects: Vec<models::Subject>,
	teachers: Vec<models::Teacher>,
	students: Vec<models::Student>,
	student: models::Student,
	teacher: models::Teacher,

	// auth
	student_login: StudentLoginForm,
	teacher_login: TeacherLoginForm,
	student_register: StudentRegisterForm,
	teacher_register: TeacherRegisterForm,

	// administrace
	my_description: String,
	new_subject: NewSubjectForm,
	new_grade: NewGradeForm,
	selected_subject: Uuid,
	grade_view: GradeView,
}

impl Default for Model {
	fn default() -> Self {
		Self {
			route: Routes::Index,
			grades: vec![],
			subjects: vec![],
			teachers: vec![],
			students: vec![],
			student: models::Student::default(),
			teacher: models::Teacher::default(),
			student_login: StudentLoginForm::default(),
			teacher_login: TeacherLoginForm::default(),
			student_register: StudentRegisterForm::default(),
			teacher_register: TeacherRegisterForm::default(),
			my_description: String::new(),
			new_subject: NewSubjectForm::default(),
			new_grade: NewGradeForm::default(),
			selected_subject: Uuid::default(),
			grade_view: GradeView::default(),
		}
	}
}


#[derive(Clone, Debug)]
enum Msg {
	Route(Routes),
	SignUp(Uuid),
	FetchStudent,
	FetchTeacher,
	Alert(String),
	Logout,

	Student(fetch::ResponseDataResult<models::Student>),
	Teacher(fetch::ResponseDataResult<models::Teacher>),

	Grades(fetch::ResponseDataResult<Vec<models::Grade>>),
	Teachers(fetch::ResponseDataResult<Vec<models::Teacher>>),
	Students(fetch::ResponseDataResult<Vec<models::Student>>),
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

	MyDescription(String),
	MyDescriptionSubmit,

	NewSubjectName(String),
	NewSubjectDescription(String),
	NewSubjectGradeFormula(String),
	NewSubjectYear(String),
	NewSubjectKind(String),
	NewSubjectSubmit,

	NewGradeType(String),
	NewGradeDate(String),
	NewGradeSubject(Uuid),
	NewGradeStudent(Uuid),
	NewGradeName(String),
	NewGradeDescription(String),
	NewGradeNumber(String),
	NewGradeSubmit,

	SelectedSubject(Uuid),

	GradeViewSubject(Uuid),
	GradeViewStudent(Uuid),
	GradeViewKind(String),
}

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
	orders.perform_cmd(grades_info());
	orders.perform_cmd(subjects_info());
	orders.perform_cmd(teachers_info());
	orders.perform_cmd(students_info());

	let store = seed::storage::get_storage().unwrap();
	if let Some(t) = store.get_item("typ").unwrap() {
		match t.replace('"', "").as_str() {
			"teacher" => {orders.perform_cmd(fetch_teacher());},
			"student" => {orders.perform_cmd(fetch_student());},
			_ => (),
		}
	}

	AfterMount::default()
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	console_log!("{:?}", msg);
	match msg {
		Msg::Route(r) 			=> model.route = r,
		Msg::SignUp(id) 		=> {orders.perform_cmd(sign_up_for_subject(id)); alert("předmět zapsán")},
		Msg::FetchStudent       => {orders.perform_cmd(fetch_student());},
		Msg::FetchTeacher       => {orders.perform_cmd(fetch_teacher());},
		Msg::Alert(s)           => alert(&s),
		Msg::Logout             => {
			let _ = seed::storage::get_storage().unwrap().clear();
			model.route = Routes::Index;
			model.student = models::Student::default();
			model.teacher = models::Teacher::default();
		},

		Msg::Grades(Ok(grades)) => model.grades = grades,
		Msg::Grades(Err(f))     => console_log!("grades failed: {:?}", f),

		Msg::Subjects(Ok(subjects)) => model.subjects = subjects,
		Msg::Subjects(Err(f))     => console_log!("subjects failed: {:?}", f),

		Msg::Teachers(Ok(teachers)) => model.teachers = teachers,
		Msg::Teachers(Err(f))     => console_log!("teachers failed: {:?}", f),
		
		Msg::Students(Ok(students)) => model.students = students,
		Msg::Students(Err(f))     => console_log!("students failed: {:?}", f),

		Msg::Student(Ok(student)) => model.student = student,
		Msg::Student(Err(f))     => console_log!("student failed: {:?}", f),

		Msg::Teacher(Ok(teacher)) => model.teacher = teacher,
		Msg::Teacher(Err(f))     => console_log!("teacher failed: {:?}", f),

		Msg::StudentRegisterName(val) 	   => model.student_register.name = val,
		Msg::StudentRegisterEmail(val) 	   => model.student_register.email = val,
		Msg::StudentRegisterPass(val) 	   => model.student_register.pass = val,
		Msg::StudentRegisterPassAgain(val) => model.student_register.pass_again = val,
		Msg::StudentRegister => {orders.perform_cmd(register_student(model.student_register.clone()));},

		Msg::TeacherRegisterName(val) 	   => model.teacher_register.name = val,
		Msg::TeacherRegisterEmail(val) 	   => model.teacher_register.email = val,
		Msg::TeacherRegisterPass(val) 	   => model.teacher_register.pass = val,
		Msg::TeacherRegisterPassAgain(val) => model.teacher_register.pass_again = val,
		Msg::TeacherRegister => {orders.perform_cmd(register_teacher(model.teacher_register.clone()));},

		Msg::StudentLoginEmail(val) => model.student_login.email = val,
		Msg::StudentLoginPass(val)  => model.student_login.pass = val,
		Msg::StudentLogin => {orders.perform_cmd(login_student(model.student_login.clone()));},

		Msg::TeacherLoginEmail(val) => model.teacher_login.email = val,	
		Msg::TeacherLoginPass(val)  => model.teacher_login.pass = val,
		Msg::TeacherLogin => {orders.perform_cmd(login_teacher(model.teacher_login.clone()));},

		Msg::MyDescription(val) => model.my_description = val,
		Msg::MyDescriptionSubmit => {orders.perform_cmd(my_description(model.my_description.clone()));}

		Msg::NewSubjectName(val)		   => model.new_subject.name = val,
		Msg::NewSubjectDescription(val)	   => model.new_subject.description = val,
		Msg::NewSubjectGradeFormula(val)   => model.new_subject.grade_formula = val,
		Msg::NewSubjectYear(val)		   => model.new_subject.year = val,
		Msg::NewSubjectKind(val)		   => model.new_subject.kind = models::Kind::new(&val),
		Msg::NewSubjectSubmit              => {orders.perform_cmd(new_subject(model.new_subject.clone()));},
		
		Msg::NewGradeType(val)     		  => model.new_grade.typ = val,
		Msg::NewGradeDate(val)     		  => model.new_grade.date = val,
		Msg::NewGradeSubject(val)  		  => model.new_grade.subject = val,
		Msg::NewGradeStudent(val)  		  => model.new_grade.student = val,
		Msg::NewGradeName(val)     		  => model.new_grade.name = val,
		Msg::NewGradeDescription(val)     => model.new_grade.description = val,
		Msg::NewGradeNumber(val)          => model.new_grade.number = val,
		Msg::NewGradeSubmit               => {orders.perform_cmd(new_grade(model.new_grade.clone()));},

		Msg::SelectedSubject(val) => model.selected_subject = val,
		
		Msg::GradeViewSubject(val)     => model.grade_view.subject = val,
		Msg::GradeViewStudent(val)     => model.grade_view.student = val,
		Msg::GradeViewKind(val)        => model.grade_view.kind = val,
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
				h1![
					style![St::MarginLeft => "1em"],
					{
					let store = seed::storage::get_storage().unwrap();
					let typ: String = store.get_item("typ").unwrap().unwrap_or_default().replace('"', "");

					match typ.as_ref() {
						"student" => model.student.name.clone(),
						"teacher" => model.teacher.name.clone(),
						_ => "".into(),
					}
				}],
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
					a_style.clone(),
					simple_ev(Ev::Click, Msg::Route(Routes::Teachers)),
					"Učitelé"
				],
				if seed::storage::get_storage()
					.unwrap()
					.get_item("typ")
					.unwrap()
					.unwrap_or_default()
					.replace("\"", "") == "teacher"
				{
					a![
						a_style.clone(),
						simple_ev(Ev::Click, Msg::Route(Routes::Admin)),
						"Administrace",
					]
				} else { br![] },
				if seed::storage::get_storage().unwrap().get_item("typ").unwrap().is_some() {
					a![
						a_style,
						simple_ev(Ev::Click, Msg::Logout),
						"Odhlásit se"
					]
				} else { br![] }
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
					Routes::Admin => admin::view(model),
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
