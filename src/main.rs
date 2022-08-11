use yew::prelude::*;
// use std::fs::File;
// use std::io::BufReader;
// use rodio::{Decoder, OutputStream, source::Source, OutputStreamHandle};

mod audio;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
    handle : audio::Handle
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // let (_stream, _stream_handle) = OutputStream::try_default().unwrap();
        // let file1 = BufReader::new(File::open("resources/c4vh").unwrap());
        // // let file2 = BufReader::new(File::open("resources/e4vh").unwrap());
        // // let file3 = BufReader::new(File::open("resources/g4vh").unwrap());
        // let _source = Decoder::new(file1).unwrap();
        // // let _result = _stream_handle.play_raw(source.convert_samples());

        // std::thread::sleep(std::time::Duration::from_secs(5));
        Self {
            value: 0,
            handle: audio::init()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                self.handle = audio::beep();
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}