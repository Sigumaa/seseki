use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Default)]
struct SeisekiCalc {
    kadai: Option<usize>,
    sikenratio: Option<usize>,
    ans: Option<usize>,
}

enum Message {
    FieldKadai(String),
    FieldSikenRatio(String),
    Run,
}

#[allow(unused_mut)]
impl Component for SeisekiCalc {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::FieldKadai(kadai) => {
                self.kadai = kadai.trim().parse().ok();
                false
            }
            Message::FieldSikenRatio(sikenratio) => {
                self.sikenratio = sikenratio.trim().parse().ok();
                false
            }
            Message::Run => {
                if let Self {
                    kadai: Some(mut kadai),
                    sikenratio: Some(mut sikenratio),
                    ..
                } = self
                {
                    let answer = ((60 - kadai) * 10) / sikenratio;
                    self.ans = Some(answer);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <div>
                <p>{"課題点"}</p>
                <input type="number" oninput={ctx.link().callback(|e: InputEvent| Message::FieldKadai(e.target_unchecked_into::<HtmlInputElement>().value()))} />
                <p>{"試験の割合"}</p>
                <input type="number" oninput={ctx.link().callback(|e: InputEvent| Message::FieldSikenRatio(e.target_unchecked_into::<HtmlInputElement>().value()))} />
                <br /><br />
                <button onclick={ctx.link().callback(|_| Message::Run)}>{"Run"}</button>
            </div>
            <div>
                if let Some(ans) = self.ans {
                    <b>{ans}</b>
                }
            </div>
            <div>
            <h2>{"次の試験でとらないとならない点数をもとめます．"}</h2>
            <h2>{"計算されない場合は再読み込みしてください"}</h2>
            </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<SeisekiCalc>();
}
