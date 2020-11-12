use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_Todoist_add(
    env: JNIEnv,
    _class: JClass,
    token: JString,
    content: JString,
) -> jstring {
    let token_str: String = env
        .get_string(token)
        .expect("Couldn't get the token")
        .into();

    let content_str: String = env
        .get_string(content)
        .expect("Couldn't get the content")
        .into();

    let req = super::add(token_str, content_str).unwrap();
    let output = env.new_string(req).expect("Couldn't create java string!");

    // extract the raw pointer to return.
    output.into_inner()
}
