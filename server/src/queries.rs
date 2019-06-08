use diesel;
use super::models::{NewUser};

// INSERT INTO example_table
//     (id, name)
// SELECT 1, 'John'
// WHERE
//     NOT EXISTS (
//         SELECT id FROM example_table WHERE id = 1
//     );
pub fn create_user_if_none_exists(new_user: &NewUser) {
    // diesel::insert_into(users::table)
    //     .values(&new_user)
    //     .get_result::<User>(&*connection)
    //     .expect("Error saving new post");
}
