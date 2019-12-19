use crate::{
	Msg,
	Routes,
	StudentRegisterForm,
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

pub(crate) async fn fetch_student() -> Result<Msg, Msg> {
	let store = seed::storage::get_storage().unwrap();
	let id: String = store.get_item("id").unwrap().unwrap();

	Request::new(format!("/student/{}", id))
		.fetch_json_data(Msg::Student)
		.await 
}

pub(crate) async fn sign_up_for_subject(id: Uuid) -> Result<Msg, Msg> {
	let store = seed::storage::get_storage().unwrap();
	let tok: String = store.get_item("tok").unwrap().unwrap();

	Request::new("/subject/sign-up")
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
		.fetch(|_| Msg::Route(Routes::Index))
		.await
}
