use smithay_client_toolkit::{
                    compositor::{CompositorState, CompositorHandler},
                    output::{OutputState, OutputHandler},
                    seat::{SeatState, SeatHandler},
                    shm::{Shm, ShmHandler},
                    registry::{RegistryState, RegistryHandler}
                    };

use super::search_element;



/**
 * Window struct
 *
 * Struct that represents the drawn window. Should only exist once.
 */
pub struct Window {

    registry_state : RegistryState,
    output_state : OutputState,
    compositor_state : CompositorState,
    shm : Shm,
    seat_state : SeatState,



    exists : bool,
    has_keyboard : bool,
    data : Vec<search_element::SearchElement>,
    cur_search : Vec<char>,
    highlighted_pos : u8,
}



impl Window {


    fn draw(&mut self) {
        //TODO
    }
}
