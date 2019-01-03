/*
const KEYSPACE_NAME: &'static str = "user_ks";

const EVENT_TABLE_NAME: &'static str= "user_events";

const STATE_TABLE_NAME: &'static str= "user_state";

pub const KEYSPACE_QUERY: &str = format!("CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{\
                                 'class' : 'SimpleStrategy', 'replication_factor' : 1 }};",KEYSPACE_NAME).as_str();

pub const EVENT_TABLE_QUERY: &str = format!("CREATE TABLE IF NOT EXISTS {}.{}(user_id uuid PRIMARY KEY , \
     user_event text);", KEYSPACE_NAME, EVENT_TABLE_NAME).as_str();

pub const STATE_TABLE_QUERY: &str = format!("CREATE TABLE IF NOT EXISTS {}.{} (user_id uuid PRIMARY KEY , \
     user_state text);", KEYSPACE_NAME, STATE_TABLE_NAME).as_str();*/

pub const KEYSPACE_QUERY: &str = "CREATE KEYSPACE IF NOT EXISTS user_ks WITH REPLICATION = {\
                                 'class' : 'SimpleStrategy', 'replication_factor' : 1 };";

pub const EVENT_TABLE_QUERY: &str = "CREATE TABLE IF NOT EXISTS user_ks.user_events\
                                    (user_id text PRIMARY KEY , user_event text);";

pub const STATE_TABLE_QUERY: &str = "CREATE TABLE IF NOT EXISTS user_ks.user_states \
                                     (user_id text PRIMARY KEY ,user_state text);";

pub const USER_EVENT_STORE_QUERY: &str = "INSERT INTO user_ks.user_events (user_id,user_event) \
        VALUES (?,?)";

pub const USER_STATE_STORE_QUERY: &str = "INSERT INTO user_ks.user_states (user_id,user_state) \
        VALUES (?,?)";

pub const SELECT_QUERY: &str = "SELECT * FROM user_ks.user_states WHERE user_id = ? ";

pub const SELECT_ALL_QUERY: &str = "SELECT * FROM user_ks.user_states";

