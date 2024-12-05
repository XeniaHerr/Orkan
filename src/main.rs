mod orkan;

use orkan::search_element;
use orkan::window;

use smithay_client_toolkit::compositor::CompositorState;
use smithay_client_toolkit::output::OutputState;
use smithay_client_toolkit::registry::RegistryState;
use smithay_client_toolkit::seat::SeatState;
use smithay_client_toolkit::shell::wlr_layer::Layer;
use smithay_client_toolkit::shell::wlr_layer::LayerShell;

use smithay_client_toolkit::shell::WaylandSurface;
use smithay_client_toolkit::shm::slot::SlotPool;
use smithay_client_toolkit::shm::Shm;
use wayland_client;
use wayland_client::globals;
use wayland_client::Connection;
use smithay_client_toolkit::shell::wlr_layer::Anchor;






fn main() {

    println!("Staring Orkan");

    let conn : Connection = Connection::connect_to_env().unwrap();

    

    let (globals, mut event_queue) = globals::registry_queue_init(&conn).unwrap();


    let qh   = event_queue.handle();

    let compositor = CompositorState::bind(&globals, &qh).expect("Compositor is not available");

    let layer_shell = LayerShell::bind(&globals, &qh).expect("Layer shell is not supported");

    let shm = Shm::bind(&globals, &qh).expect("Shm is not available");

    let surface = compositor.create_surface(&qh);

    let layer = layer_shell.create_layer_surface(&qh, surface, Layer::Top, Some("OrkanWindow"), None);

    layer.set_anchor(Anchor::TOP|Anchor::LEFT);
    layer.set_keyboard_interactivity(smithay_client_toolkit::shell::wlr_layer::KeyboardInteractivity::Exclusive);
    layer.set_size(400, 20);
    layer.set_margin(10, 0, 0, 50);
    layer.commit();

    let pool = SlotPool::new(400* 20 *4, &shm).expect("Failed to create pool");
    println!("Setup done");

    let mut window = window::OrkanWindow {
        registry_state : RegistryState::new(&globals),
        seat_state : SeatState::new(&globals, &qh),
        output_state : OutputState::new(&globals, &qh),
        shm : shm, 

        exists : true,

        pool : pool,
        width : 400,
        height : 20,

        keyboard : None,

 //       compositor_state : compositor,

        buffer : None,

        cur_search : Vec::new(),

        data : search_element::get_binaries(),

        has_keyboard : false,
//        layer_shell : layer_shell,
        layer_surface : layer,

        shift : None,

        highlighted_pos : 0,

        first_configure : true,

        padding_rel: 0.1,
        padding_abs: 0,


        need_update : true,


    };

    println!("Window created");
    loop {
        event_queue.blocking_dispatch(&mut window).unwrap();

        if !window.exists {
            break;
        }

        //println!("Loop iterated");
    }


}

