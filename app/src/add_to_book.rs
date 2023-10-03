use patternfly_yew::prelude::{
    ActionGroup, Bullseye, Button, ButtonVariant, Form, FormGroup, Modal, ModalVariant,
    SimpleSelect, TextInput,
};
use yew::{function_component, html, use_callback, use_state_eq, Callback, Html, Properties};

#[derive(Debug, Properties, PartialEq)]
pub struct AddToBookProps {
    pub onsubmit: Callback<String>,
    pub entries: Vec<String>,
}

#[function_component(AddToBook)]
pub fn add_to_book(props: &AddToBookProps) -> Html {
    let selected_tag = use_state_eq(Option::<String>::default);
    let new_tag = use_state_eq(Option::<String>::default);

    let onselect = use_callback(
        |tag, selected_tag| selected_tag.set(Some(tag)),
        selected_tag.clone(),
    );
    let onchange = use_callback(|tag, new_tag| new_tag.set(Some(tag)), new_tag.clone());

    let onclick = use_callback(
        |_, (onsubmit, new_tag, selected_tag)| {
            let tag: &String = match (new_tag.as_ref(), selected_tag.as_ref()) {
                // No tag selected ignore the click
                (None, None) => return,
                (None, Some(tag)) => tag,
                (Some(tag), None) => tag,
                (Some(tag), Some(_)) => tag,
            };
            onsubmit.emit(tag.to_owned());
        },
        (
            props.onsubmit.clone(),
            new_tag.clone(),
            selected_tag.clone(),
        ),
    );
    let empty = &String::new();
    let selected_tag = selected_tag.as_ref().unwrap_or(empty);
    let new_tag = new_tag.as_ref().unwrap_or(empty);
    html! {
        <Bullseye>
            <div class="recipe_modal">
            <Modal
                title = {"Add this recpie to your book "}
                variant = { ModalVariant::Medium }
            >
                <Form>
                    <FormGroup label="Select an existing tag" >
                        <SimpleSelect<String> selected={selected_tag.clone()} entries={props.entries.clone()} {onselect}/>
                    </FormGroup>
                    <FormGroup label="Or create a new tag" >
                        <TextInput onchange={onchange.clone()} placeholder="breakfast" value={new_tag.clone()}/>
                    </FormGroup>
                    <ActionGroup>
                        <Button label="Add" variant={ButtonVariant::Primary} {onclick}/>
                    </ActionGroup>
                </Form>
            </Modal>
            </div>
        </Bullseye>
    }
}
