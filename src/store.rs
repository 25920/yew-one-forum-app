use serde::{Deserialize, Serialize};
use yewdux::prelude::*;
// use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Feedback {
    pub id: uuid::Uuid,
    pub text: String,
    pub rating: u8,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Post { // anonymous
    pub id: uuid::Uuid,
    pub text: String,
    pub resps: Vec<Comment>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Comment { // anonymous
    pub id: uuid::Uuid,
    pub comment: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct AlertInput {
    pub show_alert: bool,
    pub alert_message: String,
}

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "local", storage_tab_sync)]
pub struct Store {
    pub feedbacks: Vec<Feedback>,
    pub posts: Vec<Post>,
    pub loading: bool,
    pub alert_input: AlertInput,
}

pub fn set_post(post:Post, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store: &mut Store| {
        store.posts.insert(0, post);
    })
}

pub fn set_comment(new_c: Comment, 
    dispatch: Dispatch<Store>, post:Post) {
    dispatch.reduce_mut(move |store: &mut Store| {
        if let Some(index) = store.posts.iter().position(
            |p| p.id == post.id
        ) {
            let mut modified = store.posts[index].clone();
            modified.resps.push(new_c);
            store.posts[index]=modified;
        }
    })
}

pub fn set_feedback(feedback: Feedback, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.feedbacks.insert(0, feedback);
    })
}

pub fn set_loading(loading: bool, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.loading = loading;
    })
}

pub fn set_show_alert(message: String, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input = AlertInput {
            alert_message: message,
            show_alert: true,
        };
    })
}
