use smithay_client_toolkit::{self, compositor::{CompositorHandler, CompositorState}, delegate_compositor, delegate_keyboard, delegate_layer, delegate_output, delegate_registry, delegate_seat, delegate_shm, output::{OutputHandler, OutputState}, registry::{ProvidesRegistryState, RegistryHandler, RegistryState}, registry_handlers, seat::{keyboard::{KeyCode, KeyEvent, KeyboardHandler, Keysym}, 
                         Capability, SeatHandler, SeatState}, shell::{wlr_layer::{LayerShell, LayerShellHandler, LayerSurface, LayerSurfaceConfigure}, xdg::window::{Window, WindowHandler}, WaylandSurface}, shm::{slot::{Buffer, SlotPool}, Shm, ShmHandler}
                    };


use wayland_client::{protocol::{wl_keyboard, wl_seat, wl_shm, wl_surface}, Connection, QueueHandle};

use super::search_element;

use std::slice;





/**
 * Window struct
 *
 * Struct that represents the drawn window. Should only exist once.
 */
pub struct OrkanWindow {

    pub registry_state : RegistryState,
    pub output_state : OutputState,
    pub shm : Shm,
    pub seat_state : SeatState,
    pub layer_surface: LayerSurface,
    pub pool : SlotPool,

    pub width : u32,
    pub height : u32,
    pub shift : Option<u32>,
    pub keyboard : Option<wl_keyboard::WlKeyboard>,
    pub buffer : Option<Buffer>,
    pub first_configure : bool,

    pub need_update : bool,

    pub padding_abs : i32,




    pub padding_rel : f32, //TODO: Make Padding proportional
    pub exists : bool,
    pub has_keyboard : bool,
    pub data : Vec<search_element::SearchElement>,
    pub cur_search : Vec<char>,
    pub highlighted_pos : usize,
}



impl OrkanWindow {


    fn draw(&mut self, _conn: &Connection, qh: &QueueHandle<Self>) {

        //println!("Drawing");

            let (width, height) = (self.width, self.height);

       //     println!("Width: {width}, Height: {height}");
            let buffer = self.buffer.get_or_insert_with(|| {
                self.pool.create_buffer(width as i32, height as i32, self.width as i32 * 4, wl_shm::Format::Argb8888).expect("create Buffer").0
            });


            let canvas = match self.pool.canvas(buffer) {
                Some(canvas) => canvas,

                None => {
                    let (second, canvas) = self.pool.create_buffer(width as i32, height as i32, self.width as i32 * 4, wl_shm::Format::Argb8888).expect("create Buffer");

                    *buffer = second;
                    canvas
                }
            };

            canvas.fill(0xff);

            let color : u32 = (0xff << 24) | (0x00) << 16 | (0x00) << 8 | 0x00;

            canvas[4..8].copy_from_slice(color.to_le_bytes().as_ref());
            canvas[8..12].copy_from_slice(color.to_le_bytes().as_ref());
            canvas[12..16].copy_from_slice(color.to_le_bytes().as_ref());
            canvas[16..20].copy_from_slice(color.to_le_bytes().as_ref());



            //println!("Canvas: {}", canvas.len());
            canvas[3] = 0xff;
            //Apparently canvas is simply a &mut [u8], ie a slice to some bytes, which i can modify as
            //i please... i may want to split it in chunks.

            self.layer_surface.wl_surface().damage_buffer(0,0, width as i32, height as i32);

            self.layer_surface.wl_surface().frame(qh, self.layer_surface.wl_surface().clone());

            buffer.attach_to(self.layer_surface.wl_surface()).expect("buffer attach");

            self.layer_surface.commit();

            self.need_update = false;
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
        println!("Keyboard Entered");

    }

    fn leave(
        &mut self, 
        _: &Connection, 
        _: &QueueHandle<Self>, 
        _: &wl_keyboard::WlKeyboard, 
        surface: &wl_surface::WlSurface, 
        _: u32) {

        if self.layer_surface.wl_surface() == surface {
            self.has_keyboard = false;
        }

        println!("Keyboard leave");

    }


    fn press_key(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _keyboard: &wl_keyboard::WlKeyboard,
            _serial: u32,
            _event: KeyEvent,
        ) {

        self.need_update = true;
        if _event.keysym == Keysym::Escape {
            self.exists = false;
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
        else if let Some(key) = _event.keysym.key_char() { 
            self.cur_search.push(key);
            //TODO: Sort List and Redraw
            println!("Key Pressed: {key:?}");
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

            println!("Keyboard Added");
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
            println!("Keyboard Removed");
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
        //println!("Drawing Frame");
        self.draw(conn, qh);
    }

    fn surface_enter(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _surface: &wl_surface::WlSurface,
            _output: &wayland_client::protocol::wl_output::WlOutput,
        ) {

        let (width, _) = self.output_state.info(_output).unwrap().logical_size.unwrap();


        self.width = if width < 20 {
            self.width
        } else {
                self.padding_abs = (width as f32 * self.padding_rel).round() as i32;
                println!("Padding = {}", self.padding_abs);
                width as u32 - 2 * self.padding_abs as u32
        };

        println!("New Monitor = {}*{}", self.width, self.height);
        self.need_update = true;
        // Resetting some default like margins Original in configure
        self.layer_surface.set_size(self.width, self.height);
        println!("New Margins: Top = 20, left = {}", self.padding_abs);
        self.layer_surface.set_margin(20, 0, 0, self.padding_abs);
        self.draw(_conn, _qh);
        
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


impl  LayerShellHandler for OrkanWindow {

    fn closed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _layer: &LayerSurface) {

        self.exists = false;
        
    }

    fn configure(
            &mut self,
            conn: &Connection,
            qh: &QueueHandle<Self>,
            _layer: &LayerSurface,
            configure: LayerSurfaceConfigure,
            _serial: u32,
        ) {

        //println!("Configure");

        self.need_update = true;

        if configure.new_size.0 == 0 || configure.new_size.1 == 0 {
            self.height = 20;
        } else {
            println!("Size determined by Hyprland");
            self.width = configure.new_size.0;
            self.height = configure.new_size.1;
        }



        if self.first_configure {
            self.first_configure = false;
            println!("First Configure");
            self.draw(conn, qh);
        }
        
    }
    
}
delegate_keyboard!(OrkanWindow);
delegate_compositor!(OrkanWindow);
delegate_layer!(OrkanWindow);
delegate_seat!(OrkanWindow);
delegate_output!(OrkanWindow);
delegate_registry!(OrkanWindow);
delegate_shm!(OrkanWindow);

impl ProvidesRegistryState for OrkanWindow {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState, SeatState];
}


