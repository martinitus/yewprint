use crate::{Icon, IconName};
use std::convert::TryInto;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Debug)]
pub struct HtmlSelect<T: Clone + PartialEq + 'static> {
    select_element: NodeRef,
    props: HtmlSelectProps<T>,
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct HtmlSelectProps<T: Clone + PartialEq + 'static> {
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub minimal: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub onchange: Callback<T>,
    #[prop_or_default]
    pub value: Option<T>,
    pub options: Vec<(T, String)>,
    #[prop_or_default]
    pub class: Classes,
}

impl<T: Clone + PartialEq + 'static> Component for HtmlSelect<T> {
    type Message = ChangeData;
    type Properties = HtmlSelectProps<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            select_element: NodeRef::default(),
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let i = if let ChangeData::Select(select) = msg {
            select.selected_index()
        } else {
            unreachable!("unexpected ChangeData variant: {:?}", msg);
        };
        if i >= 0 {
            let i = i as usize;
            let variant = self.props.options[i].0.clone();
            self.props.onchange.emit(variant);
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if props != self.props {
            self.props = props;
            if let Some(value) = self.props.value.as_ref() {
                if let Some(select) = self.select_element.cast::<HtmlSelectElement>() {
                    if let Some(i) = self.props.options.iter().position(|(x, _)| x == value) {
                        if let Ok(i) = i.try_into() {
                            if select.selected_index() != i {
                                select.set_selected_index(i);
                            }
                        }
                    }
                }
            }
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let option_children = self
            .props
            .options
            .iter()
            .map(|(value, label)| {
                let selected = self
                    .props
                    .value
                    .as_ref()
                    .map(|x| value == x)
                    .unwrap_or_default();

                html! {
                    <option selected={selected}>
                        {label}
                    </option>
                }
            })
            .collect::<Html>();

        html! {
            <div
                class=classes!(
                    "bp3-html-select",
                    self.props.minimal.then(|| "bp3-minimal"),
                    self.props.large.then(|| "bp3-large"),
                    self.props.fill.then(|| "bp3-fill"),
                    self.props.disabled.then(|| "bp3-disabled"),
                    self.props.class.clone(),
                )
            >
                <select
                    disabled=self.props.disabled
                    onchange={self.link.callback(|x| x)}
                    title={self.props.title.clone()}
                    value={"".to_string()}
                    ref={self.select_element.clone()}
                >
                    {option_children}
                </select>
                <Icon icon=IconName::DoubleCaretVertical/>
            </div>
        }
    }
}