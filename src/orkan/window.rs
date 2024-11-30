use smithay_client_toolkit::{
                    compositor::{CompositorHandler, CompositorState}, delegate_compositor, delegate_keyboard, output::{OutputHandler, OutputState}, registry::{RegistryHandler, RegistryState}, seat::{keyboard::{KeyCode, KeyEvent, KeyboardHandler, Keysym}, 
                         Capability, SeatHandler, SeatState}, shell::{wlr_layer::{LayerShell, LayerShellHandler}, xdg::window::{Window, WindowHandler}, WaylandSurface}, shm::{slot::{Buffer, SlotPool}, Shm, ShmHandler}
                    };


use wayland_client::{protocol::{wl_keyboard, wl_seat, wl_shm, wl_surface}, Connection, QueueHandle};

use super::search_element;





/**
 * Window struct
 *
 * Struct that represents the drawn window. Should only exist once.
 */
pub struct OrkanWindow {

    registry_state : RegistryState,
    output_state : OutputState,
    compositor_state : CompositorState,
    shm : Shm,
    seat_state : SeatState,
    layer_shell: LayerShell,
    pool : SlotPool,

    width : u32,
    height : u32,
    shift : Option<u32>,
    keyboard : Option<wl_keyboard::WlKeyboard>,
    buffer : Option<Buffer>,
    window : Window,




    exists : bool,
    has_keyboard : bool,
    data : Vec<search_element::SearchElement>,
    cur_search : Vec<char>,
    highlighted_pos : usize,
}



impl OrkanWindow {


    fn draw(&mut self, _conn: &Connection, qh: &QueueHandle<Self>) {

        let (width, height) = (self.width, self.height);
        let buffer = self.buffer.get_or_insert_with(|| {
            self.pool.create_buffer(width as i32, height as i32, 0, wl_shm::Format::Argb8888).expect("create Buffer").0
        });


        let canvas = match self.pool.canvas(buffer) {
            Some(canvas) => canvas,

            None => {
                let (second, canvas) = self.pool.create_buffer(width as i32, height as i32, 0, wl_shm::Format::Argb8888).expect("create Buffer");

                *buffer = second;
                canvas
            }
        };

        canvas.fill(0xff);
        canvas[3] = 0xff;
        //Apparently canvas is simply a &mut [u8], ie a slice to some bytes, which i can modify as
        //i please... i may want to split it in chunks.

        self.window.wl_surface().damage_buffer(0,0, width as i32, height as i32);

        self.window.wl_surface().frame(qh, self.window.wl_surface().clone());

        buffer.attach_to(self.window.wl_surface()).expect("buffer attach");
    }


}



impl KeyboardHandler for OrkanWindow {


    fn enter(
        &mut self, 
        _: &Connection, 
        _: &QueueHandle<Self>, 
        _: &wl_keyboard::WlKeyboard, 
        _surface : &wl_surface::WlSurface, 
        _ : u32, 
        _: &[u32], 
        _keysyms: &[Keysym])  {

    }

    fn leave(
        &mut self, 
        _: &Connection, 
        _: &QueueHandle<Self>, 
        _: &wl_keyboard::WlKeyboard, 
        surface: &wl_surface::WlSurface, 
        _: u32) {

        if self.window.wl_surface() == surface {
            self.has_keyboard = false;
        }


    }


    fn press_key(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _keyboard: &wl_keyboard::WlKeyboard,
            _serial: u32,
            _event: KeyEvent,
        ) {

        if let Some(key) = _event.keysym.key_char() { //I don't know if this already matches
                                                      //Backspace and Return
            self.cur_search.push(key);
            //TODO: Sort List and Redraw
            println!("Key Pressed: {key:?}");
        }
        else if _event.keysym == Keysym::BackSpace {

            self.cur_search.pop();
            //TODO: Sort List and Redraw
            println!("Backspace Pressed");
        }

        else if _event.keysym == Keysym::Return {
            //TODO: Handle spawning Process
            println!("Return Pressed");
        }
        
    }


    fn release_key(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _keyboard: &wl_keyboard::WlKeyboard,
            _serial: u32,
            _event: KeyEvent,
        ) {
        //Probably not needed
    }

    fn update_keymap(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _keyboard: &wl_keyboard::WlKeyboard,
            _keymap: smithay_client_toolkit::seat::keyboard::Keymap<'_>,
        ) {
        
    }

    fn update_modifiers(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _keyboard: &wl_keyboard::WlKeyboard,
            _serial: u32,
            _modifiers: smithay_client_toolkit::seat::keyboard::Modifiers,
            _layout: u32,
        ) {
        
    }

}



impl OutputHandler for OrkanWindow {

    fn new_output(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _output: wayland_client::protocol::wl_output::WlOutput,
        ) {
        
    }

    fn output_destroyed(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _output: wayland_client::protocol::wl_output::WlOutput,
        ) {
        
    }
    fn output_state(&mut self) -> &mut OutputState {
       &mut self.output_state
    }

    fn update_output(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _output: wayland_client::protocol::wl_output::WlOutput,
        ) {
        
    }

}



impl SeatHandler for OrkanWindow {

    fn new_seat(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _seat: wl_seat::WlSeat) {
        
    }

    fn new_capability(
            &mut self,
            _conn: &Connection,
            qh: &QueueHandle<Self>,
            seat: wl_seat::WlSeat,
            capability: smithay_client_toolkit::seat::Capability,
        ) {
        if capability == smithay_client_toolkit::seat::Capability::Keyboard && self.keyboard.is_none() {
            let kb = self.seat_state.get_keyboard(qh, &seat, None).expect("Keyboard error");

            self.keyboard = Some(kb);
        }
    }
    
    fn seat_state(&mut self) -> &mut SeatState {
        &mut self.seat_state
    }

    fn remove_seat(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _seat: wl_seat::WlSeat) {
        
    }

    fn remove_capability(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _seat: wl_seat::WlSeat,
            capability: Capability,
        ) {
        
        if capability == smithay_client_toolkit::seat::Capability::Keyboard && self.keyboard.is_some() {
            self.keyboard.take().unwrap().release();
        }
    }
}


impl CompositorHandler for OrkanWindow {

    fn frame(
            &mut self,
            conn: &Connection,
            qh: &QueueHandle<Self>,
            _surface: &wl_surface::WlSurface,
            _time: u32,
        ) {
        self.draw(conn, qh);
    }

    fn surface_enter(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _surface: &wl_surface::WlSurface,
            _output: &wayland_client::protocol::wl_output::WlOutput,
        ) {
        
    }

    fn surface_leave(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _surface: &wl_surface::WlSurface,
            _output: &wayland_client::protocol::wl_output::WlOutput,
        ) {
        
    }

    fn transform_changed(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _surface: &wl_surface::WlSurface,
            _new_transform: wayland_client::protocol::wl_output::Transform,
        ) {
        
    }

    fn scale_factor_changed(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _surface: &wl_surface::WlSurface,
            _new_factor: i32,
        ) {
        
    }
}

impl ShmHandler for OrkanWindow {

    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm
    }
}


impl  WindowHandler for OrkanWindow {

    fn request_close(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &Window) {
        self.exists = false;
    }

    fn configure(
            &mut self,
            conn: &Connection,
            qh: &QueueHandle<Self>,
            window: &Window,
            configure: smithay_client_toolkit::shell::xdg::window::WindowConfigure,
            serial: u32,
        ) {
        
    }
    
}


delegate_keyboard!(OrkanWindow);
delegate_compositor!(OrkanWindow);
