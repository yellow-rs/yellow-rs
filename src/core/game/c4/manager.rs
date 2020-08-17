pub type C4Manager = HashMap<u64, Arc<RwLock<C4Instance>>>;

pub struct C4ManagerContainer;
impl TypeMapKey for C4ManagerContainer {
    type Value = Arc<RwLock<C4Manager>>;
}
