pub const KEYSPACE_QUERY: &str = "CREATE KEYSPACE IF NOT EXISTS item_ks WITH REPLICATION = {\
                                 'class' : 'SimpleStrategy', 'replication_factor' : 1 };";

pub const EVENT_TABLE_QUERY: &str = "CREATE TABLE IF NOT EXISTS item_ks.item_events\
                                    (user_id text PRIMARY KEY,item_id text , item_event text);";

pub const STATE_TABLE_QUERY: &str = "CREATE TABLE IF NOT EXISTS item_ks.item_states \
                                     (user_id text PRIMARY KEY , item_id text ,item_state text);";

pub const ITEM_EVENT_STORE_QUERY: &str = "INSERT INTO item_ks.item_events (user_id,item_id,item_event) \
        VALUES (?,?)";

pub const ITEM_STATE_STORE_QUERY: &str = "INSERT INTO item_ks.item_states (user_id,item_id,item_state) \
        VALUES (?,?)";

pub const SELECT_QUERY: &str = "SELECT * FROM item_ks.item_states WHERE user_id = ? ";

pub const SELECT_ALL_QUERY: &str = "SELECT * FROM item_ks.item_states";