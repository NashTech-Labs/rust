use eventsourcing::{Aggregate, Result};

use crate::item_eventsourcing::item_command::models::PItemCommand;
use crate::item_eventsourcing::item_event::models::PItemEvent;
use crate::item_eventsourcing::item_state::pitem_state::PItemState;
use crate::models::p_item::PItem;
use crate::models::item_status::PItemStatus;
use crate::models::item_data::PItemData;
use std::time::Duration;

impl Aggregate for PItem {
    type Event = PItemEvent;
    type Command = PItemCommand;
    type State = PItemState;

    /// apply_event takes events and state as input parameter and returns result of states
    fn apply_event(state: &Self::State, evt: Self::Event) -> Result<Self::State> {
        unimplemented!();
        /*let events = evt.to_owned();
                let item_state: PItemState = match evt {
                    PItemEvent::ItemCreated{item} => PItemState {
                        item: Some(item),
                        generation: state.generation + 1,
                    },
                    PItemEvent::ItemUpdated { item_id, creator, item_details, item_status } => PItemState {
                        item: Some(PItem {
                            id: item_id,
                            creator: creator,
                            item_data: item_details,
                            price: 0.0,
                            status: item_status,
                            auction_start: None,
                            auction_end: None,
                            auction_winner: None,
                    }),
                        generation: state.generation + 1,
                    },
                    PItemEvent::AuctionStarted { item_id, start_time } => PItemState {
                        item: Some(PItem {
                            id: item_id,
                            creator: "".to_string(),
                            item_data: PItemData {
                                title: "".to_string(),
                                description: "".to_string(),
                                currency_id: "".to_string(),
                                increment: 0.0,
                                reserve_price: 0.0,
                                auction_duration: Duration::new(5, 0),
                                category_id: None
                            },
                            price: 0.0,
                            status: PItemStatus::NOT_CREATED,
                            auction_start: Some(start_time),
                            auction_end: None,
                            auction_winner: None
                        }),
                        generation: state.generation + 1,
                    },
                    PItemEvent::PriceUpdated { item_id, price } => PItemState {
                        item: Some(PItem {
                            id: item_id,
                            creator: "".to_string(),
                            item_data: PItemData {
                                title: "".to_string(),
                                description: "".to_string(),
                                currency_id: "".to_string(),
                                increment: 0.0,
                                reserve_price: 0.0,
                                auction_duration: Duration::new(5, 0),
                                category_id: None
                            },
                            price: price,
                            status: PItemStatus::NOT_CREATED,
                            auction_start: None,
                            auction_end: None,
                            auction_winner: None
                        }),
                        generation: state.generation + 1,
                    },
                    PItemEvent::AuctionFinished { item_id, winner, price } => PItemState {
                        item:Some(PItem {
                            id: item_id,
                            creator: "".to_string(),
                            item_data: PItemData {
                                title: "".to_string(),
                                description: "".to_string(),
                                currency_id: "".to_string(),
                                increment: 0.0,
                                reserve_price: 0.0,
                                auction_duration: Duration::new(5, 0),
                                category_id: None
                            },
                            price: price,
                            status: PItemStatus::NOT_CREATED,
                            auction_start: None,
                            auction_end: None,
                            auction_winner: winner
                        }),
                        generation: state.generation + 1,
                    },
                };
                Ok(item_state)*/
    }

    /// handle_command takes command and state as input parameter and
    /// returns result of vector of events
    fn handle_command(_state: &Self::State, cmd: Self::Command) -> Result<Vec<Self::Event>> {
        let item_event: PItemEvent = match cmd {
            PItemCommand::CreateItem(pitem) => PItemEvent::ItemCreated { item: pitem },
            PItemCommand::GetItem => PItemEvent::ItemRetrieved,
            PItemCommand::FinishAuction {winner,price}  => PItemEvent::AuctionFinished {
                item_id: 0,
                winner: winner,
                price: price,
            },
            PItemCommand::StartAuction {user_id} => PItemEvent::AuctionStarted { item_id: 0, start_time: "".to_string() },
            PItemCommand::UpdateItem {commander, item_data} => PItemEvent::ItemUpdated {
                item_id: 0,
                creator: commander,
                item_details: item_data,
                item_status: PItemStatus::NOT_CREATED
            },
            PItemCommand::UpdatePrice {price} => PItemEvent::PriceUpdated { item_id: 0, price: price },
        };
        Ok(vec![item_event])
    }
}