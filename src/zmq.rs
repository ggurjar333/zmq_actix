use std::thread;

pub fn start_zmq_listener() {
    thread::spawn(|| {
        let context =  zmq::Context::new();
        let socket = context.socket(zmq::SUB).unwrap();
        socket.connect("tcp://localhost:5555").unwrap();
        socket.set_subscribe(b"").unwrap();

        loop {
            let msg = socket.recv_msg(0).unwrap();
            let message = msg.as_str().unwrap();
            println!("Received message: {}", message);
        }
    });
}


pub fn send_zmq_message(message: &str) {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::PUB).unwrap();
    socket.bind("tcp://*.5555").unwrap();
    socket.send(message, 0).unwrap();
}
