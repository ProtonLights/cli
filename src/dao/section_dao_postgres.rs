use project_types::Section;
use dao::{SectionDao, DaoPostgres};
use error::Error;


impl SectionDao for DaoPostgres {

    #[allow(unused_variables)]
    fn get_section(&self, secid: u32) -> Result<Section, Error> {
        Err(Error::TodoErr)
    }
}
