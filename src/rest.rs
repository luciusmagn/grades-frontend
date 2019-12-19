use crate::{
	Msg,
	Routes,
	NewGradeForm,
	NewSubjectForm,
	StudentLoginForm,
	TeacherLoginForm,
	StudentRegisterForm,
	TeacherRegisterForm,
};
use uuid::Uuid;
use seed::*;

pub(crate) async fn grades_info() -> Result<Msg, Msg> {
	Request::new("/grades")
		.fetch_json_data(Msg::Grades)
		.await
}

pub(crate) async fn subjects_info() -> Result<Msg, Msg> {
	Request::new("/subjects")
		.fetch_json_data(Msg::Subjects)
		.await
}

pub(crate) async fn teachers_info() -> Result<Msg, Msg> {
	Request::new("/teachers")
		.fetch_json_data(Msg::Teachers)
		.await
}

pub(crate) async fn students_info() -> Result<Msg, Msg> {
	Request::new("/students")
		.fetch_json_data(Msg::Students)
		.await
}

pub(crate) async fn fetch_student() -> Result<Msg, Msg> {
	let store = seed::storage::get_storage().unwrap();
	let tok: String = store.get_item("tok").unwrap().unwrap();

	Request::new("/me")
		.header("Authorization", &format!("Bearer {}", tok))
		.fetch_json_data(Msg::Student)
		.await 
}

pub(crate) async fn fetch_teacher() -> Result<Msg, Msg> {
	let store = seed::storage::get_storage().unwrap();
	let tok: String = store.get_item("tok").unwrap().unwrap();

	Request::new("/me")
		.header("Authorization", &format!("Bearer {}", tok))
		.fetch_json_data(Msg::Teacher)
		.await 
}

pub(crate) async fn my_description(s: String) -> Result<Msg, Msg> {
	let store = seed::storage::get_storage().unwrap();
	let tok: String = store.get_item("tok").unwrap().unwrap();

	Request::new("/my_description")
		.method(Method::Post)
		.header("Authorization", &format!("Bearer {}", tok))
		.header("Content-Type", "application/json")
		.send_json(&s)
		.fetch(|_| Msg::Alert("úspěšně změněno".into()))
		.await 
}

pub(crate) async fn sign_up_for_subject(id: Uuid) -> Result<Msg, Msg> {
	let store = seed::storage::get_storage().unwrap();
	let tok: String = store.get_item("tok").unwrap().unwrap();

	Request::new("/subject/sign_up")
		.method(Method::Post)
		.header("Authorization", &format!("Bearer {}", tok))
		.send_json(&id)
		.fetch(|_| Msg::FetchStudent)
		.await
}

pub(crate) async fn register_student(student_form: StudentRegisterForm) -> Result<Msg, Msg> {
	Request::new("/register_student")
		.method(Method::Post)
		.send_json(&student_form)
		.fetch(|res| if res.response().is_ok() {
			Msg::Route(Routes::Index)
		} else { Msg::Alert("Registrace selhala ".into())  })
		.await
}

pub(crate) async fn register_teacher(teacher_form: TeacherRegisterForm) -> Result<Msg, Msg> {
	Request::new("/register_teacher")
		.method(Method::Post)
		.send_json(&teacher_form)
		.fetch(|res| if res.response().is_ok() {
			Msg::Route(Routes::Index)
		} else { Msg::Alert("Registrace selhala ".into())  })
		.await
}

pub(crate) async fn login_student(student_form: StudentLoginForm) -> Result<Msg, Msg> {
	Request::new("/login_student")
		.method(Method::Post)
		.send_json(&student_form)
		.fetch_json_data(|res: Result<String, fetch::FailReason<String>>| if let Ok(tok) = res {
			let store = seed::storage::get_storage().unwrap();
			seed::storage::store_data::<String>(&store, "tok", &tok.replace("\"", ""));
			seed::storage::store_data::<String>(&store, "typ", &"student".into());
			Msg::FetchStudent
		} else {
			Msg::Alert("Přihlášení selhalo".into())
		})
		.await
}

pub(crate) async fn login_teacher(teacher_form: TeacherLoginForm) -> Result<Msg, Msg> {
	Request::new("/login_teacher")
		.method(Method::Post)
		.send_json(&teacher_form)
		.fetch_json_data(|res: Result<String, fetch::FailReason<String>>| if let Ok(tok) = res {
			let store = seed::storage::get_storage().unwrap();
			seed::storage::store_data::<String>(&store, "tok", &tok.replace("\"", ""));
			seed::storage::store_data::<String>(&store, "typ", &"teacher".into());
			Msg::FetchTeacher
		} else {
			Msg::Alert("Přihlášení selhalo".into())
		})
		.await
}

pub(crate) async fn new_subject(subject: NewSubjectForm) -> Result<Msg, Msg> {
	let store = seed::storage::get_storage().unwrap();
	let tok: String = store.get_item("tok").unwrap().unwrap();

	Request::new("/subject")
		.method(Method::Post)
		.header("Authorization", &format!("Bearer {}", tok))
		.send_json(&subject)
		.fetch(|_| Msg::Alert("Předmět vytvořen".into()))
		.await
}

pub(crate) async fn new_grade(grade: NewGradeForm) -> Result<Msg, Msg> {
	let store = seed::storage::get_storage().unwrap();
	let tok: String = store.get_item("tok").unwrap().unwrap();

	Request::new("/grade")
		.method(Method::Post)
		.header("Authorization", &format!("Bearer {}", tok))
		.send_json(&grade)
		.fetch(|_| Msg::Alert("Známka zapsána".into()))
		.await
}
