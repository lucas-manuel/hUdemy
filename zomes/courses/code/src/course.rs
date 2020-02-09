use hdk::prelude::*;
use hdk::AGENT_ADDRESS;

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct Course {
    title: String,
    teacher_address: Address,
    modules: Vec<Address>,
    timestamp: u64,
}

pub fn entry_definition() -> ValidatingEntryType {
    entry!(
        name: "course",
        description: "this is a course definition",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Course>| {
            match validation_data{
                EntryValidationData::Create { entry, validation_data } => {
                    // Homework: add a validation rule that the title can only contain 50 chars or less
                    // validate_course()
                    Ok(())
                },
                    => Ok(())
            }
        }
    )
}

// fun validate_course() -> ZomeApiResult<bool> {

// }

pub fn create(title: String, timestamp: u64) -> ZomeApiResult<Address> {
    let teacher = AGENT_ADDRESS.clone();
    Err(ZomeApiError::from(String::from("Do your homework please")));
}

pub fn get_course(course_address: Address) -> ZomeApiResult<Option<Entry>> {
    // Homework: finish the get course call
    // HintL use hdk::get_entry
}

pub fn delete_course(course_address: Address) -> ZomeApiResult<Address> {
    // Homework: finish the delete course call
    // HintL use hdk::get_entry
}
