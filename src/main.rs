use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use node::stun::connect::*;
use node::config::init::*;
use signaling::call::*;
use tokio;
use hex;
#[tokio::main]
async fn main() {
    // initialize the node
    let node = DcNodeInit::new().await;
    // start the signaling listner
    start_signal(node.address.clone()).await;
    //
    let pbk = hex::encode(node.pub_key_vec);
    //
    let addr = hex::encode(node.address);
    //
    push_text_call(String::from("UNKNOWN"),addr,pbk).await;
    
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        // Don't forget to make all widgets visible.
        win.show_all();
    });

    app.run();

}
