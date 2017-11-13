
// DAO traits/interfaces
mod daos;

pub use self::daos::ProtonDao;

pub use self::daos::ChannelDao;
pub use self::daos::DataDao;
pub use self::daos::FixtureDao;
pub use self::daos::LayoutDao;
pub use self::daos::PermissionDao;
pub use self::daos::ProjectDao;
pub use self::daos::SectionDao;
pub use self::daos::SequenceDao;
pub use self::daos::UserDao;

// Postgres implementations
mod daos_postgres;
pub use self::daos_postgres::DaoPostgres;

// Connection configuration
mod connection_config;
use self::connection_config::ConnectionConfig;

// Load postgres implementations to show that ProtonDao is satisfied
mod channel_dao_postgres;
mod data_dao_postgres;
mod fixture_dao_postgres;
mod layout_dao_postgres;
mod permission_dao_postgres;
mod project_dao_postgres;
mod section_dao_postgres;
mod sequence_dao_postgres;
mod user_dao_postgres;

// Make DaoPostgres conform to the ProtonDao interface (follow all Daos)
impl ProtonDao for DaoPostgres {}
