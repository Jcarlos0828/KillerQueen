use nom::{
  IResult,
  sequence::tuple,
  bytes::complete::tag
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::declaraciones::declaraciones::*;
use crate::parser::bloque::*;
use crate::semantica::globales::*;

pub fn programa(input: &str) -> IResult<&str, &str> {
  let mut next: &str;
  
  next = match tuple((ws, tag("programa"), necessary_ws))(input) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err),
  };

  let id_programa: &str;

  match id(next) {
    Ok((next_input, id)) => {
      next = next_input;
      id_programa = id;
    },
    Err(err) => return Err(err),
  };

  let mut funcs1 = FUNCIONES.lock().unwrap();

  match funcs1.agregar_funcion(id_programa.to_owned(), "void".to_owned()) {
    // Ok(res) => Ok(res),
    Ok(_) => (),
    // Err(err) => Err(err)
    Err(_) => ()
  };
  drop(funcs1);

  let mut contexto_funcion1 = CONTEXTO_FUNCION.lock().unwrap();
  *contexto_funcion1 = id_programa.to_owned();
  drop(contexto_funcion1);
  // Mutex::unlock(contexto_funcion1);

  next = match tuple((ws, tag(";"), ws))(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err),
  };

  next = match (declaraciones)(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err),
  };

  next = match tuple((ws, tag("principal()"), ws))(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err),
  };

  let mut funcs2 = FUNCIONES.lock().unwrap();

  match funcs2.agregar_funcion("principal".to_owned(), "void".to_owned()) {
    // Ok(res) => Ok(res),
    Ok(_) => (),
    // Err(err) => Err(err)
    Err(_) => ()
  };
  drop(funcs2);

  let mut contexto_funcion2 = CONTEXTO_FUNCION.lock().unwrap();
  *contexto_funcion2 = "principal".to_owned();
  drop(contexto_funcion2);
  // Mutex::unlock(contexto_funcion2);

  next = match bloque(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err),
  };

  println!("{:?}", FUNCIONES.lock().unwrap());
  println!("{:?}", CLASES.lock().unwrap());
  println!("{:?}", VARIABLES.lock().unwrap());

  match ws(next) {
    Ok((_, _)) => Ok(("", "programa")),
    Err(err) => Err(err),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  // use crate::semantica::tabla_variables::*;
  // use nom::{
  //     error::{ErrorKind, VerboseError, VerboseErrorKind},
  //     Err,
  // };

  #[test]
  fn test_programa() {
    assert_eq!(programa("
      programa idPrograma;
      principal() {}"
    ), Ok(("", "programa")));

    assert_eq!(programa("
      programa idPrograma;
      principal() {
        %% comentario %%
      }"
    ), Ok(("", "programa")));

    assert_eq!(programa("
      programa idPrograma;
      entero num;
      principal() {}"
    ), Ok(("", "programa")));

    assert_eq!(programa("
      programa idPrograma;
      clase Estudiante <Persona> {
        char nombre[10], apellido[10];
      };
      principal() {}"
    ), Ok(("", "programa")));

    assert_eq!(programa("
      programa idPrograma;
      void funcion func (entero var) {
        id = 10;
        regresa expresion;
      }
      principal() {}"
    ), Ok(("", "programa")));
    
    assert_eq!(programa("
      programa idPrograma;
      void funcion func (entero var) {
        id = 10;
        regresa expresion;
      }
      entero num;
      clase Estudiante <Persona> {
        char nombre[10], apellido[10];
      };
      principal() {}"
    ), Ok(("", "programa")));

    assert_eq!(programa("
      programa idPrograma;
      void funcion func (entero var) {
        id = 10;
        regresa expresion;
      }
      entero num;
      clase Estudiante <Persona> {
        char nombre[10], apellido[10];
      };
      principal() {
        x = 10;
        d = 10 + 10;
        lee(var);
        escribe(var);
        id();
        id(param);
        id.metodo();
        mientras ( id > 10 ) {
          escribe(id);
        }

        desde arr[10] = 10 hasta 20 {
          escribe(id);
        }
        %% comentario %%
        si (id > 2) {
          escribe(id);
        }
        si (id > 2) {
          escribe(id);
        } sino {
          escribe(id);
        }
      }"
    ), Ok(("", "programa")));
  }
}
