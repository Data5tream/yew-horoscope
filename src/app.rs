use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlSelectElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct CalendarProps {
    on_input_change: Callback<AttrValue>,
}

#[function_component]
fn CalendarInput(props: &CalendarProps) -> Html {
    let on_input_change_clone = props.on_input_change.clone();
    let options = vec![
        "Aquarius",
        "Pisces",
        "Aries",
        "Taurus",
        "Gemini",
        "Cancer",
        "Leo",
        "Virgo",
        "Libra",
        "Scorpio",
        "Sagittarius",
        "Capricorn",
    ];

    let onchange = Callback::from(move |e: Event| {
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());

        if let Some(input) = input {
            on_input_change_clone.emit(input.value().into());
        }
    });

    html! {
        <select {onchange}>
        {
            options.into_iter().map(|opt| {
                html!{<option value={opt}>{opt}</option>}
            }).collect::<Html>()
        }
        </select>
    }
}

#[derive(Properties, PartialEq)]
struct ResultProps {
    result: AttrValue,
}

#[function_component]
fn ResultDisplay(props: &ResultProps) -> Html {
    let selected_zodiac = props.result.clone();

    html! {
        <div>
            <p>{"Mysterious "}{&*selected_zodiac}{", your future is:"}</p>
            <h2 style="font-size: 5rem">{"You will die"}</h2>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let app_title = Html::from_html_unchecked(AttrValue::from(
        "<h1>Totally Real Horoscope&reg;&trade;</h1>",
    ));

    let show_result = use_state_eq(|| false);
    let selected_zodiac = use_state_eq(|| AttrValue::from(""));

    let on_input_change: Callback<AttrValue> = {
        let selected_zodiac = selected_zodiac.clone();

        Callback::from(move |value: AttrValue| selected_zodiac.set(value))
    };

    let onclick = {
        let show_result = show_result.clone();
        let selected_zodiac = selected_zodiac.clone();

        Callback::from(move |_| {
            if *selected_zodiac != "" {
                show_result.set(!*show_result)
            }
        })
    };

    html! {
        <main>
            {app_title}
            if *show_result {
                <ResultDisplay result={&*selected_zodiac} />
            } else {
                <p>{"Select your zodiac"}</p>
                <CalendarInput {on_input_change} />
                if *selected_zodiac != "" {
                    <button {onclick}>{ "Read my future!" }</button>
                }
            }
        </main>
    }
}
