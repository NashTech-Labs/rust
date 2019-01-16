/*
use models::p_item::PItem;

pub struct PItemState {
    item: Option<PItem>
}

impl PItemState {

    fn new(pitem:Option<PItem>) -> PItemState{
        PItemState{
            item: pitem,
        }
    }

    pub fn empty() -> PItemState{
         PItemState { item: None }
    }

    pub fn create(item: PItem) -> PItemState {

        PItemState { item: Some(item) }
    }

    pub fn start(start_time: Instant) -> PItemState {
        PItemState::update (|i| i.start(start_time))

    }

    pub fn end(winner: Option<String>, price:f32) -> PItemState {
        PItemState::update(|i| i.end(winner,price))
    }

    pub fn update_price(price:f32) -> PItemState {
        PItemState::update(|i| i.update_price(price))
    }

    pub fn update_details(details: PItemData) -> PItemState {
        PItemState::update(|i| i.with_details(details))
    }

    pub fn cancel() -> PItemState {
        PItemState::update(|i| i.cancel())
    }

    pub fn get_status(&self) -> PItemState {
        self.item.map()
    }

    fn update(pitem: PItem) -> PItem {

    }
}

*/
