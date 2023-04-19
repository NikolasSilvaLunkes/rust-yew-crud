use yew::{html, Component, ComponentLink, Html, ShouldRender, Callback, Properties};

use crate::item::Item;
use crate::item::ItemFormData;
use crate::item::ItemValidationErr;
use crate::input::TextInput;

use yew::services::{
  ConsoleService
};

#[derive(Properties, Clone)]
pub struct ModalProperties {
  pub item: Item,
  pub visible: bool,
  pub on_close: Callback<bool>,
  pub on_save: Callback<Item>
}

pub struct Modal {
  pub item: Item,
  pub nome: String,
  pub descricao: String,
  pub relatorio: String,
  pub visible: bool,
  pub on_close: Callback<bool>,
  pub on_save: Callback<Item>,
  error: Option<Vec<ItemValidationErr>>,
  link: ComponentLink<Self>
}

pub enum ModalMsg {
  HideModal,
  SetNome(String),
  SetDescricao(String),
  SetRelatorio(String),
  Save
}

impl Component for Modal {
  type Message = ModalMsg;
  type Properties = ModalProperties;

  fn create(prop: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      item: prop.item,
      nome: "".to_string(),
      descricao: "".to_string(),
      relatorio: "".to_string(),
      visible: prop.visible,
      on_close: prop.on_close,
      on_save: prop.on_save,
      error: None,
      link
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      ModalMsg::HideModal => {
        self.visible = false;
        self.on_close.emit(true);

        true
      }

      ModalMsg::SetNome(nome) => {
        self.nome = nome;

        true
      }

      ModalMsg::SetDescricao(descricao) => {
        self.descricao = descricao;

        true
      }

      ModalMsg::SetRelatorio(relatorio) => {
        self.relatorio = relatorio;

        true
      }

      ModalMsg::Save => {
        let form_data: ItemFormData = (self.nome.clone(), self.descricao.clone(), self.relatorio.clone()).into();
        let valid = ItemFormData::validate(&form_data);

        match valid {
          Ok(_v) => {
            self.visible = false;
            self.on_save.emit(Item {
              id: self.item.id,
              nome: form_data.nome,
              descricao: form_data.descricao,
              relatorio: form_data.relatorio,
              ..Default::default()
            });

            //self.error = None;
            ConsoleService::info("Saved");
          },
          Err(e) => {
            self.error = Some(e)
          }
        }

        true
      }
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.nome = props.item.nome.clone();
    self.descricao = props.item.descricao.clone();
    self.relatorio = props.item.relatorio.clone();
    self.item = props.item;
    self.visible = props.visible;
    self.error = None;

    true
  }

  fn view(&self) -> Html {
    let visible = if self.visible { "is-active" } else { "" };

    let error = |e: &ItemValidationErr| {
      match e {
        ItemValidationErr::InvalidNome => html! {
          <div>
            {"Nome é requirido"}
          </div>
        },
        ItemValidationErr::InvalidDescricao => html! {
          <div>
            {"Descrição invalida"}
          </div>
        },
        ItemValidationErr::InvalidRelatorio => html! {
          <div>
            {"Relatório invalido"}
          </div>
        }
      }
    };

    let errors = match self.error.as_ref() {
      None => {
        html! {}
      }

      Some(errors) => {
        html! {
          <div class="notification is-danger">
            {for errors.iter().map(error)}
          </div>
        }
      }
    };

    let title = if self.item.nome.is_empty() {
      "Nova Tarefa"
    } else {
      "Atualizar Tarefa"
    };

    html! {
      <div class=("modal", visible)>
        <div class="modal-background"></div>
        <div class="modal-card">
          <form onsubmit=self.link.callback(|e: yew::events::FocusEvent| {
            e.prevent_default();

            ModalMsg::Save
          })>
            <header class="modal-card-head">
              <p class="modal-card-title">{title}</p>
              <a onclick=self.link.callback(|_| ModalMsg::HideModal) class="delete" aria-label="close"></a>
            </header>
            <section class="modal-card-body">
              {errors}
              <div class="field">
                <label class="label">{"Nome"}</label>
                <div class="control">
                <TextInput value=&self.nome oninput=self.link.callback(|val: String| ModalMsg::SetNome(val))/>
                </div>
              </div>

              <div class="field">
                <label class="label">{"Descricao"}</label>
                <p class="control has-icons-left has-icons-right">
                  <TextInput value=&self.descricao oninput=self.link.callback(|val: String| ModalMsg::SetDescricao(val))/>
                  
                </p>
              </div>

              <div class="field">
                <label class="label">{"Relatorio"}</label>
                <p class="control has-icons-left has-icons-right">
                  <TextInput value=&self.relatorio oninput=self.link.callback(|val: String| ModalMsg::SetRelatorio(val))/>
                  
                </p>
              </div>
            </section>
            <footer class="modal-card-foot">
              <button type="submit" class="button is-info">{"Save"}</button>
              <a onclick=self.link.callback(|_| ModalMsg::HideModal) class="button">{"Cancel"}</a>
            </footer>
          </form>
        </div>
      </div>
    }
  }
}