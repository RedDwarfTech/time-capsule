

pub fn repo_app_create(request: &Json<AddAppRequest>) {
    use crate::model::diesel::dolphin::dolphin_schema::apps::dsl::*;
    let connection = config::establish_connection();
    let apps_record = apps.order(app_id.desc())
        .limit(1)
        .load::<App>(&connection)
        .expect("query app  failed");
    let app_db = take(apps_record,0).unwrap();

    let current_time = get_current_millisecond();
   
}


