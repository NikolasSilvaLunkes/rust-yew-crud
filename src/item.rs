use std::sync::atomic::{AtomicUsize, Ordering};
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
  pub id: usize,
  pub nome: String,
  pub descricao: String,
  pub relatorio: String
}

#[derive(Default, PartialEq)]
pub struct ItemFormData {
  pub nome: String,
  pub descricao: String,
  pub relatorio: String
}

#[derive(Debug, PartialEq)]
pub struct ValidatedItem {
  nome: String,
  descricao: String,
  relatorio: String
}

#[derive(Debug, PartialEq)]
pub enum ItemValidationErr {
  InvalidNome,
  InvalidDescricao,
  InvalidRelatorio
}

impl ItemFormData {
  pub fn validate(form_data: &ItemFormData) -> Result<ValidatedItem, Vec<ItemValidationErr>> {
    let mut errors = vec![];

    let nome = ItemFormData::validate_nome(String::from(&form_data.nome))
      .unwrap_or_else(|e| {
        errors.push(e);
        String::from("")
      });

    let descricao = ItemFormData::validate_descricao(String::from(&form_data.descricao))
      .unwrap_or_else(|e| {
        errors.push(e);
        String::from("")
      });

    let relatorio = ItemFormData::validate_relatorio(String::from(&form_data.relatorio))
      .unwrap_or_else(|e| {
        errors.push(e);
        String::from("")
      });

    if !errors.is_empty() {
      return Err(errors);
    }

    Ok( ValidatedItem { nome, descricao, relatorio } )
  }

  fn validate_nome(nome: String) -> Result<String, ItemValidationErr> {
    if nome.len() > 1 {
      Ok(nome)
    } else {
      Err(ItemValidationErr::InvalidNome)
    }
  }

  fn validate_descricao(descricao: String) -> Result<String, ItemValidationErr> {
    if descricao.len() > 1 {
      Ok(descricao)
    } else {
      Err(ItemValidationErr::InvalidDescricao)
    }
  }

  fn validate_relatorio(relatorio: String) -> Result<String, ItemValidationErr> {
    if relatorio.len() < 244 {
      Ok(relatorio)
    } else {
      Err(ItemValidationErr::InvalidRelatorio)
    }
  }
}

impl From<(String, String, String)> for ItemFormData {
  fn from(fd: (String, String, String)) -> Self {
    let nome = fd.0;
    let descricao = fd.1;
    let relatorio = fd.2;

    Self {
      nome,
      descricao,
      relatorio,
      ..Default::default()
    }
  }
}

impl Item {
  pub fn generate_id() -> usize {
    static COUNTER:AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
  }
}
